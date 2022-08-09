use fractal_generator::event::run;

fn main() {
    pollster::block_on(run());
}
