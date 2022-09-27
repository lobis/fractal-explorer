use std::iter;

use instant::Instant;
use wgpu::util::DeviceExt;
use winit::{event::*, window::Window};

use crate::uniform::Uniform;
use crate::vertex::{Vertex, INDICES, VERTICES};

pub struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
    instant: Instant,
    c_from_mouse: bool,
    dragging: bool,
    dragging_position_original: [f32; 2],
    //
    pub uniform: Uniform,
    uniform_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,
}

impl State {
    pub async fn new(window: &Window) -> Self {
        let size = window.inner_size();
        let c_from_mouse = false;
        // The instance is a handle to our GPU
        // BackendBit::PRIMARY => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    // WebGL doesn't support all of wgpu's features, so if
                    // we're building for the web we'll have to disable some.
                    limits: if cfg!(target_arch = "wasm32") {
                        wgpu::Limits::downlevel_webgl2_defaults()
                    } else {
                        wgpu::Limits::default()
                    },
                },
                None, // Trace path
            )
            .await
            .unwrap();

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_supported_formats(&adapter)[0],
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &config);

        let uniform = Uniform::default();

        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::cast_slice(&[uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let uniform_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("uniform_bind_group_layout"),
            });

        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &uniform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
            label: Some("uniform_bind_group"),
        });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/shader.wgsl").into()),
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&uniform_bind_group_layout],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent::REPLACE,
                        alpha: wgpu::BlendComponent::REPLACE,
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::POLYGON_MODE_LINE
                // or Features::POLYGON_MODE_POINT
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            // If the pipeline will be used with a multiview render pass, this
            // indicates how many array layers the attachments will have.
            multiview: None,
        });

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });
        let num_indices = INDICES.len() as u32;
        let instant = Instant::now();
        let dragging = false;
        let dragging_position_original = [0.0, 0.0];

        Self {
            surface,
            device,
            queue,
            config,
            size,
            c_from_mouse,
            dragging,
            dragging_position_original,
            render_pipeline,
            vertex_buffer,
            index_buffer,
            num_indices,
            instant,
            uniform,
            uniform_buffer,
            uniform_bind_group,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);

            let domain_size = self.uniform.get_domain_size();

            let ratio_window = self.size.width as f32 / self.size.height as f32;

            let domain_size_target_y = domain_size[0] / ratio_window;
            let domain_size_delta_y = domain_size[1] - domain_size_target_y;

            self.uniform.domain[1] = [
                self.uniform.domain[1][0] + domain_size_delta_y / 2.0,
                self.uniform.domain[1][1] - domain_size_delta_y / 2.0,
            ];
        }
    }

    pub fn reset_zoom(&mut self) {
        self.uniform.domain = Uniform::default().domain;
        self.resize(self.size);

        let size_y = self.uniform.domain[1][1] - self.uniform.domain[1][0];
        let size_y_min = (Uniform::default().domain[1][1] - Uniform::default().domain[1][0]) * 0.75;
        if size_y < size_y_min {
            let ratio = size_y_min / size_y;
            self.uniform.domain[1] = [-size_y_min / 2.0, size_y_min / 2.0];
            self.uniform.domain[0] = [
                self.uniform.domain[0][0] * ratio,
                self.uniform.domain[0][1] * ratio,
            ];
        }
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Space),
                        ..
                    },
                ..
            }
            | WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Plus),
                        ..
                    },
                ..
            }
            | WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Up),
                        ..
                    },
                ..
            } => {
                self.c_from_mouse = false;
                self.uniform.zoom_in();
                true
            }
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Minus),
                        ..
                    },
                ..
            }
            | WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Down),
                        ..
                    },
                ..
            } => {
                self.c_from_mouse = false;
                self.uniform.zoom_out();
                true
            }
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::C),
                        ..
                    },
                ..
            }
            | WindowEvent::MouseInput {
                state: ElementState::Pressed,
                button: MouseButton::Left,
                ..
            } => {
                if self.uniform.mandelbrot == 1 {
                    self.c_from_mouse = false;
                    return true;
                }

                self.c_from_mouse = !self.c_from_mouse;

                // reset zoom
                if (self.uniform.domain[0][1] - self.uniform.domain[0][0])
                    < (Uniform::default().domain[0][1] - Uniform::default().domain[0][0])
                {
                    // do not reset when zoomed out
                    self.reset_zoom();
                }

                true
            }
            WindowEvent::CursorMoved { position, .. } => {
                if self.c_from_mouse && !self.dragging {
                    self.uniform.c = [
                        (position.x as f32 / self.size.width as f32 - 0.5) * 2.0,
                        (position.y as f32 / self.size.height as f32 - 0.5) * 2.0,
                    ];
                }
                true
            }
            WindowEvent::MouseWheel { delta, .. } => {
                let y: f32 = match delta {
                    MouseScrollDelta::LineDelta(_, line_delta_y) => *line_delta_y,
                    MouseScrollDelta::PixelDelta(position) => position.y as f32,
                };
                if y.abs() > 0.0 {
                    let mut zoom_many_times: u32 = 1;
                    if y.abs() > 1.0 {
                        // when users scrolls fast, zoom much faster
                        zoom_many_times = 5;
                    }
                    let zoom_in: bool = y > 0.0;
                    for _ in 0..zoom_many_times {
                        if zoom_in {
                            self.uniform.zoom_in();
                        } else {
                            self.uniform.zoom_out();
                        }
                    }
                }
                true
            }
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::R),
                        ..
                    },
                ..
            }
            | WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::F5),
                        ..
                    },
                ..
            } => {
                self.reset_zoom();
                true
            }
            WindowEvent::MouseInput {
                state,
                button: MouseButton::Middle | MouseButton::Right,
                ..
            } => {
                self.dragging_position_original = self.uniform.mouse;
                match state {
                    ElementState::Pressed => {
                        self.dragging = true;
                    }
                    ElementState::Released => {
                        self.dragging = false;
                    }
                }
                true
            }
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::M),
                        ..
                    },
                ..
            } => {
                self.uniform.mandelbrot = 1;
                self.c_from_mouse = false;
                self.uniform.c = [0.0, 0.0];
                self.reset_zoom();
                // center the mandelbrot a bit
                let shift = 0.6;
                self.uniform.domain[0][0] -= shift;
                self.uniform.domain[0][1] -= shift;
                true
            }
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::J),
                        ..
                    },
                ..
            } => {
                self.uniform.mandelbrot = 0;
                self.c_from_mouse = false;
                self.reset_zoom();
                self.uniform.c = Uniform::default().c;
                true
            }
            _ => false,
        };
        false
    }

    pub fn update(&mut self) {
        self.uniform.time = self.instant.elapsed().as_secs_f32();
        if self.dragging {
            self.uniform.translate([
                self.uniform.mouse[0] - self.dragging_position_original[0],
                self.uniform.mouse[1] - self.dragging_position_original[1],
            ]);
            self.dragging_position_original = self.uniform.mouse;
        }

        self.queue.write_buffer(
            &self.uniform_buffer,
            0,
            bytemuck::cast_slice(&[self.uniform]),
        );
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);

            render_pass.set_bind_group(0, &self.uniform_bind_group, &[]);

            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

            render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
        }

        self.queue.submit(iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
