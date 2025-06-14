use super::Machine;
use crate::error::Error;

type Result<T> = std::result::Result<T, Error>;

impl Machine {
    pub(super) fn op_set_immediate(&mut self, vx: u8, kk: u8) -> Result<()> {
        self.registers[vx as usize] = kk;
        Ok(())
    }

    pub(super) fn op_set(&mut self, vx: u8, vy: u8) -> Result<()> {
        self.registers[vx as usize] = self.registers[vy as usize];
        Ok(())
    }

    pub(super) fn op_set_index(&mut self, addr: u16) -> Result<()> {
        if addr < 0x200 {
            return Err(Error::InvalidIndexAddress(addr));
        } else if addr >= 0x1000 {
            return Err(Error::IndexOverflow(addr));
        }

        self.index = addr;
        Ok(())
    }

    pub(super) fn op_add_index(&mut self, vx: u8) -> Result<()> {
        let offset = self.registers[vx as usize] as u16;
        let target = offset + self.index;

        if target < 0x200 {
            return Err(Error::InvalidIndexAddress(target));
        } else if target >= 0x1000 {
            return Err(Error::IndexOverflow(target));
        }

        self.index += offset as u16;
        Ok(())
    }
}
