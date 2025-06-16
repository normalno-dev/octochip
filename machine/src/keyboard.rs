use crate::error::Error;

pub struct Keyboard(u16);

impl Keyboard {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn is_key_pressed(&self, key: u8) -> bool {
        if key > 0xF {
            return false;
        }

        let mask = 1 << key;
        self.0 & mask != 0
    }

    pub fn get_keys(&self) -> u16 {
        self.0
    }

    pub fn any_key_pressed(&self) -> bool {
        self.0 != 0
    }

    pub fn clear_all_keys(&mut self) {
        self.0 = 0
    }

    pub fn set_key(&mut self, key: u8, pressed: bool) {
        if key > 0xF {
            return;
        }

        if pressed {
            self.0 |= 1 << key;
        } else {
            self.0 &= !(1 << key);
        }
    }

    pub fn get_first_pressed_key(&self) -> Option<u8> {
        for i in 0..16 {
            if self.is_key_pressed(i) {
                return Some(i);
            }
        }
        None
    }
}
