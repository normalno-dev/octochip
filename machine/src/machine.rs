use crate::display::Display;
use crate::instruction::Instruction;
use crate::keyboard::Keyboard;
use crate::machine::quircks::Quircks;
use crate::{error::Error, memory::Memory};

type Result<T> = std::result::Result<T, Error>;

mod ops_alu;
mod ops_control;
mod ops_io;
mod ops_memory;
mod ops_register;
mod ops_system;
mod quircks;

mod debug;

pub struct Machine {
    memory: Memory,
    display: Display,
    quircks: Quircks,

    registers: [u8; 16], // V0 to FF registers
    stack: [u16; 16],
    pc: u16,    // program counter register
    sp: u8,     // stack counter register
    dt: u8,     // delay timer register
    st: u8,     // sound timer register
    index: u16, // index register (I)

    keys: Keyboard,
}

impl Machine {
    pub fn new() -> Self {
        Self {
            memory: Memory::new(),
            display: Display::new(),
            quircks: Quircks::default(),
            registers: [0; 16],
            stack: [0; 16],
            pc: 0,
            sp: 0,
            dt: 0,
            st: 0,
            index: 0,
            keys: Keyboard::new(),
        }
    }

    pub fn reset(&mut self) {
        self.memory = Memory::new();
        self.keys.clear_all_keys();
        self.display.clear();
        self.registers = [0; 16];
        self.stack = [0; 16];
        self.dt = 0;
        self.st = 0;
        self.pc = 0;
        self.sp = 0;
        self.index = 0;
    }

    pub fn step(&mut self) -> Result<bool> {
        if self.pc < 0x200 || self.pc >= 0x1000 - 2 {
            return Err(Error::InvalidProgramCounter(self.pc));
        }

        if self.pc % 2 != 0 {
            return Err(Error::UnalignedProgramCounter(self.pc));
        }

        let word = self.memory.read_word(self.pc)?;
        self.pc += 2;

        let instruction = Instruction::decode(word)?;
        self.exec(instruction)?;

        Ok(true)
    }

    // resets CPU state and load program into memory
    pub fn load_program(&mut self, program: Vec<u16>) -> Result<()> {
        self.reset();

        let start_addr = 0x200;
        self.pc = start_addr;

        for (i, &word) in program.iter().enumerate() {
            let addr = start_addr + (i * 2) as u16;
            self.memory.write_word(addr, word)?;
        }
        Ok(())
    }
}

impl Machine {
    pub fn get_registers(&self) -> &[u8] {
        &self.registers
    }

    pub fn get_pc(&self) -> u16 {
        self.pc
    }

    pub fn get_sp(&self) -> u8 {
        self.sp
    }

    pub fn get_memory(&self) -> &Memory {
        &self.memory
    }

    pub fn get_display(&self) -> &Display {
        &self.display
    }

    pub fn get_delay_timer(&self) -> u8 {
        self.dt
    }

    pub fn get_sound_timer(&self) -> u8 {
        self.st
    }

    pub fn get_index(&self) -> u16 {
        self.index
    }
}

impl Machine {
    fn exec(&mut self, instruction: Instruction) -> Result<()> {
        use Instruction::*;

        match instruction {
            Clear => self.op_clear(),
            Return => self.op_return(),
            Syscall(addr) => self.op_syscall(addr),

            Jump(addr) => self.op_jump(addr),
            Call(addr) => self.op_call(addr),
            SkipIfEqualImm { vx, kk } => self.op_skip_if_equal_imm(vx, kk),
            SkipIfNotEqualImm { vx, kk } => self.op_skip_if_not_equal_imm(vx, kk),
            SkipIfEqual { vx, vy } => self.op_skip_if_equal(vx, vy),
            SkipIfNotEqual { vx, vy } => self.op_skip_if_not_equal(vx, vy),

            SetImmediate { vx, kk } => self.op_set_immediate(vx, kk),
            Set { vx, vy } => self.op_set(vx, vy),
            SetIndex(addr) => self.op_set_index(addr),
            AddIndex(x) => self.op_add_index(x),

            AddImmediate { vx, kk } => self.op_add_immediate(vx, kk),
            Or { vx, vy } => self.op_or(vx, vy),
            And { vx, vy } => self.op_and(vx, vy),
            Xor { vx, vy } => self.op_xor(vx, vy),
            Add { vx, vy } => self.op_add(vx, vy),
            Subtract { vx, vy } => self.op_subtract(vx, vy),
            SubtractNegate { vx, vy } => self.op_subtract_negate(vx, vy),
            ShiftRight { vx, vy } => self.op_shift_right(vx, vy),
            ShiftLeft { vx, vy } => self.op_shift_left(vx, vy),

            LoadFont(vx) => self.op_load_font(vx),
            Draw { vx, vy, n } => self.op_draw(vx, vy, n),

            _ => return Err(Error::NotImplementedYet(instruction)),
        }
    }
}
