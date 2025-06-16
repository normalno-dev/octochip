use rand::RngCore;

use super::Machine;
use crate::error::Error;

type Result<T> = std::result::Result<T, Error>;

// system instructions
impl Machine {
    pub(super) fn op_clear(&mut self) -> Result<()> {
        self.display.clear();

        Ok(())
    }

    pub(super) fn op_syscall(&self, _: u16) -> Result<()> {
        Ok(())
    }

    pub(super) fn op_rnd(&mut self, vx: u8, kk: u8) -> Result<()> {
        let value = (self.rng.next_u32() & 0xFF) as u8;
        self.registers[vx as usize] = value & kk;

        Ok(())
    }

    pub(super) fn op_set_delay_timer(&mut self, vx: u8) -> Result<()> {
        let value = self.registers[vx as usize];
        self.dt = value;

        Ok(())
    }

    pub(super) fn op_set_sound_timer(&mut self, vx: u8) -> Result<()> {
        let value = self.registers[vx as usize];
        self.st = value;

        Ok(())
    }

    pub(super) fn op_load_delay_timer(&mut self, vx: u8) -> Result<()> {
        self.registers[vx as usize] = self.dt;

        Ok(())
    }
}
