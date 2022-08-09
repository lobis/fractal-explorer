#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

impl Vertex {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}

pub const VERTICES: &[Vertex] = &[
    Vertex {
        position: [-1.0, -1.0, 0.0],
        color: [0.5, 0.0, 0.5],
    },
    Vertex {
        position: [1.0, 1.0, 0.0],
        color: [0.5, 0.0, 0.5],
    },
    Vertex {
        position: [-1.0, 1.0, 0.0],
        color: [0.5, 0.0, 0.5],
    },
    Vertex {
        position: [1.0, -1.0, 0.0],
        color: [0.5, 0.0, 0.5],
    },
];

pub const INDICES: &[u16] = &[
    0, 1, 2, //
    // 1, 0, 3, //
    /* padding */ 0,
];
