use super::error::Error;

type Result<T> = std::result::Result<T, Error>;

mod decoder;
mod encoder;

// NNN for address
// KK for immediate
// X,Y for registers
// Each letter represents 4 bits: 1NNN is 0001 plus 12 address bits
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Instruction {
    Clear,        // 00E0
    Return,       // 00EE
    Syscall(u16), // 0NNN

    Jump(u16),                            // 1NNN
    Call(u16),                            // 2NNN
    SkipIfEqualImm { vx: u8, kk: u8 },    // 3XKK: skip next if Vx == KK
    SkipIfNotEqualImm { vx: u8, kk: u8 }, // 4XKK: skip next if Vx != KK
    SkipIfEqual { vx: u8, vy: u8 },       // 5XY0: skip next if Vx == Y

    SetImmediate { vx: u8, kk: u8 }, // 6XKK: Vx = KK
    AddImmediate { vx: u8, kk: u8 }, // 7XKK: Vx += KK

    Set { vx: u8, vy: u8 },            // 8XY0: Vx = Vy
    Or { vx: u8, vy: u8 },             // 8XY1: Vx |= Vy
    And { vx: u8, vy: u8 },            // 8XY2: Vx &= Vy
    Xor { vx: u8, vy: u8 },            // 8XY3: Vx ^= Vy
    Add { vx: u8, vy: u8 },            // 8XY4: Vx += Vy, Vf = carry
    Subtract { vx: u8, vy: u8 },       // 8XY5: Vf = (Vx >= Vy) ? 1 : 0, Vx = Vx - Vy
    ShiftRight { vx: u8, vy: u8 },     // 8XY6: Vf = LBS, Vx = Vy >> 1
    SubtractNegate { vx: u8, vy: u8 }, // 8XY7: Vf = (Vy >= Vx) ? 1 : 0, Vx = Vy - Vx
    ShiftLeft { vx: u8, vy: u8 },      // 8XYE: Vf = HBS, Vx = Vy << 1

    SkipIfNotEqual { vx: u8, vy: u8 }, // 9XY0: skip if Vx != Vy

    SetIndex(u16),   // ANNN: I = NNN
    JumpOffset(u16), // BNNN: pc = v0 + NNN

    Rnd { vx: u8, kk: u8 },         // CXKK: Vx = rand(0,255) & KK
    Draw { vx: u8, vy: u8, n: u8 }, // DXYN: Draw mem[i:i+n] at (x, y)
    SkipIfKey(u8),                  // EX9E skip next if keyPressed(Vx)
    SkipIfNotKey(u8),               // EXA1 skip next if !keyPressed(Vx)
    LoadDelayTimer(u8),             // FX07: Vx = delay_time
    WaitForKey(u8),                 // FX0A: Vx = keyPressed()
    SetDelayTimer(u8),              // FX15: delay_timer = Vx
    SetSoundTimer(u8),              // FX18: sound_timer = Vx
    AddIndex(u8),                   // FX1E: I = I + Vx
    LoadFont(u8),                   // FX29: I = font_addresses_for_digit(Vx)
    StoreBcd(u8), // FX33: mem[I] = Vx / 100, mem[I+1] = (Vx / 10) % 10, mem[I+2] = Vx % 10
    StoreRegisters(u8), // FX55: mem[I] = v0, mem[I+1] = v1, ..., mem[I+n] = Vx
    LoadRegisters(u8), // FX65: v0 = mem[I], v1 = mem[I+1], ..., Vx = mem[I+n]
}
