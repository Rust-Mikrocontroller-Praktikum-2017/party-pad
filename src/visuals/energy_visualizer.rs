use visuals::visualizer as vz;
use collections::boxed::Box;
use super::super::stm;
use visuals::draw;
use visuals::draw::xy;
use stm32f7::lcd;
use core;
use visuals::constants as cons;


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
            data0 += (spectrum[i] as u32) * (spectrum[i] as u32);
        }

        let zero_size = 0;
        let vary_size = 60;
        let color_red: u16 = 0x7C00 | 0x8000; //red
        stm.lcd.clear_screen();
        print_circle_vary_size(&mut stm,
                               data0,
                               xy.x_max / 2,
                               xy.y_max / 2,
                               zero_size,
                               vary_size,
                               color_red);
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
                          color: u16) {
    let x_max = 480;
    let y_max = 272;

    //let scale_factor = value as f32 / core::i16::MAX as f32;
    //let value = (vary_size as f32 * scale_factor / 2.0) as u16;
    //let value = ((vary_size as u32 * value as u32) / ((core::u16::MAX as u32 * 2))) as u16;
    let scale_factor = value as f32 / (core::u32::MAX as f32*3.0);
    let value = core::cmp::min((y_max as f32 * scale_factor) as u16, 130);


    draw::draw_fill_circle(&mut stm,
                           x_pos,
                           y_pos,
                           zero_size + value as u16,
                           color);
}

