
use super::twiddles;
use super::butterfly::*;
use core;
use collections::Vec;

pub struct FFT {
    fft_len: usize,
    spectrum: Vec<f32>,
    twiddles: &'static [f32],
    odd_bits: bool
}

impl FFT {
    pub fn new(fft_len: usize) -> Self {
        let num_bits = fft_len.trailing_zeros();

        FFT {
            fft_len: fft_len,
            spectrum: Vec::with_capacity(2 * fft_len),
            twiddles: twiddles::get_twiddles(fft_len),
            odd_bits: num_bits % 2 != 0
        }
    }

    pub fn process(&mut self, signal: &[f32], magnitudes: &mut [f32]) {
        let fft_len = self.fft_len;
        self.spectrum.clear();
        self.spectrum.resize(2 * fft_len, 0.0);
        self.prepare(fft_len, signal, 0, 1);

        self.process_inplace();

        for (i, val) in self.spectrum.chunks(2).enumerate() {
            let power = val[0] * val[0] + val[1] * val[1];
            let magnitude = unsafe { core::intrinsics::sqrtf32(power) };
            magnitudes[i] = magnitude;
        }
    }

    fn prepare(&mut self, size: usize, signal: &[f32], idx : usize, stride: usize) {
        match size {
            4 => for i in 0..4 {
                self.spectrum[2 * (idx + i)] = signal[i * stride];
            },
            2  => for i in 0..2 {
                self.spectrum[2 * (idx + i)] = signal[i * stride];
            },
            _ => for i in 0..4 {
                self.prepare(size / 4, &signal[i * stride..], idx + i * (size / 4), stride * 4);
            }
        }
    }

    fn process_inplace(&mut self) {
        let mut current_size = if self.odd_bits {
            butterfly2_multi_inplace(&mut self.spectrum);
            8
        } else {
            butterfly4_multi_inplace(&mut self.spectrum);
            16
        };
        
        let fft_len = self.fft_len;
        while current_size <= fft_len {
            let group_stride = fft_len / current_size;
            for i in 0..group_stride {
                self.process_stride(i * current_size, group_stride, current_size / 4)
            }
            current_size *= 4;
        }
    }

    fn process_stride(&mut self, start : usize, stride : usize, num_ffts : usize) {
        let data = &mut self.spectrum[2 * start..];
        let mut tw_idx = [0;3];
        let mut scratch = [0.0; 12];
        for idx in 0..num_ffts {
            let idx = 2 * idx;
            multiply_complex(&data[idx + 2 * num_ffts..], &self.twiddles[tw_idx[0]..], &mut scratch[0..]);
            multiply_complex(&data[idx + 4 * num_ffts..], &self.twiddles[tw_idx[1]..], &mut scratch[2..]);
            multiply_complex(&data[idx + 6 * num_ffts..], &self.twiddles[tw_idx[2]..], &mut scratch[4..]);
            scratch[10] = data[idx] - scratch[2];
            scratch[11] = data[idx + 1] - scratch[3];
            data[idx] += scratch[2];
            data[idx + 1] += scratch[3];
            scratch[6] = scratch[0] + scratch[4];
            scratch[7] = scratch[1] + scratch[5];
            scratch[8] = scratch[0] - scratch[4];
            scratch[9] = scratch[1] - scratch[5];
            data[idx + 4 * num_ffts] = data[idx] - scratch[6];
            data[idx + 4 * num_ffts + 1] = data[idx + 1] - scratch[7];
            data[idx] += scratch[6];
            data[idx + 1] += scratch[7];

            data[idx + 2 * num_ffts] = scratch[10] + scratch[9];
            data[idx + 2 * num_ffts + 1] = scratch[11] - scratch[8];
            data[idx + 6 * num_ffts] = scratch[10] - scratch[9];
            data[idx + 6 * num_ffts + 1] = scratch[11] + scratch[8];

            tw_idx[0] += 2 * stride;
            tw_idx[1] += 4 * stride;
            tw_idx[2] += 6 * stride;
        }
    }
}

fn multiply_complex(a : &[f32], b : &[f32], c : &mut[f32]) {
    c[0] = a[0] * b[0] - a[1] * b[1];
    c[1] = a[1] * b[0] + a[0] * b[1];
}