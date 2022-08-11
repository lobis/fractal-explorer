#![windows_subsystem = "windows"]

use fractal_explorer::window::run;

fn main() {
    pollster::block_on(run());
}
