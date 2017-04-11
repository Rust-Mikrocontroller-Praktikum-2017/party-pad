use super::super::STM;
use core;
use visuals::constants::*;


use stm32f7::system_clock;


impl STM {
    pub fn blink_led(&mut self) -> usize {
        // toggle the led
        let led_current = self.led.get();
        self.led.set(!led_current);
        system_clock::ticks()
    }

    pub fn draw_circle(&mut self, x_center: u16, y_center: u16, radius: u16, color: u16) {
        let mut x_offset = radius;
        for y_offset in 0..radius {
            while euclidean_dist_squared(x_center + x_offset,
                                         y_center + y_offset,
                                         x_center,
                                         y_center) > radius * radius {
                x_offset -= 1;
            }
            self.lcd
                .print_point_color_at(x_center + x_offset, y_center + y_offset, color);
            self.lcd
                .print_point_color_at(x_center + x_offset, y_center - y_offset, color);
            self.lcd
                .print_point_color_at(x_center - x_offset, y_center + y_offset, color);
            self.lcd
                .print_point_color_at(x_center - x_offset, y_center - y_offset, color);

        }
    }

    pub fn draw_circle_filled(&mut self, x_center: u16, y_center: u16, radius: u16, color: u16) {
        /*
        assert!(x_center + radius <= x_max && y_center + radius <= y_max);
        assert!(x_center - radius <= x_max && y_center - radius <= y_max);
        //assert!(is_legal_coord(x_center, y_center));
        */

        //self.lcd.print_point_color_at(x_center, y_center, color);
        let mut x_offset = radius + 1;
        for y_offset in 0..radius + 1 {
            while euclidean_dist_squared(x_center + x_offset,
                                         y_center + y_offset,
                                         x_center,
                                         y_center) > radius * radius {
                x_offset -= 1;
            }
            self.draw_rectangle_filled(x_center - x_offset,
                                     x_center + x_offset + 1,
                                     y_center + y_offset,
                                     y_center + y_offset + 1,
                                     color);
            self.draw_rectangle_filled(x_center - x_offset,
                                     x_center + x_offset + 1,
                                     y_center - y_offset,
                                     y_center - y_offset + 1,
                                     color);
            /*
            self.lcd
                .print_point_color_at(x_center + x_offset, y_center + y_offset, color);
            self.lcd
                .print_point_color_at(x_center + x_offset, y_center - y_offset, color);
            self.lcd
                .print_point_color_at(x_center - x_offset, y_center + y_offset, color);
            self.lcd
                .print_point_color_at(x_center - x_offset, y_center - y_offset, color);
            */

        }
    }



    pub fn draw_ring_filled(&mut self,
                          x_center: u16,
                          y_center: u16,
                          radius_inner: u16,
                          radius_outer: u16,
                          color: u16) {
        //assert!(radius_outer > radius_inner);

        let radius_outer_squared = radius_outer * radius_outer;
        let radius_inner_squared = radius_inner * radius_inner;
        let mut x_offset_outer = radius_outer + 1;
        let mut x_offset_inner;
        /*for every horizontal line, draw line between outer and inner circle,
            uses symmetry
        */
        for y_offset in 0..radius_outer + 1 {
            //compute outer circle point
            while euclidean_dist_squared(x_center + x_offset_outer,
                                         y_center + y_offset,
                                         x_center,
                                         y_center) > radius_outer_squared {
                x_offset_outer -= 1;
            }
            //if inner circle is intersected, compute inner circle point
            if y_offset < radius_inner {
                x_offset_inner = x_offset_outer;
                while euclidean_dist_squared(x_center + x_offset_inner,
                                             y_center + y_offset,
                                             x_center,
                                             y_center) >
                      radius_inner_squared {
                    x_offset_inner -= 1;
                }
                //lower right quarter
                self.draw_line_h(x_center + x_offset_inner,
                                         x_center + x_offset_outer + 1,
                                         y_center + y_offset,
                                         color);
                //lower left quarter
                self.draw_line_h(x_center - x_offset_outer,
                                         x_center - x_offset_inner + 1,
                                         y_center + y_offset,
                                         color);
                //upper left quarter
                self.draw_line_h(x_center - x_offset_outer,
                                         x_center - x_offset_inner + 1,
                                         y_center - y_offset,
                                         color);
                //upper right quarter
                self.draw_line_h(x_center + x_offset_inner,
                                         x_center + x_offset_outer + 1,
                                         y_center - y_offset,
                                         color);
            } else {
                //if inner circle is not intersected, draw line between outer circle points
                self.draw_line_h(x_center - x_offset_outer,
                                         x_center + x_offset_outer + 1,
                                         y_center - y_offset,
                                         color);
                self.draw_line_h(x_center - x_offset_outer,
                                         x_center + x_offset_outer + 1,
                                         y_center + y_offset,
                                         color);
            }

        }
    }

