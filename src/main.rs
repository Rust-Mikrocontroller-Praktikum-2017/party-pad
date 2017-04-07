#![no_std]
#![no_main]
#![feature(collections)]
pub mod visuals;

extern crate stm32f7_discovery as stm32f7;
extern crate collections;
// initialization routines for .data and .bss
extern crate r0;
use stm32f7::{system_clock, sdram, lcd, i2c, audio, touch, board, embedded};
use core::ptr;


fn main(mut stm: stm) -> ! {
    stm.lcd.clear_screen();
    let mut visuals = visuals::Visuals::new(stm);
    let spectrum: [f32; 16] = [1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0];
    let mut data0;
    let mut data1;
    loop {
        while !stm.sai_2.bsr.read().freq() {} // fifo_request_flag
        data0 = stm.sai_2.bdr.read().data();
        while !stm.sai_2.bsr.read().freq() {} // fifo_request_flag
        data1 = stm.sai_2.bdr.read().data();
        visuals.draw_with_current(spectrum);
    }
}

pub struct stm {
    gpio: embedded::interfaces::gpio::Gpio,
        i2c_3: i2c::I2C,
    lcd: stm32f7::lcd::Lcd,
    led: embedded::interfaces::gpio::OutputPin,
    sai_2: &'static mut board::sai::Sai,
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
    unsafe {
        let scb = stm32f7::cortex_m::peripheral::scb_mut();
        scb.cpacr.modify(|v| v | 0b1111 << 20);
    }
    
    let mut stm: stm = init(board::hw());
    main(stm);
}

fn init(hw: board::Hardware) -> stm {
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
    let mut lcd = lcd::init(ltdc, rcc, &mut gpio);

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

    stm {
        gpio: gpio,
        i2c_3: i2c_3,
        lcd: lcd,
        led: led,
        sai_2: sai_2,
    }
}