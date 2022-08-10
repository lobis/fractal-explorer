use fractal_generator::window::run;

fn main() {
    pollster::block_on(run());
}
