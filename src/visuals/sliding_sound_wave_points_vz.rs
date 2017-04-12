use visuals::Visualizer;
use collections::boxed::Box;
use super::STM;
use visuals::constants::*;
use core;
use audio;


pub struct SlidingSoundPointsVisualizer {
    buffer: [i16; X_MAX as usize],
    bar_width: u16,
    color_front: u16,
    color_back: u16,
}
impl<'a> Visualizer for SlidingSoundPointsVisualizer {
    fn draw(&mut self, mut stm: &mut STM) {
        let mode = false;
        let mut mic_input:[i16;1] = [0];
        audio::get_microphone_input(&mut stm, &mut mic_input, mode);

        /*
        let scale_factor = mic_input[0]  as f32 * 4.0 / core::i16::MAX as f32;
        let new_value = core::cmp::max(core::cmp::min((Y_MAX as f32 * scale_factor) as i16, (
            130/self.bar_width) as i16), (-130 as i16) / (self.bar_width as i16));
        self.buffer[((X_MAX / self.bar_width)-1) as usize] = new_value;
        */
        let scale_factor = mic_input[0]  as f32 * 2.0 / core::i16::MAX as f32;
        let new_value = core::cmp::max(core::cmp::min((Y_MAX as f32 * scale_factor) as i16,130 as i16),-130 as i16);
        self.buffer[((X_MAX / self.bar_width) - 1) as usize] = new_value;
        //for i in 1..X_MAX {
        for i in 1..((X_MAX / self.bar_width)) {
            /*
            //remove old point            
            stm.lcd.print_point_color_at(i-1,((Y_MAX/2) as i16 - self.buffer[(i-1) as usize]) as u16, 
            self.color_back);
            //add new point
            stm.lcd.print_point_color_at(i-1,((Y_MAX/2) as i16 - self.buffer[i as usize]) as u16,
             self.color_back);
            self.buffer[(i-1) as usize] = self.buffer[i as usize];
            */
            //remove old point
            stm.draw_square((i - 1) as u16 * self.bar_width,
                            ((Y_MAX/2) as i16 - self.buffer[(i-1) as usize]) as u16,
                            self.bar_width,
                            self.color_back);
            //add new point
            stm.draw_square((i - 1) as u16 * self.bar_width,
                            ((Y_MAX/2) as i16 - self.buffer[i as usize]) as u16,
                            self.bar_width,
                            self.color_front);
            self.buffer[i as usize - 1] = self.buffer[i as usize];
        }
        /*
        let scale_factor = param.mic_input[0]  as f32 * 4.0 / core::i16::MAX as f32;
        let new_value = core::cmp::max(core::cmp::min((Y_MAX as f32 * scale_factor) as i16,
         (130/self.bar_width) as i16), (-130 as i16) / (self.bar_width as i16));
        self.buffer[((X_MAX / self.bar_width)-2) as usize] = new_value;
        */
    }
}
impl SlidingSoundPointsVisualizer {
    pub fn new(bar_width: u16,
               color_front:u16,
               color_back:u16,)
               -> Box<SlidingSoundPointsVisualizer> {
        Box::new(SlidingSoundPointsVisualizer {
                     bar_width: bar_width,
                     buffer: [0; X_MAX as usize],
                     color_front:color_front,
                     color_back:color_back,
                 })
    }
}

