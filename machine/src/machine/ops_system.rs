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
}
