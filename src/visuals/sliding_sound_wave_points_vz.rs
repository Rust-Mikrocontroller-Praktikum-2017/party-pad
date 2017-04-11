use visuals::Visualizer;
use collections::boxed::Box;
use super::{STM, VizParameter};
use stm32f7::lcd;
use visuals::constants::*;
use core;
use audio;


pub struct SlidingSoundPointsVisualizer<'a> {
    buffer: &'a mut [i16; X_MAX as usize],
    bar_width: u16,
}
impl<'a> Visualizer for SlidingSoundPointsVisualizer<'a> {
    fn draw(&mut self, mut stm: &mut STM, param: &mut VizParameter) {
        let mode = false;
        let mut mic_input:[i16;1] = [0];
        audio::get_microphone_input(&mut stm, &mut mic_input, mode);

        /*
        let scale_factor = mic_input[0]  as f32 * 4.0 / core::i16::MAX as f32;
        let new_value = core::cmp::max(core::cmp::min((Y_MAX as f32 * scale_factor) as i16, (130/self.bar_width) as i16), (-130 as i16) / (self.bar_width as i16));
        self.buffer[((X_MAX / self.bar_width)-1) as usize] = new_value;
        */
        let scale_factor = mic_input[0]  as f32 * 2.0 / core::i16::MAX as f32;
        let new_value = core::cmp::max(core::cmp::min((Y_MAX as f32 * scale_factor) as i16,
                                                  130 as i16),
                                   -130 as i16);
        self.buffer[((X_MAX / self.bar_width) - 1) as usize] = new_value;
        //for i in 1..X_MAX {
        for i in 1..((X_MAX / self.bar_width)) {
            /*
            //remove old point            
            stm.lcd.print_point_color_at(i-1,((Y_MAX/2) as i16 - self.buffer[(i-1) as usize]) as u16, BLACK);
            //add new point
            stm.lcd.print_point_color_at(i-1,((Y_MAX/2) as i16 - self.buffer[i as usize]) as u16, RED);
            self.buffer[(i-1) as usize] = self.buffer[i as usize];
            */
            //remove old point
            stm.draw_square((i - 1) as u16 * self.bar_width,
                            ((Y_MAX/2) as i16 - self.buffer[(i-1) as usize]) as u16,
                            self.bar_width,
                            BLACK);
            //add new point
            stm.draw_square((i - 1) as u16 * self.bar_width,
                            ((Y_MAX/2) as i16 - self.buffer[i as usize]) as u16,
                            self.bar_width,
                            RED);
            self.buffer[i as usize - 1] = self.buffer[i as usize];
        }
        /*
        let scale_factor = param.mic_input[0]  as f32 * 4.0 / core::i16::MAX as f32;
        let new_value = core::cmp::max(core::cmp::min((Y_MAX as f32 * scale_factor) as i16, (130/self.bar_width) as i16), (-130 as i16) / (self.bar_width as i16));
        self.buffer[((X_MAX / self.bar_width)-2) as usize] = new_value;
        */
    }
}
impl<'a> SlidingSoundPointsVisualizer<'a> {
    pub fn new(buffer: &'a mut [i16; X_MAX as usize],
               bar_width: u16)
               -> Box<SlidingSoundPointsVisualizer<'a>> {
        Box::new(SlidingSoundPointsVisualizer {
                     bar_width: bar_width,
                     buffer: buffer,
                 })
    }
}

