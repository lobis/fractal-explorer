#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Zeroable, bytemuck::Pod)]
pub struct Uniform {
    pub mouse: [f32; 2],
    pub time: f32,
    _padding: u32, // necessary to avoid error
}

impl Uniform {
    pub fn new() -> Self {
        let mouse = [0.0, 0.0];
        let time = 0.0;
        Self {
            mouse,
            time,
            _padding: 0,
        }
    }
}
