use std::time::Duration;

use crate::display::Display;
use crate::instruction::Instruction;
use crate::keyboard::Keyboard;
use crate::platform::{ExecutionMode, Platform};
use crate::{error::Error, memory::Memory};
use rand::rngs::SmallRng;
use rand::{Rng, RngCore, SeedableRng};

type Result<T> = std::result::Result<T, Error>;

pub mod config;
pub mod quircks;

mod ops_alu;
mod ops_control;
mod ops_io;
mod ops_memory;
mod ops_register;
mod ops_system;

mod debug;

pub struct Machine {
    memory: Memory,
    display: Display,
    config: config::Config,

    registers: [u8; 16], // V0 to FF registers
    stack: [u16; 16],
    pc: u16,    // program counter register
    sp: u8,     // stack counter register
    dt: u8,     // delay timer register
    st: u8,     // sound timer register
    index: u16, // index register (I)

    keys: Keyboard,
    rng: SmallRng,

    last_frame_time: Duration,
    timer_period: Duration,
    timer_accumulator: Duration,
}

impl Machine {
    pub fn new() -> Self {
        let cfg = config::Config::default();
        Self {
            memory: Memory::new(),
            display: Display::new(),
            config: config::Config::default(),
            keys: Keyboard::new(),

            registers: [0; 16],
            stack: [0; 16],
            pc: 0,
            sp: 0,
            dt: 0,
            st: 0,
            index: 0,

            rng: SmallRng::from_rng(&mut rand::rng()),
            last_frame_time: Duration::new(0, 0),
            timer_period: Duration::from_millis(1000 / cfg.timer_frequency as u64),
            timer_accumulator: Duration::new(0, 0),
        }
    }

    // with_seed creates Machine and sets a random generator seed
    // for predictable and determenistic testing
    pub fn with_seed(seed: u64) -> Self {
        let mut machine = Self::new();
        machine.rng = SmallRng::seed_from_u64(seed);
        machine
    }

    pub fn with_config(cfg: config::Config) -> Self {
        let mut machine = Self::new();
        machine.timer_period = Duration::from_millis(1000 / cfg.timer_frequency as u64);
        machine.config = cfg;
        machine
    }

    // reset display buffer, memory, keyboard input, registers, stack, timers,
    // index register. it does not reset random generator.
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
        self.timer_accumulator = Duration::new(0, 0);
        self.last_frame_time = Duration::new(0, 0);
    }

    pub fn run_frame<P: Platform>(
        &mut self,
        platform: &mut P,
    ) -> std::result::Result<bool, P::Error> {
        let mode = platform.get_execution_mode();
        let frame_start = platform.get_time();

        self.keys = platform.get_keys();

        let instructions_to_run = match mode {
            ExecutionMode::Paused => 0,
            ExecutionMode::Step => 1,
            ExecutionMode::Running => self.calculate_instructions_for_frame(frame_start),
        };

        for _ in 0..instructions_to_run {
            if !self.step()? {
                return Ok(false);
            }
        }

        if matches!(mode, ExecutionMode::Running) {
            let delta = platform.get_time() - frame_start;
            self.update_timers(delta);
        }

        platform.draw_display(&self.display)?;
        platform.play_sound(self.st > 0)?;

        Ok(true)
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

    fn calculate_instructions_for_frame(&mut self, current_time: Duration) -> u32 {
        let delta = current_time - self.last_frame_time;
        let expected_instructions = self.config.cpu_frequency as f64 * delta.as_secs_f64();

        self.last_frame_time = current_time;
        expected_instructions.round() as u32
    }

    fn update_timers(&mut self, delta: Duration) {
        self.timer_accumulator += delta;

        if self.timer_accumulator < self.timer_period {
            return;
        }

        if self.dt > 0 {
            self.dt -= 1;
        }

        if self.st > 0 {
            self.st -= 1;
        }

        self.timer_accumulator -= self.timer_period;
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
            // system operations
            Clear => self.op_clear(),
            Syscall(addr) => self.op_syscall(addr),
            Rnd { vx, kk } => self.op_rnd(vx, kk),
            SetDelayTimer(vx) => self.op_set_delay_timer(vx),
            SetSoundTimer(vx) => self.op_set_sound_timer(vx),
            LoadDelayTimer(vx) => self.op_load_delay_timer(vx),

            // flow control operations
            Jump(addr) => self.op_jump(addr),
            JumpOffset(nnn) => self.op_jump_offset(nnn),
            Call(addr) => self.op_call(addr),
            Return => self.op_return(),

            // branch operations
            SkipIfEqualImm { vx, kk } => self.op_skip_if_equal_imm(vx, kk),
            SkipIfNotEqualImm { vx, kk } => self.op_skip_if_not_equal_imm(vx, kk),
            SkipIfEqual { vx, vy } => self.op_skip_if_equal(vx, vy),
            SkipIfNotEqual { vx, vy } => self.op_skip_if_not_equal(vx, vy),

            // register operations
            SetImmediate { vx, kk } => self.op_set_immediate(vx, kk),
            Set { vx, vy } => self.op_set(vx, vy),
            SetIndex(addr) => self.op_set_index(addr),
            AddIndex(x) => self.op_add_index(x),

            // ALU operaitions
            AddImmediate { vx, kk } => self.op_add_immediate(vx, kk),
            Or { vx, vy } => self.op_or(vx, vy),
            And { vx, vy } => self.op_and(vx, vy),
            Xor { vx, vy } => self.op_xor(vx, vy),
            Add { vx, vy } => self.op_add(vx, vy),
            Subtract { vx, vy } => self.op_subtract(vx, vy),
            SubtractNegate { vx, vy } => self.op_subtract_negate(vx, vy),
            ShiftRight { vx, vy } => self.op_shift_right(vx, vy),
            ShiftLeft { vx, vy } => self.op_shift_left(vx, vy),

            // IO operaitions
            SkipIfKey(vx) => self.op_skip_if_key(vx),
            SkipIfNotKey(vx) => self.op_skip_if_not_key(vx),
            WaitForKey(vx) => self.op_wait_for_key(vx),
            Draw { vx, vy, n } => self.op_draw(vx, vy, n),

            // memory operations
            StoreBcd(vx) => self.op_store_bcd(vx),
            StoreRegisters(x) => self.op_store_registers(x),
            LoadRegisters(x) => self.op_load_registers(x),
            LoadFont(vx) => self.op_load_font(vx),
        }
    }
}
