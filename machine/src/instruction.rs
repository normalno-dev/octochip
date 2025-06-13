// NNN for address
// KK for immediate
// X,Y for registers
// Each letter represents 4 bits: 1NNN is 0001 plus 12 address bits

type u4 = u8;
type u12 = u16;

pub enum Instruction {
    Clear,        // 00E0
    Return,       // 00EE
    Syscall(u12), // 0NNN

    Jump(u12),                             // 1NNN
    Call(u12),                             // 2NNN
    SkipIfEqualImm { vx: u4, imm: u8 },    // 3XNN: skip next if X == KK
    SkipIfNotEqualImm { vx: u4, imm: u8 }, // 4XKK: skip next if X != KK
    SkipIfEqual { vx: u4, vy: u4 },        // 5XY0: skip next if X == Y

    LoadImmediate { vx: u4, imm: u8 }, // 6XKK: X = KK
    AddImmediate { vx: u4, imm: u8 },  // 7XKK: vx += imm
    Load { vx: u4, vy: u4 },           // 8XY0: vx = vy
    Or { vx: u4, vy: u4 },             // 8XY1: vx |= vy
    And { vx: u4, vy: u4 },            // 8XY2: vx &= vy
    Xor { vx: u4, vy: u4 },            // 8XY3: vx ^= vy
    Add { vx: u4, vy: u4 },            // 8XY4: vx += vy, vf = carry
    Subtract { vx: u4, vy: u4 },       // 8XY5: vf = (vx >= vy) ? 1 : 0, vx = vx - vy
    ShiftRight { vx: u4, vy: u4 },     // 8XY6: vf = LBS, vx = vy >> 1
    SubtractNegate { vx: u4, vy: u4 }, // 8XY7: vf = (vy >= vx) ? 1 : 0, vx = vy - vx
    ShiftLeft { vx: u4, vy: u4 },      // 8XYE: vf = HBS, vx = vy << 1
    SkipIfNotEqual { vx: u4, vy: u4 }, // 9XY0: ski if vx != vy
    LoadI { addr: u12 },               // ANNN: I = NNN
    JumpOffset(u12),                   // BNNN: pc = v0 + NNN
    Rnd { vx: u4, imm: u8 },           // CXKK: vx = rand(0,255) & KK
    Draw { vx: u4, vy: u4, len: u4 },  // DXYN: Draw mem[i:i+n] at (vx, vy)
    SkipIfKey(u4),                     // EX9E skip next if keyPressed(vx)
    SkipIfNotKey(u4),                  // EXA1 skip next if !keyPressed(vx)
    LoadDelayTimer(u4),                // FX07: vx = delay_time
    WaitForKey(u4),                    // FX0A: vx = keyPressed()
    SetDelayTimer(u4),                 // FX15: delay_timer = vx
    SetSoundTimer(u4),                 // FX18: sound_timer = vx
    AddI(u4),                          // FX1E: I = I + vx
    LoadFont(u4),                      // FX29: I = font_addresses_for_digit(vx)
    StoreBcd(u4), // FX33: mem[I] = Vx / 100, mem[I+1] = (vx / 10) % 10, mem[I+2] = vx % 10
    StoreRegisters(u4), // FX55: mem[I] = v0, mem[I+1] = v1, ..., mem[I+n] = vx
    LoadRegisters(u4), // FX65: v0 = mem[I], v1 = mem[I+1], ..., vx = mem[I+n]
}
