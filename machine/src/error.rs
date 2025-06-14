use crate::instruction::Instruction;

#[derive(Debug)]
pub enum Error {
    MemoryOutOfBound,
    InvalidInstruction(u16),
    NotImplementedYet(Instruction),
    StackUnderflow,
    StackOverflow,

    InvalidIndexAddress(u16),
    IndexOverflow(u16),
    InvalidProgramCounter(u16),
    UnalignedProgramCounter(u16),

    InvalidKeyIndex(u8),
}
