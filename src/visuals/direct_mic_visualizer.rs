use visuals::Visualizer;
use collections::boxed::Box;
use super::{STM, VizParameter};
use visuals::constants::*;

pub struct DirectMicVisualizer<'a> {
    current_pos: &'a mut u16,
    bar_width: u16,
}

impl<'a> Visualizer for DirectMicVisualizer<'a> {
    fn draw(&mut self, mut stm: &mut STM, param: &mut VizParameter) {
        //draw something
        let data0 = param.spectrum[0] as i16;
        if *self.current_pos + 2 * self.bar_width >= X_MAX {
            *self.current_pos = 0;
            stm.lcd.clear_screen();
        }
        stm.print_bar_signed(data0, *self.current_pos, self.bar_width, Y_MAX, RED);
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
