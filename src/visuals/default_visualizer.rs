use visuals::visualizer as vz;
use collections::boxed::Box;

pub struct DefaultVisualizer;
impl vz::Visualizer for DefaultVisualizer {
    fn draw(&self, spectrum: [f32; 16]) {
        //draw something
    }
}
impl DefaultVisualizer {
    pub fn new() -> Box<DefaultVisualizer> {
        Box::new(DefaultVisualizer {})
    }
}
