use visuals::Visualizer;
use collections::boxed::Box;
use super::super::{STM,VizParameter};
use visuals::draw::{self,xy};
use stm32f7::lcd;
use visuals::constants as cons;


pub struct DirectMicVisualizer<'a> {
    current_pos: &'a mut u16,
    bar_width: u16,
}
impl<'a> Visualizer for DirectMicVisualizer<'a> {
    fn draw(&mut self, mut stm: &mut STM, param : &mut VizParameter) {
        //draw something
        let xy = xy {
            x_min: 0,
            x_max: 480,
            y_min: 0,
            y_max: 272,
        };
        for i in 0..spectrum.len() {
            let data0 = spectrum[i] as i16;
            if *self.current_pos + 2 * self.bar_width >= xy.x_max {
                *self.current_pos = 0;
                stm.lcd.clear_screen();
            }
            //TODO
            let color_red: u16 = 0x7C00 | 0x8000; //red
            stm.print_bar_signed( data0,
                            *self.current_pos,
                            self.bar_width,
                            xy.y_max,
                            color_red);
            *self.current_pos += self.bar_width;
        }
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


