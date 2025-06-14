use super::Machine;
use crate::error::Error;

type Result<T> = std::result::Result<T, Error>;

impl Machine {
    pub(super) fn op_return(&mut self) -> Result<()> {
        if self.sp == 0 {
            return Err(Error::StackUnderflow);
        }

        self.pc = self.stack[self.sp as usize];
        self.sp -= 1;

        Ok(())
    }

    pub(super) fn op_call(&mut self, addr: u16) -> Result<()> {
        if self.sp == 0xF {
            return Err(Error::StackOverflow);
        }

        self.stack[self.sp as usize] = self.pc;
        self.sp += 1;
        self.pc = addr;

        Ok(())
    }

    pub(super) fn op_jump(&mut self, addr: u16) -> Result<()> {
        self.pc = addr;
        Ok(())
    }

    pub(super) fn op_jump_offset(&mut self, offset: u16) -> Result<()> {
        let target = offset + self.registers[0] as u16;
        self.pc = target;
        Ok(())
    }

    pub(super) fn op_skip_if_equal_imm(&mut self, vx: u8, kk: u8) -> Result<()> {
        if self.registers[vx as usize] == kk {
            self.pc += 2;
        }

        Ok(())
    }

    pub(super) fn op_skip_if_not_equal_imm(&mut self, vx: u8, kk: u8) -> Result<()> {
        if self.registers[vx as usize] != kk {
            self.pc += 2;
        }
        Ok(())
    }

    pub(super) fn op_skip_if_equal(&mut self, vx: u8, vy: u8) -> Result<()> {
        if self.registers[vx as usize] == self.registers[vy as usize] {
            self.pc += 2;
        }

        Ok(())
    }

    pub(super) fn op_skip_if_not_equal(&mut self, vx: u8, vy: u8) -> Result<()> {
        if self.registers[vx as usize] != self.registers[vy as usize] {
            self.pc += 2;
        }

        Ok(())
    }
}
