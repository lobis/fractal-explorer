#![windows_subsystem = "windows"] // Remove this to use console for debugging

use fractal_explorer::window;

fn main() {
    pollster::block_on(window::run());
}
