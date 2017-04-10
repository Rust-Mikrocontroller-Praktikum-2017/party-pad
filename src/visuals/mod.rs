pub mod constants;
pub mod default_visualizer;
pub mod direct_mic_visualizer;
pub mod energy_visualizer;
pub mod draw;

use super::stm;

pub trait Visualizer {
    fn draw(&mut self, stm: &mut stm, spectrum: [f32; 16]);
}
