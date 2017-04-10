use super::super::stm;
use visuals::visualizer as vz;
use collections::boxed::Box;
use visuals::constants::*;
use visuals::draw;
use visuals::draw::xy;


pub struct DefaultVisualizer {
    color1: u16,
    color2: u16,
}

impl vz::Visualizer for DefaultVisualizer {
    fn draw(& mut self, mut stm: &mut stm, spectrum: [f32; 16]) {
        //draw something
        draw::draw_spiral(xy {
                              x_min: X_MIN,
                              x_max: X_MAX,
                              y_min: Y_MIN,
                              y_max: Y_MAX,
                          },
                          self.color1,
                          self.color2,
                          &mut stm);
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

