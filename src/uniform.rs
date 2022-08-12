#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Zeroable, bytemuck::Pod)]
pub struct Uniform {
    pub mouse: [f32; 2],
    pub time: f32,
    _padding0: i32, // necessary to avoid error
    pub domain: [[f32; 2]; 2],
    pub c: [f32; 2],
    pub mandelbrot: i32,
    _padding1: i32, // necessary to avoid error
}

impl Uniform {
    pub fn default() -> Self {
        let mouse = [0.0, 0.0];
        let time = 0.0;
        let domain = [[-1.55, 1.55], [-1.55, 1.55]];
        let c = [-0.75, 0.0];
        let mandelbrot = 0; // 0 is false, > 0 is true (cannot use bool)
        Self {
            mouse,
            time,
            domain,
            c,
            mandelbrot,
            _padding0: 0,
            _padding1: 0,
        }
    }

    fn zoom(&mut self, zoom_in: bool) {
        let domain_size = self.get_domain_size();
        let domain_size_min = domain_size[0].min(domain_size[1]);
        if zoom_in && domain_size_min <= 0.00001 {
            // limit zoom in due to precision
            return;
        }

        let zoom_factor: f32 = 0.025;
        let r: f32 = if zoom_in {
            1.0 - zoom_factor
        } else {
            1.0 + zoom_factor
        };
        let mouse = self.mouse; // from 0.0 to 1.0

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

    pub fn zoom_in(&mut self) {
        self.zoom(true);
    }

    pub fn zoom_out(&mut self) {
        self.zoom(false);
    }

    pub fn get_domain_size(&self) -> [f32; 2] {
        return [
            self.domain[0][1] - self.domain[0][0],
            self.domain[1][1] - self.domain[1][0],
        ];
    }

    pub fn translate(&mut self, vector: [f32; 2]) {
        let domain_size = self.get_domain_size();
        self.domain = [
            [
                self.domain[0][0] - domain_size[0] * vector[0],
                self.domain[0][1] - domain_size[0] * vector[0],
            ],
            [
                self.domain[1][0] + domain_size[1] * vector[1],
                self.domain[1][1] + domain_size[1] * vector[1],
            ],
        ];
    }
}
