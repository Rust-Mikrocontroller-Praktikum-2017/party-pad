pub mod constants;
pub mod default_visualizer;
pub mod direct_mic_visualizer;
pub mod direct_mic_batch_vz;
pub mod energy_visualizer;
pub mod draw;

use super::{STM,VizParameter};

pub trait Visualizer {
    fn draw(&mut self, stm: &mut STM, param: &mut VizParameter);
}
