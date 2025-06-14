use super::error::Error;
type Result<T> = std::result::Result<T, Error>;

const FONTS: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

pub struct Memory {
    data: [u8; 4096],
}

impl Memory {
    pub fn new() -> Self {
        let mut memory = Self { data: [0; 4096] };
        memory.load_fonts();

        return memory;
    }

    pub fn read(&self, addr: u16) -> Result<u8> {
        if addr >= 0x1000 {
            Err(Error::MemoryOutOfBound)
        } else {
            Ok(self.data[addr as usize])
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) -> Result<()> {
        if addr >= 0x1000 {
            Err(Error::MemoryOutOfBound)
        } else {
            self.data[addr as usize] = value;
            Ok(())
        }
    }
    pub fn write_word(&mut self, addr: u16, value: u16) -> Result<()> {
        if addr >= 0x1000 - 1 {
            return Err(Error::MemoryOutOfBound);
        }

        let high = (value >> 8) as u8;
        let low = (value & 0xFF) as u8;

        self.write(addr, high)?;
        self.write(addr + 1, low)?;
        Ok(())
    }

    pub fn read_range(&self, start: u16, length: u16) -> Vec<u8> {
        let start_idx = start as usize;
        let end = start as u32 + length as u32;

        if end > 0x1000 {
            Vec::new()
        } else {
            let end_idx = (start + length) as usize;
            self.data[start_idx..end_idx].to_vec()
        }
    }

    pub fn read_word(&self, addr: u16) -> Result<u16> {
        if addr >= 0x1000 - 1 {
            return Err(Error::MemoryOutOfBound);
        }

        let high = self.data[addr as usize] as u16;
        let low = self.data[addr as usize + 1] as u16;

        let result = high << 8 | low;
        Ok(result)
    }

    fn load_fonts(&mut self) {
        let addr = 0x050;
        for (i, byte) in FONTS.iter().copied().enumerate() {
            let target = addr + i;
            self.data[target as usize] = byte;
        }
    }
}
