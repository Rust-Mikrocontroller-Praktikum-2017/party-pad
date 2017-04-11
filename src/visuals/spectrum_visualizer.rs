
use super::{Visualizer, STM};
use super::constants::*;
use ::transformation::DFT;
use ::transformation::hamming;
use core;

use collections::Vec;

const LENGTH : usize = 512;

pub struct SpectrumVisualizer {
    dft : DFT,
    signal : Vec<f32>,
    spectrum : Vec<f32>
}

impl SpectrumVisualizer {
    pub fn new() -> SpectrumVisualizer {
        SpectrumVisualizer {
            dft : DFT::new(LENGTH),
            signal : vec![0.0;LENGTH],
            spectrum : vec![0.0;LENGTH]
        }
    }
}

impl Visualizer for SpectrumVisualizer {
    fn draw(&mut self, stm: &mut STM) {
        for i in 0..LENGTH {
            while !stm.sai_2.bsr.read().freq() {} // fifo_request_flag
            let right = stm.sai_2.bdr.read().data() as i16;
            while !stm.sai_2.bsr.read().freq() {} // fifo_request_flag
            stm.sai_2.bdr.read().data();

            self.signal[i] = hamming::HAMMING_512[i] * right as f32 / core::i16::MAX as f32;
        }
        self.dft.process(&self.signal, &mut self.spectrum);
        stm.lcd.clear_screen();
        for (x, &v) in self.spectrum.iter().take(120).enumerate() {
            let x = (2 * x) as u16;
            let v = (100.0 * v) as u16;
            let v = if v < 272 { v } else { 272 };
            for y in 0..Y_MAX-v {
                stm.lcd.print_point_color_at(x, y, 0);
                stm.lcd.print_point_color_at(x+1, y, 0);
                stm.lcd.print_point_color_at(x+2, y, 0);
                stm.lcd.print_point_color_at(x+3, y, 0);
            }
            for y in Y_MAX-v..Y_MAX {
                stm.lcd.print_point_color_at(x, y, RED);
                stm.lcd.print_point_color_at(x+1, y, RED);
                stm.lcd.print_point_color_at(x+2, y, RED);
                stm.lcd.print_point_color_at(x+3, y, RED);
            }
        }
    }
}