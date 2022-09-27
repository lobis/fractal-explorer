#![windows_subsystem = "windows"] // Remove this to use console for debugging

pub mod state;
pub mod uniform;
pub mod vertex;
pub mod window;

fn main() {
    pollster::block_on(window::run());
}
