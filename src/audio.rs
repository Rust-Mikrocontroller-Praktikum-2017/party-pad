use hardware::STM;

pub fn get_microphone_input(stm: &mut STM, audio_data: &mut [i16], mode: bool) {
    if mode {
        //mode == true => get data from both mics, mic right at even indices
        for i in 0..audio_data.len() / 2 {
            // poll for new audio data
            while !stm.sai_2.bsr.read().freq() {} // fifo_request_flag
            audio_data[2 * i] = stm.sai_2.bdr.read().data() as i16;
            while !stm.sai_2.bsr.read().freq() {} // fifo_request_flag
            audio_data[2 * i + 1] = stm.sai_2.bdr.read().data() as i16;
        }
    } else {
        //mode == false => get data only from right mic
        for data in audio_data {
            // poll for new audio data
            while !stm.sai_2.bsr.read().freq() {} // fifo_request_flag
            *data = stm.sai_2.bdr.read().data() as i16;
            while !stm.sai_2.bsr.read().freq() {} // fifo_request_flag
        }
    }
}
