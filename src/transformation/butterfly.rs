
use super::complex::Complex;

pub fn butterfly2_multi_inplace(signal : &mut [f32]) {
    for chunk in signal.chunks_mut(4) {
        let temp_re = chunk[0];
        let temp_im = chunk[1];
        chunk[0] = temp_re + chunk[2];
        chunk[1] = temp_im + chunk[3];
        chunk[2] = temp_re - chunk[2];
        chunk[3] = temp_im - chunk[3];
    }    
}

pub fn butterfly4_multi_inplace(signal : &mut [f32]) {
    let (a, b) = signal.split_at_mut(4);
    butterfly2.perform_fft_direct(a.get_unchecked_mut(0), b.get_unchecked_mut(0));
    butterfly2.perform_fft_direct(a.get_unchecked_mut(1), b.get_unchecked_mut(1));

    let rot_re = b[3];
    let rot_im = -b[2];
    *b.get_unchecked_mut(1) = twiddles::rotate_90(*b.get_unchecked(1), self.inverse);

    butterfly2.process_inplace(a);
    butterfly2.process_inplace(b);
}