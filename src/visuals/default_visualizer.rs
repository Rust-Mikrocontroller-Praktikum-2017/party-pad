use visuals::visualizer as vz;
use collections::boxed::Box;
use super::super::stm;
use visuals::draw;
use visuals::constants::Color;
use visuals::draw::xy;


pub struct DefaultVisualizer {
    color1: u16,
    color2: u16,
}

impl vz::Visualizer for DefaultVisualizer {
    fn draw(& mut self, mut stm: &mut stm, spectrum: [f32; 16]) {
        //draw something
        draw::draw_spiral(xy {
                              x_min: 0,
                              x_max: 480,
                              y_min: 0,
                              y_max: 272,
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

