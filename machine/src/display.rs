use crate::error::Error;
use std::slice::Windows;

pub struct Display {
    // row-major frame buffer
    framebuffer: Vec<u8>,

    height: u8,
    width: u8,
}

impl Display {
    pub fn new() -> Self {
        let height = 32;
        let width = 64;

        Self {
            framebuffer: Self::alloc_framebuffer(height, width),
            height,
            width,
        }
    }

    pub fn height(&self) -> u8 {
        self.height
    }

    pub fn width(&self) -> u8 {
        self.width
    }

    pub fn clear(&mut self) {
        self.framebuffer = Self::alloc_framebuffer(self.height, self.width);
    }

    pub fn get_pixel(&self, x: u8, y: u8) -> bool {
        if x >= self.width || y >= self.height {
            return false;
        }

        let (byte_idx, bit_idx) = self.pixel_to_bit_offset(x, y);
        let v = (self.framebuffer[byte_idx] >> bit_idx) & 1 == 1;

        v
    }

    pub fn draw_sprite(&mut self, x: u8, y: u8, sprite: &[u8]) -> bool {
        let mut collision = false;

        for (row, sprite_byte) in sprite.iter().copied().enumerate() {
            let y_pos = y + row as u8;

            // stop if sprite is out of screen
            if y_pos > self.height {
                break;
            }

            for bit in 0..8 {
                let x_pos = x + bit;

                // stop if sprite is out of screen
                if x_pos > self.width {
                    break;
                }

                let sprite_pixel = (sprite_byte >> (7 - bit)) & 1;
                if sprite_pixel == 1 {
                    if self.get_pixel(x_pos, y_pos) {
                        collision = true
                    }

                    self.toggle_pixel(x_pos, y_pos);
                }
            }
        }

        collision
    }

    fn set_pixel(&mut self, x: u8, y: u8, value: bool) {
        if x >= self.width || y >= self.height {
            return;
        }

        let (byte_idx, bit_idx) = self.pixel_to_bit_offset(x, y);
        if value {
            self.framebuffer[byte_idx] |= 1 << bit_idx;
        } else {
            self.framebuffer[byte_idx] &= !(1 << bit_idx);
        }
    }

    fn toggle_pixel(&mut self, x: u8, y: u8) {
        if x < self.width && y < self.height {
            let pixel = self.get_pixel(x, y);
            self.set_pixel(x, y, !pixel);
        }
    }

    fn pixel_to_bit_offset(&self, x: u8, y: u8) -> (usize, usize) {
        // in row-major, new row starts at byte y*N,
        // where N is count of bytes in a row what is equal to number of
        // columns divided by 8 as each column takes only 1 bit.

        let offset = y as u16 * self.width as u16 / 8;
        let byte_index = x as u16 / 8 + offset;
        let bit_index = 7 - x % 8;

        (byte_index as usize, bit_index as usize)
    }

    fn alloc_framebuffer(height: u8, width: u8) -> Vec<u8> {
        let mut framebuffer = Vec::new();
        let buffer_size = height as u16 * width as u16 / 8;
        for _ in 0..buffer_size {
            framebuffer.push(0);
        }

        framebuffer
    }
}

#[cfg(test)]
mod test {
    use super::Display;

    #[test]
    fn test_pixel_to_bit_offset() {
        let dsp = Display::new();

        let (byte_idx, bit_idx) = dsp.pixel_to_bit_offset(11, 7);
        assert_eq!(byte_idx, 57);
        assert_eq!(bit_idx, 4);
    }

    #[test]
    fn test_set_pixel() {
        let mut dsp = Display::new();

        dsp.set_pixel(0, 0, true);
        assert_eq!(dsp.framebuffer[0], 0b10000000);

        dsp.set_pixel(5, 10, true);
        assert_eq!(dsp.framebuffer[80], 0b00000100);

        dsp.set_pixel(33, 10, true);
        assert_eq!(dsp.framebuffer[84], 0b01000000);
    }

    #[test]
    fn test_get_pixel() {
        let mut dsp = Display::new();
        dsp.framebuffer[0] = 0b10000000;
        dsp.framebuffer[80] = 0b00000100;
        dsp.framebuffer[84] = 0b01000000;

        let pixel = dsp.get_pixel(0, 0);
        assert!(pixel);

        let pixel = dsp.get_pixel(5, 10);
        assert!(pixel);

        let pixel = dsp.get_pixel(33, 10);
        assert!(pixel);
    }

    #[test]
    fn test_toggle_pixel() {
        let mut dsp = Display::new();

        dsp.toggle_pixel(7, 17);
        assert_eq!(dsp.get_pixel(7, 17), true);

        dsp.toggle_pixel(7, 17);
        assert_eq!(dsp.get_pixel(7, 17), false);
    }
}
