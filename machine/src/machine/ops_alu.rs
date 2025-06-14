use super::Machine;
use crate::error::Error;

type Result<T> = std::result::Result<T, Error>;

impl Machine {
    pub(super) fn op_add_immediate(&mut self, vx: u8, kk: u8) -> Result<()> {
        let x = self.registers[vx as usize];
        let result = x.wrapping_add(kk);
        self.registers[vx as usize] = result;
        Ok(())
    }

    pub(super) fn op_add(&mut self, vx: u8, vy: u8) -> Result<()> {
        let x = self.registers[vx as usize];
        let y = self.registers[vy as usize];
        let (result, carry) = x.overflowing_add(y);

        self.registers[0xF] = carry as u8;
        self.registers[vx as usize] = result;
        Ok(())
    }

    pub(super) fn op_subtract(&mut self, vx: u8, vy: u8) -> Result<()> {
        let x = self.registers[vx as usize];
        let y = self.registers[vy as usize];

        self.registers[0xF] = (x >= y) as u8;
        self.registers[vx as usize] = x.wrapping_sub(y);
        Ok(())
    }

    pub(super) fn op_subtract_negate(&mut self, vx: u8, vy: u8) -> Result<()> {
        let x = self.registers[vx as usize];
        let y = self.registers[vy as usize];

        self.registers[0xF] = (y >= x) as u8;
        self.registers[vx as usize] = y.wrapping_sub(x);
        Ok(())
    }

    pub(super) fn op_or(&mut self, vx: u8, vy: u8) -> Result<()> {
        self.registers[vx as usize] |= self.registers[vy as usize];
        Ok(())
    }

    pub(super) fn op_and(&mut self, vx: u8, vy: u8) -> Result<()> {
        self.registers[vx as usize] &= self.registers[vy as usize];
        Ok(())
    }

    pub(super) fn op_xor(&mut self, vx: u8, vy: u8) -> Result<()> {
        self.registers[vx as usize] ^= self.registers[vy as usize];
        Ok(())
    }

    pub(super) fn op_shift_right(&mut self, vx: u8, vy: u8) -> Result<()> {
        if self.quircks.shift {
            self.registers[0xF] = self.registers[vx as usize] & 0x01;
            self.registers[vx as usize] >>= 1;
        } else {
            self.registers[0xF] = self.registers[vy as usize] & 0x01;
            self.registers[vx as usize] = self.registers[vy as usize] >> 1;
        }

        Ok(())
    }

    pub(super) fn op_shift_left(&mut self, vx: u8, vy: u8) -> Result<()> {
        if self.quircks.shift {
            self.registers[0xF] = (self.registers[vx as usize] & 0x80) >> 7;
            self.registers[vx as usize] <<= 1;
        } else {
            self.registers[0xF] = (self.registers[vy as usize] & 0x80) >> 7;
            self.registers[vx as usize] = self.registers[vy as usize] << 1;
        }

        Ok(())
    }
}
