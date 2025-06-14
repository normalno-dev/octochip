use super::Machine;
use crate::error::Error;

type Result<T> = std::result::Result<T, Error>;

impl Machine {
    pub(super) fn op_load_font(&mut self, vx: u8) -> Result<()> {
        let digit = self.registers[vx as usize];
        self.index = 0x50 + (digit as u16 * 5);
        Ok(())
    }
}
