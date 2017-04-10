use collections::boxed::Box;
use super::super::{STM, VizParameter};
use visuals::constants::*;
use visuals::Visualizer;
use core;


pub struct EnergyVisualizer<'a> {
    last_radius: &'a mut u16,
}

impl<'a> Visualizer for EnergyVisualizer<'a> {
    fn draw(&mut self, mut stm: &mut STM, param: &mut VizParameter) {
        //draw something
        let mut data0: u32 = 0;
        for i in 0..param.spectrum.len() {
            data0 += (param.spectrum[i] * param.spectrum[i]) as u32;
        }
        //let max_val = spectrum.len() as u32 * core::i16::MAX as u32 * core::i16::MAX as u32;
        //let scale_factor = data0 as f32 / spectrum.len() as f32 / core::i16::MAX as f32 /
        //core::i16::MAX as f32;
        let scale_factor = data0 as f32 / (core::i16::MAX as f32 * core::i16::MAX as f32) / param.spectrum.len() as f32;

        let zero_size = 0;
        let vary_size = 60;
        stm.lcd.clear_screen();
        print_circle_vary_size(&mut stm,
                               X_MAX / 2,
                               Y_MAX / 2,
                               zero_size,
                               vary_size,
                               scale_factor,
                               RED);
    }
}
impl<'a> EnergyVisualizer<'a> {
    pub fn new(last_radius: &'a mut u16) -> Box<EnergyVisualizer<'a>> {
        Box::new(EnergyVisualizer { last_radius: last_radius })
    }
}
fn print_circle_vary_size(mut stm: &mut STM,
                          x_pos: u16,
                          y_pos: u16,
                          zero_size: u16,
                          vary_size: u16,
                          scale_factor: f32,
                          color: u16) {

    //let scale_factor = value as f32 / core::i16::MAX as f32;
    //let value = (vary_size as f32 * scale_factor / 2.0) as u16;
    //let value = ((vary_size as u32 * value as u32) / ((core::u16::MAX as u32 * 2))) as u16;
    //let scale_factor = value as f32 / (core::u32::MAX as f32*3.0);
    let value = core::cmp::min((Y_MAX as f32 * scale_factor) as u16, 130);


    stm.draw_circle_filled(x_pos, y_pos, zero_size + value as u16, color);
}

