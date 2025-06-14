use crate::memory::Memory;

use super::Machine;

impl Machine {
    pub fn dump_registers(&self) -> String {
        let mut output = String::new();

        // print header
        output.push_str(&format!("REG     | HEX    | BIN                | DEC\n"));
        output.push_str(&format!("--------|--------|--------------------|----\n"));

        // print common registers 0x0..=0xF
        for i in (0..=0xF).into_iter() {
            let val = self.registers[i];
            output.push_str(&format!(
                "0x{:X}\t| 0x{:04X} | 0b{:016b} | {}\n",
                i, val, val, val,
            ));
        }

        // print special registers
        let val = self.pc;
        output.push_str(&format!("PC\t| 0x{:04X} | 0b{:016b} | {}\n", val, val, val,));

        // print special registers
        let val = self.sp;
        output.push_str(&format!("SP\t| 0x{:04X} | 0b{:016b} | {}\n", val, val, val,));

        output
    }

    pub fn dump_memory_hex(&self, start: u16, length: u16) -> String {
        let mut output = String::new();
        let data = self.memory.read_range(start, length);

        for (i, byte) in data.iter().enumerate() {
            if i % 16 == 0 {
                output.push_str(&format!("0x{:04X}: ", start + i as u16));
            }

            output.push_str(&format!("0x{:02X} ", byte));
            if i % 16 == 15 {
                output.push('\n');
            }
        }

        output
    }

    pub fn dump_screen(&self) -> String {
        let mut output = String::new();

        for y in 0..self.display.height() {
            for x in 0..self.display.width() {
                if self.display.get_pixel(x, y) {
                    output.push('█');
                } else {
                    output.push('·');
                }
            }
            output.push('\n');
        }

        output
    }
}
