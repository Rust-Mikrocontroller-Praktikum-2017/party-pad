
use super::twiddles;

pub struct DFT {
    fft_len: usize,
    twiddles: &'static [f32]
}

impl DFT {
    pub fn create(fft_len: usize) -> Self {
        DFT {
            fft_len: fft_len,
            twiddles: twiddles::get_twiddles(fft_len)
        }
    }

    pub fn process(&mut self, signal: &[f32], magnitudes: &mut[f32]) {
        assert_eq!(self.fft_len, signal.len());
        assert_eq!(self.fft_len, magnitudes.len());
        for k in 0..self.fft_len {
            let mut twiddle_index = 0;
            let mut spectrum_re = 0.0;
            let mut spectrum_im = 0.0;
            for input in signal {
                let twiddle_re = self.twiddles[2 * twiddle_index];
                let twiddle_im = self.twiddles[2 * twiddle_index + 1];
                spectrum_re += input * twiddle_re;
                spectrum_im += input * twiddle_im;
                twiddle_index += k;
                if twiddle_index >= self.fft_len {
                    twiddle_index -= self.fft_len;
                }
            }
            let power = spectrum_re * spectrum_re + spectrum_im * spectrum_im;
            //let magnitude = sqrtf(power);
            magnitudes[k] = power;
        }
    }
}

fn sqrtf(n : f32) -> f32 {
    let mut x = 0.5 * n;
    while {
        let error = (x*x - n) / x - 1.0;
        error > 0.01 && error < -0.01
    } {
        x = 0.5 * (x + n / x);
    }
    x
}