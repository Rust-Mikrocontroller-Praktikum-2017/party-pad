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

pub struct SpectrumVisualizer2<'a> {
    max: &'a mut [u16; X_MAX as usize],
    history: &'a mut [u16; X_MAX as usize],
    bar_width: u16,
    color_bar: u16,
    color_max: u16,
    dft: DFT,
    signal: Vec<f32>,
    spectrum: Vec<f32>,
}
impl<'a> Visualizer for SpectrumVisualizer2<'a> {
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
        
        
        stm.lcd.clear_screen();
        for (x, &value) in self.spectrum.iter().take(120).enumerate() {
            let x = (2 * x) as u16;
            //TODO scaling?
            let value = (100.0 * value) as u16;
            let value = if value < 272 { value } else { 272 };
            stm.draw_rectangle_filled(x as u16 * self.bar_width,
                                      (x + 1) as u16 * self.bar_width,
                                      Y_MAX - value as u16,
                                      Y_MAX,
                                      self.color_bar);
            //save history
            self.history[x as usize] = value;
            //handle max: new max or decrease
            if value as u16 > self.max[x as usize] {
                self.max[x as usize] = value;
            } else {
                self.max[x as usize] = if self.max[x as usize] > 0 { self.max[x as usize] - 1 } else { 0 };
            }
            //print max
            stm.draw_rectangle_filled(x as u16 * self.bar_width,
                                      (x + 1) as u16 * self.bar_width,
                                      Y_MAX - self.max[x as usize],
                                      core::cmp::min(Y_MAX - self.max[x as usize] + 2, Y_MAX),
                                      self.color_max);
        }

    }
}
impl<'a> SpectrumVisualizer2<'a> {
    pub fn new(history: &'a mut [u16; X_MAX as usize],
               max: &'a mut [u16; X_MAX as usize],
               bar_width: u16,
               color_bar: u16,
               color_max: u16)
               -> Box<SpectrumVisualizer2<'a>> {
        Box::new(SpectrumVisualizer2 {
                     history: history,
                     max: max,
                     bar_width: bar_width,
                     color_bar: color_bar,
                     color_max: color_max,
                     dft: DFT::new(LENGTH),
                     signal: vec![0.0;LENGTH],
                     spectrum: vec![0.0;LENGTH],
                 })
    }
}

