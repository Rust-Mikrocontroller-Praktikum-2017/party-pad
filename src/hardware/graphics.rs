
use visuals::constants::*;

const SIZE : usize = X_MAX as usize * Y_MAX as usize;
const LAYER_1_ADDR : usize = 0xC000_0000;
const LAYER_2_ADDR : usize = LAYER_1_ADDR + 2 * SIZE;

use core::slice;

pub struct Graphics {
    pub buffer : &'static mut [u32]
}

impl Graphics {
    pub fn new() -> Graphics {
        let ptr = LAYER_1_ADDR as *mut u32;
        let slice = unsafe { slice::from_raw_parts_mut(ptr, SIZE) };
        Graphics {
            buffer : slice
        }
    }

    pub fn clear(&mut self) {
        for mut color in self.buffer.iter_mut() {
            *color = 0;
        }
    }

    pub fn draw_rectangle_filled(&mut self,
                                 x_min: u16,
                                 x_max: u16,
                                 y_min: u16,
                                 y_max: u16,
                                 color: u16) {
        for y in y_min..y_max {
            for x in x_min..x_max {
                let idx = y * X_MAX + x;
                self.buffer[idx as usize] = color as u32 & 0x0000ffffb;
            }
        }
    }
}