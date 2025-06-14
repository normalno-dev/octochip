use crate::instruction::Instruction;

pub struct Program(pub Vec<Instruction>);

impl From<Program> for Vec<u16> {
    fn from(value: Program) -> Self {
        value.0.into_iter().map(|inst| inst.into()).collect()
    }
}

impl Program {
    pub fn dump(&self) -> String {
        let mut output = String::new();

        for (addr, inst) in self.0.iter().copied().enumerate() {
            let opcode: u16 = inst.into();

            output.push_str(&format!("0x{:04X}:\t0x{:04X}\n", addr, opcode));
        }

        output
    }
}
