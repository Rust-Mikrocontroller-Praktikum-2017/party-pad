use collections::boxed::Box;
use super::stm;

pub trait Visualizer {
    fn draw(&self, stm:&mut stm, spectrum: [f32; 16]);
}
