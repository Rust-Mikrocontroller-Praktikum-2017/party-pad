

pub fn butterfly2_direct(a : &mut [f32], b : &mut [f32]) {
    let temp_re = a[0];
    let temp_im = a[1];
    a[0] = temp_re + b[0];
    a[1] = temp_im + b[1];
    b[0] = temp_re - b[0];
    b[1] = temp_im - b[1];

    let temp_re = a[2];
    let temp_im = a[3];
    a[2] = temp_re + b[2];
    a[3] = temp_im + b[3];
    b[2] = temp_re - b[2];
    b[3] = temp_im - b[3];
}

pub fn butterfly2_inplace(spectrum : &mut [f32]) {
    let temp_re = spectrum[0];
    let temp_im = spectrum[1];
    spectrum[0] = temp_re + spectrum[2];
    spectrum[1] = temp_im + spectrum[3];
    spectrum[2] = temp_re - spectrum[2];
    spectrum[3] = temp_im - spectrum[3];
}

pub fn butterfly2_multi_inplace(spectrum : &mut [f32]) {
    for chunk in spectrum.chunks_mut(4) {
        butterfly2_inplace(chunk);
    }    
}

pub fn butterfly4_inplace(spectrum : &mut [f32]) {
    {
        let (a, b) = spectrum.split_at_mut(4);
        butterfly2_direct(a, b);

        // Rotate
        b[2] = b[3];
        b[3] = -b[2];

        butterfly2_inplace(a);
        butterfly2_inplace(b);
    }
    spectrum.swap(2, 4);
}

pub fn butterfly4_multi_inplace(spectrum : &mut [f32]) {
    for chunk in spectrum.chunks_mut(8) {
        butterfly4_inplace(chunk);
    }
}