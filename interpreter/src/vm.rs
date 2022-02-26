use crate::instruction::Instruction;
pub struct State {
    memory: [u8; 256],
    stack: Vec<u8>,
    pc: usize,
    instructions: Vec<Instruction>,
}

pub enum Error {
    StackUnderflow,
}

impl State {
    pub fn init(instructions: Vec<Instruction>) -> Self {
        State {
            memory: [0; 256],
            stack: Vec::new(),
            pc: 0,
            instructions,
        }
    }

    pub fn execute(&mut self, instruction: Instruction) -> Result<(), Error> {
        match instruction {
            Instruction::PUSH => todo!(),
            Instruction::POP => {
                self.stack.pop().ok_or(Error::StackUnderflow)?;
            }
            Instruction::INC => todo!(),
            Instruction::DEC => todo!(),
            Instruction::JNZ => todo!(),
            Instruction::JZ => todo!(),
            Instruction::READ => todo!(),
            Instruction::WRITE => todo!(),
            Instruction::STORE => todo!(),
            Instruction::LOAD => todo!(),
        };
        Ok(())
    }
}
