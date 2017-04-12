use visuals::Visualizer;
use collections::boxed::Box;
use super::STM;
use visuals::constants::*;
use transformation::DFT;
use transformation::hamming;
use audio;
use core;
use collections::Vec;

const LENGTH: usize = 512;
const SCALE_FACTOR: f32 = 13.0;
//shown maximum decreases by difference between max and current value divided by this factor
const MAX_DECREASE_FACTOR: u16 = 8;

pub struct SpectrumVisualizer2 {
    max: [u16; X_MAX as usize],
    history: [u16; X_MAX as usize],
    bar_width: u16,
    color_bar: u16,
    color_max: u16,
    color_back: u16,
    dft: DFT,
    signal: Vec<f32>,
    spectrum: Vec<f32>,
}
impl<'a> Visualizer for SpectrumVisualizer2 {
    fn draw(&mut self, mut stm: &mut STM) {
        let mode = false;
        //TODO cannot set length using bar_width
        //TODO use spectrum, not sound wave
        let mut mic_input: [i16; LENGTH] = [0; LENGTH];
        audio::get_microphone_input(&mut stm, &mut mic_input, mode);

        //TODO put the following somewhere in mic input to save iterating?
        //self.signal = mic_input.iter().enumerate().map(|(i, &v)| hamming::HAMMING_512[i] * v as f32 / core::i16::MAX as f32).collect();        
        for i in 0..LENGTH {
            self.signal[i] = hamming::HAMMING_512[i] * mic_input[i] as f32 / core::i16::MAX as f32;
        }
        self.dft.process(&self.signal, &mut self.spectrum);
        
        for (x, &value) in self.spectrum.iter().take(120).enumerate() {
            let x = (2 * x) as u16;
            //TODO scaling?
            let value = (SCALE_FACTOR * value) as u16;
            let value = core::cmp::min(value, Y_MAX);

            //draw difference between history and new value
            if value > self.history[x as usize] {
                //extend bar
                stm.draw_rectangle_filled(x as u16 * self.bar_width,
                                      (x + 1) as u16 * self.bar_width,
                                      Y_MAX - value,
                                      Y_MAX - self.history[x as usize],
                                      self.color_bar);
            } else {
                //shorten bar
                stm.draw_rectangle_filled(x as u16 * self.bar_width,
                                      (x + 1) as u16 * self.bar_width,
                                      Y_MAX - self.history[x as usize],
                                      Y_MAX - value as u16,
                                      self.color_back);
            }
            //save history
            self.history[x as usize] = value;
            //handle max: new max or decrease
            if value >= self.max[x as usize] {
                self.max[x as usize] = value;
            } else {
                //remove old max
                stm.draw_rectangle_filled(x as u16 * self.bar_width,
                                        (x + 1) as u16 * self.bar_width,
                                        Y_MAX - self.max[x as usize],
                                        core::cmp::min(Y_MAX - self.max[x as usize] + 2, Y_MAX),
                                        self.color_back);
                //compute new: decresing by 1 is slow, therefore depends on difference between max and value
                //self.max[x as usize] = if self.max[x as usize] > 0 { self.max[x as usize] - 1 } else { 0 };
                self.max[x as usize] = self.max[x as usize] - (self.max[x as usize] - value) / MAX_DECREASE_FACTOR;
                //print new max
                stm.draw_rectangle_filled(x as u16 * self.bar_width,
                                        (x + 1) as u16 * self.bar_width,
                                        Y_MAX - self.max[x as usize],
                                        core::cmp::min(Y_MAX - self.max[x as usize] + 2, Y_MAX),
                                        self.color_max);
            }
        }
    }
}
impl SpectrumVisualizer2 {
    pub fn new(bar_width: u16,
               color_bar: u16,
               color_max: u16,
               color_back: u16,)
               -> Box<SpectrumVisualizer2> {
        Box::new(SpectrumVisualizer2 {
                     bar_width: bar_width,
                     color_bar: color_bar,
                     color_max: color_max,
                     color_back: color_back,
                     history: [0;X_MAX as usize],
                     max: [0; X_MAX as usize],
                     dft: DFT::new(LENGTH),
                     signal: vec![0.0;LENGTH],
                     spectrum: vec![0.0;LENGTH],
                 })
    }
}

