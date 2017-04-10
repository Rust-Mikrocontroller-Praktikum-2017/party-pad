use collections::boxed::Box;
use super::super::stm;
use visuals::constants as cons;
use visuals::draw;
use visuals::draw::xy;
use visuals::Visualizer;
use stm32f7::lcd;
use core;


pub struct EnergyVisualizer<'a> {
    last_radius: &'a mut u16,
}

impl<'a> vz::Visualizer for EnergyVisualizer<'a> {
    fn draw(&mut self, mut stm: &mut stm, spectrum: [f32; 16]) {
        //draw something
        let xy = xy {
            x_min: 0,
            x_max: 480,
            y_min: 0,
            y_max: 272,
        };
        let mut data0:u32 = 0;
        for i in 0..spectrum.len() {
            data0 += (spectrum[i] * spectrum[i]) as u32;
        }
        let max_val = spectrum.len() as u32 * core::i16::MAX as u32 * core::i16::MAX as u32;
        let scale_factor = data0 as f32 / max_val as f32;

        let zero_size = 0;
        let vary_size = 60;
        stm.lcd.clear_screen();
        print_circle_vary_size(&mut stm,
                               data0,
                               xy.x_max / 2,
                               xy.y_max / 2,
                               zero_size,
                               vary_size,
                               scale_factor,
                               cons::RED);
    }
}
impl<'a> EnergyVisualizer<'a> {
    pub fn new(last_radius: &'a mut u16) -> Box<EnergyVisualizer<'a>> {
        Box::new(EnergyVisualizer { last_radius: last_radius })
    }
}
fn print_circle_vary_size(mut stm: &mut stm,
                          value: u32,
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
    let value = core::cmp::min((cons::Y_MAX as f32 / 2.0 * scale_factor) as u16, 130);


    draw::draw_fill_circle(&mut stm,
                           x_pos,
                           y_pos,
                           zero_size + value as u16,
                           color);
}