    pub fn draw_line_h(&mut self, x_min: u16, x_max: u16, y: u16, color: u16) {
        for x in x_min..x_max {
            self.lcd.print_point_color_at(x, y, color);
        }
    }

    pub fn draw_line_v(&mut self, x: u16, y_min: u16, y_max: u16,  color: u16) {
        for y in y_min..y_max {
            self.lcd.print_point_color_at(x, y, color);
        }
    }
    
    pub fn draw_rectangle(&mut self, x_min: u16,x_max: u16, y_min: u16, y_max: u16, color: u16) {
        for x in x_min..x_max {
            self.lcd.print_point_color_at(x, y_min, color);
            self.lcd.print_point_color_at(x, y_max - 1, color);
        }
        for y in y_min + 1..y_max - 1 {
            self.lcd.print_point_color_at(x_min, y, color);
            self.lcd.print_point_color_at(x_max - 1, y, color);
        }
    }

    pub fn draw_rectangle_filled(&mut self,
                               x_min: u16,
                               x_max: u16,
                               y_min: u16,
                               y_max: u16,
                               color: u16) {

        for x in x_min..x_max {
            for y in y_min..y_max {
                self.lcd.print_point_color_at(x as u16, y as u16, color);
            }
        }
    }

    pub fn draw_spiral(&mut self, mut x_min: u16,mut x_max: u16, mut y_min: u16, mut y_max: u16, color1: u16, color2: u16) {
        let mut start_color = color1;
        let mut color = start_color;

        while y_min < 135 {
            // only works because 480 is dividable by 5

            for _ in 0..5 {
                self.draw_rectangle(x_min,x_max,y_min,y_max, color);
                // update variables
                x_min += 1;
                x_max -= 1;
                y_min += 1;
                y_max -= 1;
            }
            color = if color == color1 { color2 } else { color1 }
        }
        self.draw_rectangle(x_min,x_max,y_min,y_max, color);

    }


    pub fn print_bar_signed(&mut self, value: i16, pos: u16, width: u16, y_max: u16, color: u16) {
        /*
    let x_max = 480;
    let y_max: u16 = 272;

    assert!(pos < x_max);
    assert!(pos + width < x_max);
    */

        //TODO how to scale properly?
        let scale_factor = value as f32 * 10.0 / core::i16::MAX as f32;
        //let scale_factor = value as f32 / core::i16::MAX as f32;
        //TODO constants
        let value = core::cmp::max(core::cmp::min((Y_MAX as f32 * scale_factor) as i16,
                                                  130 as i16),
                                   -130 as i16);
        //print_fill_rect(&mut lcd, pos, 20, pos+width, 20, 0x801F);

        if value > 0 {
            self.draw_rectangle_filled(pos,
                                     pos + width,
                                     y_max / 2,
                                     (y_max as i16 / 2 + value) as u16,
                                     color);
        } else {
            self.draw_rectangle_filled(pos,
                                     pos + width,
                                     (y_max as i16 / 2 + value) as u16,
                                     y_max / 2,
                                     color);

        }
    }
    /*pub fn draw_rectangle_strip(&mut self,
                                x: u16,
                                y: u16,
                                width: u16,
                                height: u16,
                                color_low: u16,
                                color_high: u16) {
                                    
        for  {
            let color = 
            self.draw_rectangle_filled(x,x+width,y,y+height,);
        }
    }*/
}

//TODO move to different file?
fn euclidean_dist_squared(x_1: u16, y_1: u16, x_2: u16, y_2: u16) -> u16 {
    let x_low;
    let x_high;
    let y_low;
    let y_high;
    if x_1 <= x_2 {
        x_low = x_1;
        x_high = x_2;
    } else {
        x_low = x_2;
        x_high = x_1;
    }
    if y_1 <= y_2 {
        y_low = y_1;
        y_high = y_2;
    } else {
        y_low = y_2;
        y_high = y_1;
    }
    x_high - x_low;
    y_high - y_low;
    (x_high - x_low) * (x_high - x_low) + (y_high - y_low) * (y_high - y_low)
}

