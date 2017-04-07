use super::stm;
use stm32f7::system_clock;

#[derive(Clone)]
pub struct xy {
    pub x_min: u16,
    pub x_max: u16,
    pub y_min: u16,
    pub y_max: u16,
}

pub fn blink_led(stm: &mut stm) -> usize {
    // toggle the led
    let led_current = stm.led.get();
    stm.led.set(!led_current);
    system_clock::ticks()
}

pub fn draw_rectangle(xy: &xy, color: u16, mut stm: &mut stm) {
    for x in xy.x_min..xy.x_max {
        stm.lcd.print_point_color_at(x, xy.y_min, color);
        stm.lcd.print_point_color_at(x, xy.y_max - 1, color);
    }
    for y in xy.y_min + 1..xy.y_max - 1 {
        stm.lcd.print_point_color_at(xy.x_min, y, color);
        stm.lcd.print_point_color_at(xy.x_max - 1, y, color);
    }
}

pub fn print_fill_rect(stm: &mut stm,
                   x_start: u16,
                   x_end: u16,
                   y_start: u16,
                   y_end: u16,
                   color: u16) {

    for x in x_start..x_end {
        for y in y_start..y_end {
            stm.lcd.print_point_color_at(x as u16, y as u16, color);
        }
    }
}

pub fn draw_spiral(xy: xy, color1: u16, color2: u16, mut stm: &mut stm) {
    let mut yx = xy.clone();
    let mut start_color = color1;
    let mut color = start_color;

    while yx.y_min < 135 {
        // only works because 480 is dividable by 5

        for _ in 0..5 {
            draw_rectangle(&yx, color, &mut stm);
            // update variables
            yx.x_min += 1;
            yx.x_max -= 1;
            yx.y_min += 1;
            yx.y_max -= 1;
        }
        color = if color == color1 { color2 } else { color1 }
    }
    draw_rectangle(&yx, color, &mut stm);
}
