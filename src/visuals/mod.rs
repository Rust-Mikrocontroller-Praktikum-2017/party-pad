pub mod constants;
pub mod default_visualizer;
pub mod direct_mic_visualizer;
pub mod direct_mic_batch_vz;
pub mod energy_visualizer;
pub mod sliding_sound_wave_vz;
pub mod sliding_sound_wave_points_vz;
pub mod draw;

use hardware::STM;


pub struct VizParameter {
    pub spectrum: [f32; 16],
    pub mic_input: [i16; 32],
}

pub trait Visualizer {
    fn draw(&mut self, stm: &mut STM, param: &mut VizParameter);
}
