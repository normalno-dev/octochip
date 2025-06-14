use super::Instruction;

impl Instruction {
    pub fn encode(&self) -> u16 {
        use Instruction::*;
        use Opcode::*;

        match *self {
            Clear => QQQQ(0x00E0),
            Return => QNNN(0x0, 0x0EE),
            Syscall(nnn) => QNNN(0, nnn),
            Jump(nnn) => QNNN(0x1, nnn),
            Call(nnn) => QNNN(0x2, nnn),
            SkipIfEqualImm { vx, kk } => QXKK(0x3, vx, kk),
            SkipIfNotEqualImm { vx, kk } => QXKK(0x4, vx, kk),
            SkipIfEqual { vx, vy } => QXYW(0x5, vx, vy, 0x0),
            SetImmediate { vx, kk } => QXKK(0x6, vx, kk),
            AddImmediate { vx, kk } => QXKK(0x7, vx, kk),
            Set { vx, vy } => QXYW(0x8, vx, vy, 0x0),
            Or { vx, vy } => QXYW(0x8, vx, vy, 0x1),
            And { vx, vy } => QXYW(0x8, vx, vy, 0x2),
            Xor { vx, vy } => QXYW(0x8, vx, vy, 0x3),
            Add { vx, vy } => QXYW(0x8, vx, vy, 0x4),
            Subtract { vx, vy } => QXYW(0x8, vx, vy, 0x5),
            ShiftRight { vx, vy } => QXYW(0x8, vx, vy, 0x6),
            SubtractNegate { vx, vy } => QXYW(0x8, vx, vy, 0x7),
            ShiftLeft { vx, vy } => QXYW(0x8, vx, vy, 0xE),
            SkipIfNotEqual { vx, vy } => QXYW(0x9, vx, vy, 0x0),
            SetIndex(nnn) => QNNN(0xA, nnn),
            JumpOffset(nnn) => QNNN(0xB, nnn),
            Rnd { vx, kk } => QXKK(0xC, vx, kk),
            Draw {
                vx: x,
                vy: y,
                n: len,
            } => QXYW(0xD, x, y, len),
            SkipIfKey(vx) => QXKK(0xE, vx, 0x9E),
            SkipIfNotKey(vx) => QXKK(0xE, vx, 0xA1),
            LoadDelayTimer(vx) => QXKK(0xF, vx, 0x07),
            WaitForKey(vx) => QXKK(0xF, vx, 0x0A),
            SetDelayTimer(vx) => QXKK(0xF, vx, 0x15),
            SetSoundTimer(vx) => QXKK(0xF, vx, 0x18),
            AddIndex(vx) => QXKK(0xF, vx, 0x1E),
            LoadFont(vx) => QXKK(0xF, vx, 0x29),
            StoreBcd(vx) => QXKK(0xF, vx, 0x33),
            StoreRegisters(x) => QXKK(0xF, x, 0x55),
            LoadRegisters(x) => QXKK(0xF, x, 0x65),
        }
        .into()
    }
}

enum Opcode {
    QQQQ(u16),            // covers 00E0: 2 bytes const
    QNNN(u8, u16),        // covers 1NNN: const, dyn NNN
    QXKK(u8, u8, u8),     // covers 3XKK: const, dyn X, (dyn KK or 1 byte const)
    QXYW(u8, u8, u8, u8), // covers 5XY0: const, dyn X, dyn Y, dyn N or 1 nibble const
}

impl From<Opcode> for u16 {
    fn from(value: Opcode) -> Self {
        use Opcode::*;

        match value {
            QQQQ(op) => op,
            QNNN(q, nnn) => {
                let (q, nnn) = (q as u16, nnn & 0xFFF);
                (q << 12) | (nnn & 0xFFF)
            }
            QXKK(q, x, kk) => {
                let (q, x, kk) = (q as u16, x as u16, kk as u16);
                (q << 12) | (x << 8) | kk
            }
            QXYW(q, x, y, w) => {
                let (q, x, y, w) = (q as u16, x as u16, y as u16, w as u16);
                (q << 12) | (x << 8) | (y << 4) | w
            }
        }
    }
}

impl From<Instruction> for u16 {
    fn from(inst: Instruction) -> Self {
        inst.encode()
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    #[test]
    fn test_encode() {
        use super::Instruction::*;

        let table = HashMap::from([
            (0x00E0, Clear),
            (0x00EE, Return),
            (0x0ABC, Syscall(0xABC)),
            (0x1ABC, Jump(0xABC)),
            (0x2ABC, Call(0xABC)),
            (0x3ABC, SkipIfEqualImm { vx: 0xA, kk: 0xBC }),
            (0x4ABC, SkipIfNotEqualImm { vx: 0xA, kk: 0xBC }),
            (0x5AB0, SkipIfEqual { vx: 0xA, vy: 0xB }),
            (0x6ABC, SetImmediate { vx: 0xA, kk: 0xBC }),
            (0x7ABC, AddImmediate { vx: 0xA, kk: 0xBC }),
            (0x8AB0, Set { vx: 0xA, vy: 0xB }),
            (0x8AB1, Or { vx: 0xA, vy: 0xB }),
            (0x8AB2, And { vx: 0xA, vy: 0xB }),
            (0x8AB3, Xor { vx: 0xA, vy: 0xB }),
            (0x8AB4, Add { vx: 0xA, vy: 0xB }),
            (0x8AB5, Subtract { vx: 0xA, vy: 0xB }),
            (0x8AB6, ShiftRight { vx: 0xA, vy: 0xB }),
            (0x8AB7, SubtractNegate { vx: 0xA, vy: 0xB }),
            (0x8ABE, ShiftLeft { vx: 0xA, vy: 0xB }),
            (0x9AB0, SkipIfNotEqual { vx: 0xA, vy: 0xB }),
            (0xAABC, SetIndex(0xABC)),
            (0xBABC, JumpOffset(0xABC)),
            (0xCABC, Rnd { vx: 0xA, kk: 0xBC }),
            (
                0xDABC,
                Draw {
                    vx: 0xA,
                    vy: 0xB,
                    n: 0xC,
                },
            ),
            (0xEA9E, SkipIfKey(0xA)),
            (0xEAA1, SkipIfNotKey(0xA)),
            (0xFA07, LoadDelayTimer(0xA)),
            (0xFA0A, WaitForKey(0xA)),
            (0xFA15, SetDelayTimer(0xA)),
            (0xFA18, SetSoundTimer(0xA)),
            (0xFA1E, AddIndex(0xA)),
            (0xFA29, LoadFont(0xA)),
            (0xFA33, StoreBcd(0xA)),
            (0xFA55, StoreRegisters(0xA)),
            (0xFA65, LoadRegisters(0xA)),
        ]);

        for (want, inst) in table.iter() {
            let got = inst.encode();
            assert_eq!(
                *want, got,
                "failed to encode: want: 0x{:04X}, got: 0x{:04X}",
                want, got,
            )
        }
    }
}
