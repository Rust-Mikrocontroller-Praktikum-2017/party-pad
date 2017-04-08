use collections::boxed::Box;
use super::super::stm;

pub trait Visualizer {
    fn draw(& mut self, stm:&mut stm, spectrum: [f32; 16]);
}
