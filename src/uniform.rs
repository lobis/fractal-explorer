#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Zeroable, bytemuck::Pod)]
pub struct Uniform {
    pub mouse: [f32; 2],
    pub time: f32,
    _padding0: u32, // necessary to avoid error
    pub domain: [[f32; 2]; 2],
}

impl Uniform {
    pub fn default() -> Self {
        let mouse = [0.0, 0.0];
        let time = 0.0;
        let domain = [[-1.55, 1.55], [-1.55, 1.55]];
        Self {
            mouse,
            time,
            domain,
            _padding0: 0,
        }
    }
}
