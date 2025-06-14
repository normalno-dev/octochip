use super::Instruction;
use crate::error::Error;

impl Instruction {
    pub fn decode(word: u16) -> Result<Self, Error> {
        let nibbles = nibbles(word);
        let (_, vx, vy, n) = nibbles;
        let kk = (word & 0x00FF) as u8;
        let nnn = word & 0x0FFF;

        use Instruction::*;

        let inst = match nibbles {
            (0x0, 0x0, 0xE, 0x0) => Clear,
            (0x0, 0x0, 0xE, 0xE) => Return,
            (0x0, _, _, _) => Syscall(nnn),

            (0x1, _, _, _) => Jump(nnn),
            (0x2, _, _, _) => Call(nnn),
            (0x3, _, _, _) => SkipIfEqualImm { vx, kk },
            (0x4, _, _, _) => SkipIfNotEqualImm { vx, kk },
            (0xA, _, _, _) => SetIndex(nnn),
            (0xB, _, _, _) => JumpOffset(nnn),
            (0xC, _, _, _) => Rnd { vx, kk },
            (0xD, _, _, _) => Draw { vx, vy, n },
            (0x6, _, _, _) => SetImmediate { vx, kk },
            (0x7, _, _, _) => AddImmediate { vx, kk },

            (0x5, _, _, 0x0) => SkipIfEqual { vx, vy },
            (0x8, _, _, 0x0) => Set { vx, vy },
            (0x8, _, _, 0x1) => Or { vx, vy },
            (0x8, _, _, 0x2) => And { vx, vy },
            (0x8, _, _, 0x3) => Xor { vx, vy },
            (0x8, _, _, 0x4) => Add { vx, vy },
            (0x8, _, _, 0x5) => Subtract { vx, vy },
            (0x8, _, _, 0x6) => ShiftRight { vx, vy },
            (0x8, _, _, 0x7) => SubtractNegate { vx, vy },
            (0x8, _, _, 0xE) => ShiftLeft { vx, vy },
            (0x9, _, _, 0x0) => SkipIfNotEqual { vx, vy },

            (0xE, _, 0x9, 0xE) => SkipIfKey(vx),
            (0xE, _, 0xA, 0x1) => SkipIfNotKey(vx),
            (0xF, _, 0x0, 0x7) => LoadDelayTimer(vx),
            (0xF, _, 0x0, 0xA) => WaitForKey(vx),
            (0xF, _, 0x1, 0x5) => SetDelayTimer(vx),
            (0xF, _, 0x1, 0x8) => SetSoundTimer(vx),
            (0xF, _, 0x1, 0xE) => AddIndex(vx),
            (0xF, _, 0x2, 0x9) => LoadFont(vx),
            (0xF, _, 0x3, 0x3) => StoreBcd(vx),
            (0xF, _, 0x5, 0x5) => StoreRegisters(vx),
            (0xF, _, 0x6, 0x5) => LoadRegisters(vx),

            _ => return Err(Error::InvalidInstruction(word)),
        };

        Ok(inst)
    }
}

fn nibbles(word: u16) -> (u8, u8, u8, u8) {
    (
        ((word & 0xF000) >> 12) as u8,
        ((word & 0x0F00) >> 8) as u8,
        ((word & 0x00F0) >> 4) as u8,
        (word & 0x000F) as u8,
    )
}

#[cfg(test)]
mod nibbles {
    use std::collections::HashMap;

    use crate::instruction::Instruction;

    #[test]
    fn test_nibbles() {
        use super::nibbles;

        let word = 0xE79E as u16; // EX9E
        let n = nibbles(word);
        assert_eq!(0xE, n.0);
        assert_eq!(0x7, n.1);
        assert_eq!(0x9, n.2);
        assert_eq!(0xE, n.3);
    }

    #[test]
    fn test_decode() {
        use Instruction::*;

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

        for (opcode, want) in table.iter() {
            let got = Instruction::decode(*opcode).unwrap();
            assert_eq!(*want, got, "failed to decode opcode 0x{:04X}", *opcode)
        }
    }
}
