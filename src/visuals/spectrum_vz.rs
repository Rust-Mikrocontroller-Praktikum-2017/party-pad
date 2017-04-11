use visuals::Visualizer;
use collections::boxed::Box;
use super::STM;
use visuals::constants::*;
use audio;
use core;


pub struct SpectrumVisualizer<'a> {
    max: &'a mut [u16; X_MAX as usize],
    history: &'a mut [u16; X_MAX as usize],
    bar_width: u16,
    color_bar: u16,
    color_max: u16,
}
impl<'a> Visualizer for SpectrumVisualizer<'a> {
    fn draw(&mut self, mut stm: &mut STM) {
        let mode = false;
        //TODO cannot set length using bar_width
        //TODO use spectrum, not sound wave
        let mut mic_input: [i16; (X_MAX / 2) as usize] = [0; (X_MAX / 2) as usize];
        audio::get_microphone_input(&mut stm, &mut mic_input, mode);

        stm.lcd.clear_screen();
        for i in 0..mic_input.len() {
            //TODO scaling?
            let scale_factor = mic_input[i] as f32 * 10.0 / core::i16::MAX as f32;
            let value = (Y_MAX as f32 * scale_factor) as i32;
            let value = if value < 0 { -value } else { value };
            let value = core::cmp::min(value as u16, Y_MAX);
            stm.draw_rectangle_filled(i as u16 * self.bar_width,
                                      (i + 1) as u16 * self.bar_width,
                                      Y_MAX - value as u16,
                                      Y_MAX,
                                      self.color_bar);
            //save history
            self.history[i] = value;
            //handle max: new max or decrease
            if value as u16 > self.max[i] {
                self.max[i] = value;
            } else {
                self.max[i] = if self.max[i] > 0 {self.max[i] - 1} else {0};
            }
            //print max
            stm.draw_rectangle_filled(i as u16 * self.bar_width,
                                      (i + 1) as u16 * self.bar_width,
                                      Y_MAX - self.max[i],
                                      core::cmp::min(Y_MAX - self.max[i] + 2, Y_MAX),
                                      self.color_max);
        }

    }
}
impl<'a> SpectrumVisualizer<'a> {
    pub fn new(history: &'a mut [u16; X_MAX as usize],
               max: &'a mut [u16; X_MAX as usize],
               bar_width: u16,
               color_bar: u16,
               color_max: u16,)
               -> Box<SpectrumVisualizer<'a>> {
        Box::new(SpectrumVisualizer {
                     history: history,
                     max: max,
                     bar_width: bar_width,
                     color_bar: color_bar,
                     color_max: color_max,
                 })
    }
}

