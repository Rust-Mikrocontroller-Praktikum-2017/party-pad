mod visualizer;
mod default_visualizer;
mod direct_mic_visualizer;
mod draw;

use collections::boxed::Box;
use super::stm;
extern crate stm32f7_discovery as stm32f7;
use stm32f7::{system_clock, i2c, board, touch, embedded};
use core::ptr;
use visuals::visualizer::Visualizer;

pub struct Visuals {
    current_visualizer: Box<visualizer::Visualizer>,
    stm: stm,
}

impl Visuals {
    pub fn new(stm: stm) -> Visuals {
        Visuals {
            current_visualizer: direct_mic_visualizer::DirectMicVisualizer::new(),
            stm: stm,
        }
    }
    pub fn set_visualizer(&mut self, visualizer: Box<Visualizer>) {
        self.current_visualizer = visualizer;
    }
    pub fn draw_with_current(&mut self, spectrum: [f32; 16]) {
        self.current_visualizer.draw(&mut self.stm, spectrum);
        // knows current visualisation parameters
    }

}
