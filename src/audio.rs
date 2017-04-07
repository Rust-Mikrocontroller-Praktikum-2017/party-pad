#![no_std]
#![no_main]
extern crate stm32f7_discovery as stm32f7;
use stm32f7::{system_clock, i2c, board, embedded};
use core::ptr;

fn get_microphone_input() {
    // poll for new audio data
    while !sai_2.bsr.read().freq() {} // fifo_request_flag
    let data0 = sai_2.bdr.read().data();
    while !sai_2.bsr.read().freq() {} // fifo_request_flag
    let data1 = sai_2.bdr.read().data();

    lcd.set_next_col(data0, data1);
}
