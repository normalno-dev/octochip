mod display;
mod error;
mod instruction;
mod keyboard;
mod machine;
mod memory;
mod program;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instruction::Instruction;

    #[test]
    fn it_works() {
        let program = program::Program(vec![
            Instruction::SetImmediate { vx: 0, kk: 0x1 },
            Instruction::SetImmediate { vx: 1, kk: 0x1 },
            Instruction::LoadFont(0),
            Instruction::Draw { vx: 0, vy: 1, n: 5 },
        ]);

        let mut machine = machine::Machine::new();
        machine.load_program(program.into()).unwrap();

        // run all instructions
        for _ in 0..4 {
            machine.step().unwrap();
        }

        let screen = machine.dump_screen();
        println!("{}", screen);

        let registers = machine.get_registers();
        assert_eq!(
            [
                0x1, 0x1, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0
            ],
            registers
        );
    }
}
