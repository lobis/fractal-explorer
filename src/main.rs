use fractal_generator::gpu::run;

fn main() {
    pollster::block_on(run());
}
