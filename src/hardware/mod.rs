

use stm32f7::{system_clock, sdram, lcd, i2c, audio, touch, board, embedded};
use self::embedded::interfaces::gpio;

pub struct STM {
    pub gpio: gpio::Gpio,
    pub i2c_3: i2c::I2C,
    pub lcd: lcd::Lcd,
    pub led: gpio::OutputPin,
    pub sai_2: &'static mut board::sai::Sai,
}

impl STM {
    pub fn init() -> Self {
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
        } = board::hw();

        let mut gpio = gpio::Gpio::new(gpio_a,
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
}
