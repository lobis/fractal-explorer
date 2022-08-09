#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 3],
}

impl Vertex {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }
    }
}

const SIZE: f32 = 1.0;
pub const VERTICES: &[Vertex] = &[
    Vertex {
        position: [-SIZE, -SIZE, 0.0],
    },
    Vertex {
        position: [SIZE, SIZE, 0.0],
    },
    Vertex {
        position: [-SIZE, SIZE, 0.0],
    },
    Vertex {
        position: [SIZE, -SIZE, 0.0],
    },
];

pub const INDICES: &[u16] = &[
    0, 1, 2, //
    1, 0, 3, //
    /* padding */ 0,
];
