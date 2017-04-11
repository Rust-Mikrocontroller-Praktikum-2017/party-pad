use visuals::Visualizer;
use collections::boxed::Box;
use super::STM;
use visuals::constants::*;
use audio;


pub struct DirectMicBatchVisualizer<'a> {
    current_pos: &'a mut u16,
    bar_width: u16,
}
impl<'a> Visualizer for DirectMicBatchVisualizer<'a> {
    fn draw(&mut self, mut stm: &mut STM) {
        let mode = false;
        let mut mic_input:[i16;200] = [0;200];
        audio::get_microphone_input(&mut stm, &mut mic_input, mode);

        for i in 0..mic_input.len() {
            let data0 = mic_input[i] as i16;
            if *self.current_pos + 2 * self.bar_width >= X_MAX {
                *self.current_pos = 0;
                stm.lcd.clear_screen();
            }
            stm.print_bar_signed( data0,
                            *self.current_pos,
                            self.bar_width,
                            Y_MAX,
                            RED);
            *self.current_pos += self.bar_width;
        }
    }
}
impl<'a> DirectMicBatchVisualizer<'a> {
    pub fn new(current_pos: &'a mut u16, bar_width: u16) -> Box<DirectMicBatchVisualizer<'a>> {
        Box::new(DirectMicBatchVisualizer {
                     current_pos: current_pos,
                     bar_width: bar_width,
                 })
    }
}
