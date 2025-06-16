use std::iter;

use super::Machine;
use crate::error::Error;

type Result<T> = std::result::Result<T, Error>;

impl Machine {
    pub(super) fn op_load_font(&mut self, vx: u8) -> Result<()> {
        let digit = self.registers[vx as usize];
        self.index = 0x50 + (digit as u16 * 5);
        Ok(())
    }

    pub(super) fn op_store_bcd(&mut self, vx: u8) -> Result<()> {
        let value = self.registers[vx as usize];
        let hundreds = value / 100;
        let tens = value / 10 % 10;
        let ones = value % 10;

        self.memory.write(self.index, hundreds)?;
        self.memory.write(self.index + 1, tens)?;
        self.memory.write(self.index + 2, ones)?;

        Ok(())
    }

    pub(super) fn op_store_registers(&mut self, x: u8) -> Result<()> {
        for i in 0..=x {
            let value = self.registers[i as usize];
            let addr = self.index + i as u16;
            self.memory.write(addr, value)?;
        }

        Ok(())
    }

    pub(super) fn op_load_registers(&mut self, x: u8) -> Result<()> {
        for i in 0..=x {
            let addr = self.index + i as u16;
            let value = self.memory.read(addr)?;
            self.registers[i as usize] = value;
        }

        Ok(())
    }
}
