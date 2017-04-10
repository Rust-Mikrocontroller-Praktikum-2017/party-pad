use visuals::Visualizer;
use collections::boxed::Box;
use super::super::stm;
use visuals::constants::*;
use visuals::draw::{self, xy};
use stm32f7::lcd;

pub struct DirectMicVisualizer<'a> {
    current_pos: &'a mut u16,
    bar_width: u16,
}

impl<'a> Visualizer for DirectMicVisualizer<'a> {
    fn draw(&mut self, mut stm: &mut stm, spectrum: [f32; 16]) {
        //draw something
        let xy = xy {
            x_min: X_MIN,
            x_max: X_MAX,
            y_min: Y_MIN,
            y_max: Y_MAX,
        };
        let data0 = spectrum[0] as i16;
        if *self.current_pos + 2 * self.bar_width >= xy.x_max {
            *self.current_pos = 0;
            stm.lcd.clear_screen();
        }
        stm.print_bar_signed(data0, *self.current_pos, self.bar_width, xy.y_max, RED);
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
