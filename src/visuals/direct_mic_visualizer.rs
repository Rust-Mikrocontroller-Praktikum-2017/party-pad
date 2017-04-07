use visuals::visualizer as vz;
use visuals;
use collections::boxed::Box;
use super::stm;
use visuals::draw;
use visuals::draw::xy;
use stm32f7::lcd;
use core;


pub struct DirectMicVisualizer;
impl vz::Visualizer for DirectMicVisualizer {
    fn draw(&self, mut stm: &mut stm, spectrum: [f32; 16]) {
        //draw something
        let xy = xy {
            x_min: 0,
            x_max: 480,
            y_min: 0,
            y_max: 272,
        };
        let mut pos = spectrum[2] as u16;
        let bar_width = 2;
        let data0 = spectrum[0] as i16;
        if pos + 2 * bar_width >= xy.x_max {
            pos = 0;
            stm.lcd.clear_screen();
            stm.lcd.set_background_color(lcd::Color::rgb(0, 0, 0));
        }
        //TODO
        let color_red: u16 = 0x7C00 | 0x8000; //red
        print_bar_signed(&mut stm, data0, pos, bar_width, xy.y_max, color_red);
        pos += bar_width;
    }
}
impl DirectMicVisualizer {
    pub fn new() -> Box<DirectMicVisualizer> {
        Box::new(DirectMicVisualizer {})
    }
}
fn print_bar_signed(mut stm: &mut stm,
                    value: i16,
                    pos: u16,
                    width: u16,
                    y_max: u16,
                    color: u16) {
    /*
    let x_max = 480;
    let y_max: u16 = 272;

    assert!(pos < x_max);
    assert!(pos + width < x_max);
    */

    //value = value * y_max / core::u32::MAX;
    let scale_factor = value as f32 / core::i16::MAX as f32;
    let value = (y_max as f32 * scale_factor / 2.0) as i16;
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

