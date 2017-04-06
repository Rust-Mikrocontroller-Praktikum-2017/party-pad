#![no_std]
#![no_main]
extern crate stm32f7_discovery as stm32f7;
// initialization routines for .data and .bss
extern crate r0;
use stm32f7::{system_clock, sdram, lcd, i2c, touch, board, embedded};
use core::ptr;

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

    /*// enable floating point unit
    unsafe {
        let scb = stm32f7::cortex_m::peripheral::scb_mut();
        scb.cpacr.modify(|v| v | 0b1111 << 20);
    }
*/
    main(board::hw());
}

fn main(hw: board::Hardware) -> ! {
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

    let mut last_led_toggle = system_clock::ticks();
    lcd.clear_screen();

    //i2c
    i2c::init_pins_and_clocks(rcc, &mut gpio);
    let mut i2c_3 = i2c::init(i2c_3);

    touch::check_family_id(&mut i2c_3).unwrap();

    let mut color1 = 0xf00f | 0x8000;
    let mut color2 = 0xffff | 0x8000;
    let mut spiral_drawn = false;
    let mut swap_color = false;

    loop {

        ///////// spiral
        for _ in &touch::touches(&mut i2c_3).unwrap() {
            swap_color = true;
        }
        if swap_color {
            let temp_color = color1;
            color1 = color2;
            color2 = temp_color;
            swap_color = false;
            spiral_drawn = false;
        }
        if !spiral_drawn {
            draw_spiral(color1, color2, &mut lcd);
        }


        //////// LED stuff
        let ticks = system_clock::ticks();
        // every 0.5 seconds
        if ticks - last_led_toggle >= 500 {
            // toggle the led
            let led_current = led.get();
            led.set(!led_current);
            last_led_toggle = ticks;
        }
    }
}

fn draw_rectangle(x_min: u16, x_max: u16, y_min: u16, y_max: u16, color: u16, lcd: &mut lcd::Lcd) {
    for x in x_min..x_max {
        lcd.print_point_color_at(x, y_min, color);
        lcd.print_point_color_at(x, y_max - 1, color);
    }
    for y in y_min + 1..y_max - 1 {
        lcd.print_point_color_at(x_min, y, color);
        lcd.print_point_color_at(x_max - 1, y, color);
    }
}

fn draw_spiral(color1: u16, color2: u16, mut lcd: &mut lcd::Lcd) {
    let mut x_min = 0;
    let mut x_max = 480;
    let mut y_min = 0;
    let mut y_max = 272;

    let mut start_color = color1;
    let mut color = start_color;

    while y_min < 135 {
        // only works because 480 is dividable by 5

        for _ in 0..5 {
            draw_rectangle(x_min, x_max, y_min, y_max, color, &mut lcd);
            // update variables
            x_min += 1;
            x_max -= 1;
            y_min += 1;
            y_max -= 1;
        }
        color = if color == color1 { color2 } else { color1 }
    }
    draw_rectangle(x_min, x_max, y_min, y_max, color, &mut lcd);
}

/*
fn cordic(mut beta: f32, n: usize) -> [f32; 2] {
    // This function computes v = [cos(beta), sin(beta)] (beta in radians)
    // using n iterations. Increasing n will increase the precision.
    let pi = 3.14;

    if beta < -pi / 2.0 || beta > pi / 2.0 {
        let mut v;
        if beta < 0.0 {
            v = cordic(beta + pi, n);
        } else {
            v = cordic(beta - pi, n);
        }
        // flip the sign for second or third quadrant
        v[0] = -v[0];
        v[1] = -v[1];
        return v;
    }

    /* Initialization of tables of constants used by CORDIC
     * need a table of arctangents of negative powers of two, in radians:
     * angles = atan(2.^-(0:27));
     */
    let angles: [f32; 28] = [0.78539816339745,
                             0.46364760900081,
                             0.24497866312686,
                             0.12435499454676,
                             0.06241880999596,
                             0.03123983343027,
                             0.01562372862048,
                             0.00781234106010,
                             0.00390623013197,
                             0.00195312251648,
                             0.00097656218956,
                             0.00048828121119,
                             0.00024414062015,
                             0.00012207031189,
                             0.00006103515617,
                             0.00003051757812,
                             0.00001525878906,
                             0.00000762939453,
                             0.00000381469727,
                             0.00000190734863,
                             0.00000095367432,
                             0.00000047683716,
                             0.00000023841858,
                             0.00000011920929,
                             0.00000005960464,
                             0.00000002980232,
                             0.00000001490116,
                             0.00000000745058];
    // and a table of products of reciprocal lengths of vectors [1, 2^-2j]:
    // Kvalues = cumprod(1./abs(1 + 1j*2.^(-(0:23))))
    let Kvalues: [f32; 24] = [0.70710678118655,
                              0.63245553203368,
                              0.61357199107790,
                              0.60883391251775,
                              0.60764825625617,
                              0.60735177014130,
                              0.60727764409353,
                              0.60725911229889,
                              0.60725447933256,
                              0.60725332108988,
                              0.60725303152913,
                              0.60725295913894,
                              0.60725294104140,
                              0.60725293651701,
                              0.60725293538591,
                              0.60725293510314,
                              0.60725293503245,
                              0.60725293501477,
                              0.60725293501035,
                              0.60725293500925,
                              0.60725293500897,
                              0.60725293500890,
                              0.60725293500889,
                              0.60725293500888];
    let mut Kn = Kvalues[if n < Kvalues.len() { n } else { Kvalues.len() }];

    // Initialize loop variables:
    // start with 2-vector cosine and sine of zero
    let mut v0 = 1.0;
    let mut v1 = 0.0;
    let mut poweroftwo = 1;
    let mut angle = angles[1];
    let mut sigma = 1.0;
    // Iterations
    for j in 0..n {

        if beta < 0.0 {
            sigma = -1.0;
        } else {
            sigma = 1.0;
        }

        //let mut factor = sigma * poweroftwo;
        // Note the matrix multiplication can be done using scaling by powers of two
        // and addition subtraction
        //let mut R = vec![1, -factor, factor, 1];
        //v = R * v; // 2-by-2 matrix multiply

        let x = v0 - sigma * (v1 * 2.0 ^ (-(j as isize)));
        let y = sigma * (v0 * 2.0 ^ (-(j as isize))) + v1;
        v0 = x;
        v1 = y;

        beta = beta - sigma * angle; // update the remaining angle
        poweroftwo = poweroftwo / 2;
        // update the angle from table, or eventually by just dividing by two

        if j + 2 > angles.len() {
            angle = angle / 2.0;
        } else {
            angle = angles[j + 2];
        }
    }

    // Adjust length of output vector to be [cos(beta), sin(beta)]:
    v0 = v0 * Kn;
    v1 = v1 * Kn;

    return [v0, v1];
}
*/
