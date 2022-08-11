#![windows_subsystem = "windows"] // Remove this to use console for debugging

use fractal_explorer::window::run;

fn main() {
    pollster::block_on(run());
}
