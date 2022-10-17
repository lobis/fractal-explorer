use winit::{
    dpi::{LogicalPosition, PhysicalPosition},
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Icon, WindowBuilder},
};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use crate::state::State;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub async fn run() {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Warn).expect("Could't initialize logger");
        } else {
            env_logger::init();
        }
    }

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    let (icon_rgba, icon_width, icon_height) = {
        let icon_bytes = include_bytes!("../public/assets/icon.png");
        let icon_image = image::load_from_memory(icon_bytes).unwrap();
        let icon_rgba = icon_image.to_rgba8();

        use image::GenericImageView;
        let (width, height) = icon_image.dimensions();
        let rgba = icon_rgba.into_raw();

        (rgba, width, height)
    };

    let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();
    window.set_window_icon(Some(icon));
    window.set_title("Fractal Explorer");

    #[cfg(target_arch = "wasm32")]
    {
        use winit::dpi::PhysicalSize;
        use winit::platform::web::WindowExtWebSys;
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| {
                let anchor = doc
                    .get_element_by_id("wasm-anchor")
                    .expect("element 'wasm-anchor' missing in document");

                let (width, height) = (anchor.client_width(), anchor.client_height());

                window.set_inner_size(PhysicalSize::new(width, height));

                let canvas = window.canvas();
                canvas
                    .style()
                    .set_css_text(&format!("width: {}px; height: {}px", width, height));
                anchor.append_child(&web_sys::Element::from(canvas)).ok()?;
                Some(())
            })
            .expect("Couldn't append canvas to document body");
    }

    // State::new uses async code, so we're going to wait for it to finish
    let mut state = State::new(&window).await;
    state.reset_zoom(); // resize at start

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                if !state.input(event) {
                    match event {
                        WindowEvent::CloseRequested
                        | WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    state: ElementState::Pressed,
                                    virtual_keycode: Some(VirtualKeyCode::Escape),
                                    ..
                                },
                            ..
                        } => *control_flow = ControlFlow::Exit,
                        WindowEvent::Resized(physical_size) => {
                            state.resize(*physical_size);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            // new_inner_size is &mut so we have to dereference it twice
                            state.resize(**new_inner_size);
                        }
                        WindowEvent::CursorMoved { position, .. } => {
                            let position: LogicalPosition<f64> =
                                PhysicalPosition::to_logical(&position, window.scale_factor());
                            let size = window.inner_size();

                            let normalized_x = position.x as f32 / size.width as f32;
                            let normalized_y = position.y as f32 / size.height as f32;

                            state.uniform.mouse = [normalized_x, normalized_y]; // from 0.0 to 1.0
                        }
                        _ => {}
                    }
                }
            }
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                state.update();
                match state.render() {
                    Ok(_) => {}
                    // Reconfigure the surface if it's lost or outdated
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                        state.resize(state.size)
                    }
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    // We're ignoring timeouts
                    Err(wgpu::SurfaceError::Timeout) => log::warn!("Surface timeout"),
                }
            }
            Event::MainEventsCleared => {
                // RedrawRequested will only trigger once, unless we manually request it.
                window.request_redraw();
            }
            _ => {}
        }
    });
}
