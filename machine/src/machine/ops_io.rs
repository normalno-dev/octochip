use super::Machine;
use crate::error::Error;

type Result<T> = std::result::Result<T, Error>;

impl Machine {
    pub(super) fn op_skip_if_key(&mut self, vx: u8) -> Result<()> {
        let key = self.registers[vx as usize];
        if self.keys.is_key_pressed(key) {
            self.pc += 2;
        }

        Ok(())
    }

    pub(super) fn op_skip_if_not_key(&mut self, vx: u8) -> Result<()> {
        let key = self.registers[vx as usize];
        if !self.keys.is_key_pressed(key) {
            self.pc += 2;
        }

        Ok(())
    }

    pub(super) fn op_wait_for_key(&mut self, vx: u8) -> Result<()> {
        if let Some(key) = self.keys.get_first_pressed_key() {
            self.registers[vx as usize] = key;
            return Ok(());
        }

        // decrement PC back to the current one,
        // to repeat this instruction on the next iteration
        self.pc -= 2;

        Ok(())
    }

    pub(super) fn op_draw(&mut self, vx: u8, vy: u8, n: u8) -> Result<()> {
        let sprite = self.memory.read_range(self.index, n as u16);
        let x = self.registers[vx as usize];
        let y = self.registers[vy as usize];

        let collision = self.display.draw_sprite(x, y, &sprite);
        self.registers[0xF] = collision as u8;
        Ok(())
    }
}
