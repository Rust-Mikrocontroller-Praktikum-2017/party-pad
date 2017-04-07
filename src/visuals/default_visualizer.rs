use visuals::visualizer as vz;
use visuals;
use collections::boxed::Box;
use super::stm;
use visuals::draw;
use visuals::draw::xy;


pub struct DefaultVisualizer;
impl vz::Visualizer for DefaultVisualizer {
    fn draw(&self, mut stm: &mut stm, spectrum: [f32; 16]) {
        //draw something
        draw::draw_spiral(xy {
                              x_min: 0,
                              x_max: 480,
                              y_min: 0,
                              y_max: 272,
                          },
                          0xFFFF,
                          0xFC00,
                          &mut stm);
    }
}
impl DefaultVisualizer {
    pub fn new() -> Box<DefaultVisualizer> {
        Box::new(DefaultVisualizer {})
    }
}

