#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Zeroable, bytemuck::Pod)]
pub struct Uniform {
    pub mouse: [f32; 2],
    pub time: f32,
    _padding0: u32, // necessary to avoid error
    pub domain: [[f32; 2]; 2],
    pub c: [f32; 2],
}

impl Uniform {
    pub fn default() -> Self {
        let mouse = [0.0, 0.0];
        let time = 0.0;
        let domain = [[-1.55, 1.55], [-1.55, 1.55]];
        let c = [-0.75, 0.0];
        Self {
            mouse,
            time,
            domain,
            c,
            _padding0: 0,
        }
    }

    pub fn zoom(&mut self, zoom_into: bool) {
        let zoom_factor: f32 = 0.025;
        let r: f32 = if zoom_into {
            1.0 - zoom_factor
        } else {
            1.0 + zoom_factor
        };
        let mouse = self.mouse; // from 0.0 to 1.0
        let domain_size = [
            self.domain[0][1] - self.domain[0][0],
            self.domain[1][1] - self.domain[1][0],
        ];
        self.domain = [
            [
                self.domain[0][0] + (1.0 - r) * domain_size[0] * mouse[0],
                self.domain[0][1] - (1.0 - r) * domain_size[0] * (1.0 - mouse[0]),
            ],
            [
                self.domain[1][0] + (1.0 - r) * domain_size[1] * (1.0 - mouse[1]),
                self.domain[1][1] - (1.0 - r) * domain_size[1] * mouse[1],
            ],
        ];
    }
}
