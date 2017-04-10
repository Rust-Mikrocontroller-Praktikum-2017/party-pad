#![no_std]
#![no_main]
#![feature(collections)]

extern crate stm32f7_discovery as stm32f7;
extern crate collections;
// initialization routines for .data and .bss
extern crate r0;
use stm32f7::{system_clock, sdram, lcd, i2c, audio, touch, board, embedded};

mod visuals;

use collections::boxed::Box;
use visuals::constants::*;
use visuals::default_visualizer::DefaultVisualizer;
use visuals::direct_mic_visualizer::DirectMicVisualizer;
use visuals::energy_visualizer::EnergyVisualizer;
use visuals::direct_mic_batch_vz::DirectMicBatchVisualizer;
use visuals::sliding_sound_wave_vz::SlidingSoundVisualizer;
use visuals::Visualizer;

fn main(mut stm: STM) -> ! {
    stm.lcd.clear_screen();
    let mut param = VizParameter{spectrum: [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
                                   1.0, 1.0, 1.0, 1.0],
                                   mic_input: [1000;32]};

    let mut pos0 = 0; //TODO move completely to direct mic? lifetime issues..
    let mut pos1 = 0; 
    let mut pos2 = 0;
    let mut last_radius = 0;
    let mut buffer = [0;X_MAX as usize];
    let direct_mic_viz: Box<Visualizer> = DirectMicVisualizer::new(&mut pos0, 2);
    let direct_mic_batch_viz: Box<Visualizer> = DirectMicBatchVisualizer::new(&mut pos1, 2);
    let sliding_viz: Box<Visualizer> = SlidingSoundVisualizer::new(&mut buffer, &mut pos2, 2);
    let default_viz: Box<Visualizer> =  DefaultVisualizer::new(
                          0xFFFF,
                          0xFC00);
    let energy_viz: Box<Visualizer> = sliding_viz;
    let mut current_visualizer = energy_viz;
    let mut data0;
    let mut data1;
    let mut count;
    stm.lcd.set_background_color(lcd::Color::rgb(0, 0, 0));
    loop {
        count = 0;
        /*
        while count + 1 < param.mic_input.len() {
            while !stm.sai_2.bsr.read().freq() {} // fifo_request_flag
            data0 = stm.sai_2.bdr.read().data();
            while !stm.sai_2.bsr.read().freq() {} // fifo_request_flag
            data1 = stm.sai_2.bdr.read().data();

            param.mic_input[count] = data0 as i16;
            param.mic_input[count+1] = data1 as i16;

            count += 2;
        }
        */

        //while count < param.mic_input.len() {
        while count < 1 {
            while !stm.sai_2.bsr.read().freq() {} // fifo_request_flag
            data0 = stm.sai_2.bdr.read().data();
            while !stm.sai_2.bsr.read().freq() {} // fifo_request_flag
            data1 = stm.sai_2.bdr.read().data();

            param.mic_input[count] = data0 as i16;

            count += 1;
        }

        current_visualizer.draw(&mut stm, &mut param);
                stm.lcd.clear_screen();

        /*
        stm.lcd.clear_screen();
        let radius = 0;
        stm.draw_fill_ring(240, 131, radius,radius + 20,cons::BLUE);
        */
        
    }
}

pub struct STM {
    gpio: embedded::interfaces::gpio::Gpio,
    i2c_3: i2c::I2C,
    lcd: stm32f7::lcd::Lcd,
    led: embedded::interfaces::gpio::OutputPin,
    sai_2: &'static mut board::sai::Sai,

}

pub struct VizParameter {
    spectrum: [f32; 16],
    mic_input: [i16; 32],
}

#[no_mangle]
pub unsafe extern "C" fn reset() -> ! {
    extern "C" {
        static __DATA_LOAD: u32;
        static __DATA_END: u32;
        static mut __DATA_START: u32;
        static mut __BSS_START: u32;
        static mut __BSS_END: u32;
    }
    let data_load = &__DATA_LOAD;
    let data_start = &mut __DATA_START;
    let data_end = &__DATA_END;
    let bss_start = &mut __BSS_START;
    let bss_end = &__BSS_END;

    // initializes the .data section
    //(copy the data segment initializers from flash to RAM)
    r0::init_data(data_start, data_end, data_load);
    // zeroes the .bss section
    r0::zero_bss(bss_start, bss_end);

    stm32f7::heap::init();

    // enable floating point unit
    let scb = stm32f7::cortex_m::peripheral::scb_mut();
    scb.cpacr.modify(|v| v | 0b1111 << 20);

    let stm = init(board::hw());
    main(stm);
}

fn init(hw: board::Hardware) -> STM {
    let board::Hardware {
        rcc,
        pwr,
        flash,
        fmc,
        ltdc,
        gpio_a,
        gpio_b,
        gpio_c,
        gpio_d,
        gpio_e,
        gpio_f,
        gpio_g,
        gpio_h,
        gpio_i,
        gpio_j,
        gpio_k,
        i2c_3,
        sai_2,
        ..
    } = hw;

    use embedded::interfaces::gpio::{self, Gpio};
    let mut gpio = Gpio::new(gpio_a,
                             gpio_b,
                             gpio_c,
                             gpio_d,
                             gpio_e,
                             gpio_f,
                             gpio_g,
                             gpio_h,
                             gpio_i,
                             gpio_j,
                             gpio_k);




    system_clock::init(rcc, pwr, flash);
    // enable all gpio ports
    rcc.ahb1enr
        .update(|r| {
            r.set_gpioaen(true);
            r.set_gpioben(true);
            r.set_gpiocen(true);
            r.set_gpioden(true);
            r.set_gpioeen(true);
            r.set_gpiofen(true);
            r.set_gpiogen(true);
            r.set_gpiohen(true);
            r.set_gpioien(true);
            r.set_gpiojen(true);
            r.set_gpioken(true);
        });
    // init sdram (display buffer)
    sdram::init(rcc, fmc, &mut gpio);
    // lcd controller
    let lcd = lcd::init(ltdc, rcc, &mut gpio);

    // configure led pin as output pin
    let led_pin = (gpio::Port::PortI, gpio::Pin::Pin1);
    let mut led = gpio.to_output(led_pin,
                                 gpio::OutputType::PushPull,
                                 gpio::OutputSpeed::Low,
                                 gpio::Resistor::NoPull)
        .expect("led pin already in use");

    // turn led on
    led.set(true);

    //i2c
    i2c::init_pins_and_clocks(rcc, &mut gpio);
    let mut i2c_3 = i2c::init(i2c_3);

    touch::check_family_id(&mut i2c_3).unwrap();

    // sai and stereo microphone
    audio::init_sai_2_pins(&mut gpio);
    audio::init_sai_2(sai_2, rcc);
    assert!(audio::init_wm8994(&mut i2c_3).is_ok());

    STM {
        gpio: gpio,
        i2c_3: i2c_3,
        lcd: lcd,
        led: led,
        sai_2: sai_2,
    }
}
