mod visualizer;
mod default_visualizer;

use collections::boxed::Box;
use super::stm;
extern crate stm32f7_discovery as stm32f7;
use stm32f7::{system_clock, i2c, board, touch, embedded};
use core::ptr;

pub struct Visuals {
    current_visualizer: Box<visualizer::Visualizer>,
    stm: stm,
    x_min: u16,
    x_max: u16,
    y_min: u16,
    y_max: u16,
}

impl Visuals {
    pub fn new(stm: stm) -> Visuals {
        Visuals {
            current_visualizer: default_visualizer::DefaultVisualizer::new(),
            stm: stm,
            x_min: 0,
            x_max: 480,
            y_min: 0,
            y_max: 272,
        }
    }
    pub fn draw_with_current(&mut self, spectrum: [f32; 16]) {
        self.current_visualizer.draw(spectrum);
        // knows current visualisation parameters
    }

    pub fn blink_led(&mut self, time_distance: usize) {
        let mut last_led_toggle = 0;
        //////// LED stuff
        let ticks = system_clock::ticks();
        // every 0.5 seconds
        if ticks - last_led_toggle >= time_distance {
            // toggle the led
            let led_current = self.stm.led.get();
            self.stm.led.set(!led_current);
            last_led_toggle = ticks;
        }
    }

    pub fn spiral_visuals(&mut self) {
        let mut color1 = 0xf00f | 0x8000;
        let mut color2 = 0xffff | 0x8000;
        let mut spiral_drawn = false;
        let mut swap_color = false;

        ///////// spiral
        for _ in &touch::touches(&mut self.stm.i2c_3).unwrap() {
            swap_color = true;
        }
        if swap_color {
            let temp_color = color1;
            color1 = color2;
            color2 = temp_color;
            swap_color = false;
            spiral_drawn = false;
        }
        if !spiral_drawn {
            self.draw_spiral(color1, color2);
        }
    }

    fn draw_rectangle(&mut self, color: u16) {
        for x in self.x_min..self.x_max {
            self.stm.lcd.print_point_color_at(x, self.y_min, color);
            self.stm
                .lcd
                .print_point_color_at(x, self.y_max - 1, color);
        }
        for y in self.y_min + 1..self.y_max - 1 {
            self.stm.lcd.print_point_color_at(self.x_min, y, color);
            self.stm
                .lcd
                .print_point_color_at(self.x_max - 1, y, color);
        }
    }

    fn draw_spiral(&mut self, color1: u16, color2: u16) {

        let mut start_color = color1;
        let mut color = start_color;

        while self.y_min < 135 {
            // only works because 480 is dividable by 5

            for _ in 0..5 {
                self.draw_rectangle(color);
                // update variables
                self.x_min += 1;
                self.x_max -= 1;
                self.y_min += 1;
                self.y_max -= 1;
            }
            color = if color == color1 { color2 } else { color1 }
        }
        self.draw_rectangle(color);
    }
}
