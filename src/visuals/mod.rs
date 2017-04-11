pub mod constants;
pub mod default_visualizer;
pub mod direct_mic_visualizer;
pub mod direct_mic_batch_vz;
pub mod energy_visualizer;
pub mod sliding_sound_wave_vz;
pub mod sliding_sound_wave_points_vz;
pub mod draw;

use hardware::STM;

pub trait Visualizer {
    fn draw(&mut self, stm: &mut STM);
}
