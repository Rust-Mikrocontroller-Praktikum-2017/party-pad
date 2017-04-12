
use super::twiddles;
use super::butterfly::*;

pub struct FFT {
    fft_len: usize,
    twiddles: &'static [f32],
    odd_bits: bool
}

impl FFT {
    pub fn new(fft_len: usize) -> Self {
        let num_bits = fft_len.trailing_zeros();

        FFT {
            fft_len: fft_len,
            twiddles: twiddles::get_twiddles(fft_len),
            odd_bits: num_bits % 2 != 0
        }
    }

    pub fn process_inplace(&mut self, signal: &mut [f32]) {
        let mut current_size = if self.odd_bits {
            butterfly2_multi_inplace(signal);
            8
        } else {
            butterfly4(signal);
            16
        };
        
        while current_size <= self.fft_len {
            let group_stride = self.fft_len / current_size;
            for i in 0..group_stride {
                self.butterfly_4(&mut signal[i * current_size..], group_stride, current_size / 4)
            }
        }
    }

    fn butterfly_4(&mut self, data: &mut [f32], stride : usize, num_ffts : usize) {
        let mut idx = 0;
        let mut tw_idx_1 = 0;
        let mut tw_idx_2 = 0;
        let mut tw_idx_3 = 0;
        let mut scratch: [f32; 12] = [0.0; 12];
        for _ in 0..num_ffts {
            scratch[0] = data.get_unchecked(idx + 1 * num_ffts) * self.twiddles[tw_idx_1];
            scratch[2] = data.get_unchecked(idx + 2 * num_ffts) * self.twiddles[tw_idx_2];
            scratch[4] = data.get_unchecked(idx + 3 * num_ffts) * self.twiddles[tw_idx_3];
            scratch[10] = data.get_unchecked(idx) - scratch[1];
            *data.get_unchecked_mut(idx) = data.get_unchecked(idx) + scratch[1];
            scratch[6] = scratch[0] + scratch[2];
            scratch[8] = scratch[0] - scratch[2];
            *data.get_unchecked_mut(idx + 2 * num_ffts) = data.get_unchecked(idx) - scratch[3];
            *data.get_unchecked_mut(idx) = data.get_unchecked(idx) + scratch[3];

            data.get_unchecked_mut(idx + num_ffts).re = scratch[5].re + scratch[4].im;
            data.get_unchecked_mut(idx + num_ffts).im = scratch[5].im - scratch[4].re;
            data.get_unchecked_mut(idx + 3 * num_ffts).re = scratch[5].re - scratch[4].im;
            data.get_unchecked_mut(idx + 3 * num_ffts).im = scratch[5].im + scratch[4].re;

            tw_idx_1 += 1 * stride;
            tw_idx_2 += 2 * stride;
            tw_idx_3 += 3 * stride;
            idx += 1;
        }
    }
}