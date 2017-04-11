use super::{STM, VizParameter, Visualizer};
use super::constants::*;
use collections::boxed::Box;


pub struct DefaultVisualizer {
    color1: u16,
    color2: u16,
}

impl Visualizer for DefaultVisualizer {
    fn draw(&mut self, mut stm: &mut STM, param: &mut VizParameter) {
        //draw something
        stm.draw_spiral(X_MIN, X_MAX, Y_MIN, Y_MAX, self.color1, self.color2);
    }
}

impl DefaultVisualizer {
    pub fn new(color1: u16, color2: u16) -> Box<DefaultVisualizer> {
        Box::new(DefaultVisualizer {
                     color1: color1,
                     color2: color2,
                 })
    }
}
