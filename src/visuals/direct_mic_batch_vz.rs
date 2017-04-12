use visuals::Visualizer;
use collections::boxed::Box;
use super::STM;
use visuals::constants::*;
use audio;
use core;

const SIZE: usize = X_MAX as usize / 2;

pub struct DirectMicBatchVisualizer {
    history: [i16; SIZE],
    current_pos: u16,
    bar_width: u16,
}
impl Visualizer for DirectMicBatchVisualizer {
    fn draw(&mut self, mut stm: &mut STM) {
        let mode = false;
        let mut mic_input: [i16; SIZE] = [0; SIZE];
        audio::get_microphone_input(&mut stm, &mut mic_input, mode);

        for i in 0..mic_input.len() {
            //new code
            let scale_factor = mic_input[i] as f32 * 10.0 / core::i16::MAX as f32;
            let value = core::cmp::max(core::cmp::min((Y_MAX as f32 * scale_factor) as i16,
                                                      Y_MAX as i16 / 2),
                                       -((Y_MAX / 2) as i16));

            if value > self.history[i] {
                //new value is large than old now
                if value >= 0 && self.history[i] >= 0 {
                    //both positive => extend existing color
                    stm.draw_rectangle_filled((i - 1) as u16 * self.bar_width,
                                              i as u16 * self.bar_width,
                                              ((Y_MAX / 2) as i16 - value) as u16,
                                              ((Y_MAX / 2) as i16 - self.history[i]) as u16,
                                              RED);
                } else if value >= 0 && self.history[i] < 0 {
                    //new positive, old negative => draw new colored, draw old black
                    stm.draw_rectangle_filled((i - 1) as u16 * self.bar_width,
                                              i as u16 * self.bar_width,
                                              ((Y_MAX / 2) as i16 - value) as u16,
                                              Y_MAX / 2,
                                              RED);
                    stm.draw_rectangle_filled((i - 1) as u16 * self.bar_width,
                                              i as u16 * self.bar_width,
                                              Y_MAX / 2,
                                              ((Y_MAX / 2) as i16 - self.history[i]) as u16,
                                              BLACK);
                } else if value < 0 && self.history[i] < 0 {
                    //both negative => draw difference black
                    stm.draw_rectangle_filled((i - 1) as u16 * self.bar_width,
                                              i as u16 * self.bar_width,
                                              ((Y_MAX / 2) as i16 - value) as u16,
                                              ((Y_MAX / 2) as i16 - self.history[i]) as u16,
                                              BLACK);

                } else {
                    //stm.lcd.set_background_color(lcd::Color::rgb(0, 0, 255));
                }
            } else {
                //new value is smaller than old one
                if value >= 0 && self.history[i] >= 0 {
                    //both positive => draw difference black
                    stm.draw_rectangle_filled((i - 1) as u16 * self.bar_width,
                                              i as u16 * self.bar_width,
                                              ((Y_MAX / 2) as i16 - self.history[i]) as u16,
                                              ((Y_MAX / 2) as i16 - value) as u16,
                                              BLACK);
                } else if value < 0 && self.history[i] >= 0 {
                    //new negative, old positive => extend existing color
                    stm.draw_rectangle_filled((i - 1) as u16 * self.bar_width,
                                              i as u16 * self.bar_width,
                                              Y_MAX / 2,
                                              ((Y_MAX / 2) as i16 - value) as u16,
                                              RED);
                    stm.draw_rectangle_filled((i - 1) as u16 * self.bar_width,
                                              i as u16 * self.bar_width,
                                              ((Y_MAX / 2) as i16 - self.history[i]) as u16,
                                              Y_MAX / 2,
                                              BLACK);
                } else if value < 0 && self.history[i] < 0 {
                    //both negative => draw difference colored
                    stm.draw_rectangle_filled((i - 1) as u16 * self.bar_width,
                                              i as u16 * self.bar_width,
                                              ((Y_MAX / 2) as i16 - self.history[i]) as u16,
                                              ((Y_MAX / 2) as i16 - value) as u16,
                                              RED);
                } else {
                    //stm.lcd.set_background_color(lcd::Color::rgb(0, 255, 0));
                }
            }
            //save history
            self.history[i] = value;


            // old code
            /*
            if self.current_pos + self.bar_width > X_MAX {
                self.current_pos = 0;
                stm.lcd.clear_screen();
            }

            stm.print_bar_signed(mic_input[i], self.current_pos, self.bar_width, RED);
            self.current_pos += self.bar_width;
            */









        }
    }
}
impl DirectMicBatchVisualizer {
    pub fn new(bar_width: u16) -> Box<DirectMicBatchVisualizer> {
        Box::new(DirectMicBatchVisualizer {
                     history: [0; SIZE],
                     current_pos: 0,
                     bar_width: bar_width,
                 })
    }
}
