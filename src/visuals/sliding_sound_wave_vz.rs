use visuals::Visualizer;
use collections::boxed::Box;
use super::super::{STM, VizParameter};
use stm32f7::lcd;
use visuals::constants::*;


pub struct SlidingSoundVisualizer<'a> {
    buffer: &'a mut [i16; X_MAX as usize],
    current_pos: &'a mut u16,
    bar_width: u16,
}
impl<'a> Visualizer for SlidingSoundVisualizer<'a> {
    fn draw(&mut self, mut stm: &mut STM, param: &mut VizParameter) {
        stm.lcd.clear_screen();
        for i in 1..((X_MAX / self.bar_width)) {
            /*
            if i == 1 {
                stm.lcd.set_background_color(lcd::Color::rgb(255, 0, 0));
            } else if i == 2 {
                stm.lcd.set_background_color(lcd::Color::rgb(0, 255, 0));
            } else if i == 3 {
                stm.lcd.set_background_color(lcd::Color::rgb(0, 0, 255));

            }
            */
            //TODO instead of refrest, redraw last scren bars with black
            /*
            stm.print_bar_signed(self.buffer[i as usize],
                                 (i - 1) as u16 * self.bar_width,
                                 self.bar_width,
                                 Y_MAX,
                                 RED);
            */
            stm.print_bar_signed(self.buffer[i as usize],
                                 (i - 1) as u16 * self.bar_width,
                                 self.bar_width,
                                 Y_MAX,
                                 RED);
            
            self.buffer[i as usize - 1] = self.buffer[i as usize];
        }
        self.buffer[((X_MAX / self.bar_width) - 1) as usize] = param.mic_input[0];
        stm.print_bar_signed(param.mic_input[0],
                             (((X_MAX / self.bar_width) - 1) * self.bar_width),
                             self.bar_width,
                             Y_MAX,
                             RED);
       /*
        if *self.current_pos + 2 * self.bar_width >= X_MAX {
            *self.current_pos = 0;
            stm.lcd.clear_screen();
        }
        stm.print_bar_signed(param.mic_input[0],
                             *self.current_pos,
                             self.bar_width,
                             Y_MAX,
                             RED);
        *self.current_pos += self.bar_width;
        */
    }
}
impl<'a> SlidingSoundVisualizer<'a> {
    pub fn new(buffer: &'a mut [i16; X_MAX as usize],
               current_pos: &'a mut u16,
               bar_width: u16)
               -> Box<SlidingSoundVisualizer<'a>> {
        Box::new(SlidingSoundVisualizer {
                     current_pos: current_pos,
                     bar_width: bar_width,
                     buffer: buffer,
                 })
    }
}

