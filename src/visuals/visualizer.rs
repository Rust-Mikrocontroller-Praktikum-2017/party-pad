use collections::boxed::Box;

pub trait Visualizer {
    fn draw(&self, spectrum: [f32; 16]);
}
