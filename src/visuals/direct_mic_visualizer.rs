use visuals::visualizer as vz;
use collections::boxed::Box;
use super::super::stm;
use visuals::draw;
use visuals::draw::xy;
use stm32f7::lcd;
use core;
use visuals::constants as cons;


pub struct DirectMicVisualizer<'a> {
    current_pos: &'a mut u16,
    bar_width: u16,
}
impl<'a> vz::Visualizer for DirectMicVisualizer<'a> {
    fn draw(&mut self, mut stm: &mut stm, spectrum: [f32; 16]) {
        //draw something
        let xy = xy {
            x_min: 0,
            x_max: 480,
            y_min: 0,
            y_max: 272,
        };
        let data0 = spectrum[0] as i16;
        if *self.current_pos + 2 * self.bar_width >= xy.x_max {
            *self.current_pos = 0;
            stm.lcd.clear_screen();
        }
        //TODO
        let color_red: u16 = 0x7C00 | 0x8000; //red
        print_bar_signed(&mut stm,
                         data0,
                         *self.current_pos,
                         self.bar_width,
                         xy.y_max,
                         color_red);
        *self.current_pos += self.bar_width;
    }
}   
impl<'a> DirectMicVisualizer<'a> {
    pub fn new(current_pos: &'a mut u16, bar_width: u16) -> Box<DirectMicVisualizer<'a>> {
        Box::new(DirectMicVisualizer {
                     current_pos: current_pos,
                     bar_width: bar_width,
                 })
    }
}
fn print_bar_signed(mut stm: &mut stm, value: i16, pos: u16, width: u16, y_max: u16, color: u16) {
    /*
    let x_max = 480;
    let y_max: u16 = 272;

    assert!(pos < x_max);
    assert!(pos + width < x_max);
    */

    //TODO how to scale properly?
    let scale_factor = value as f32 * 10.0 / core::i16::MAX as f32;
    //let scale_factor = value as f32 / core::i16::MAX as f32;
    //TODO constants
    let value = core::cmp::max(core::cmp::min((y_max as f32 * scale_factor) as i16, 130 as i16), -130 as i16);
    //print_fill_rect(&mut lcd, pos, 20, pos+width, 20, 0x801F);

    if value > 0 {
        draw::print_fill_rect(&mut stm,
                              pos,
                              pos + width,
                              y_max / 2,
                              (y_max as i16 / 2 + value) as u16,
                              color);
    } else {
        draw::print_fill_rect(&mut stm,
                              pos,
                              pos + width,
                              (y_max as i16 / 2 + value) as u16,
                              y_max / 2,
                              color);

    }
}

