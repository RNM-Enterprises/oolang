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

    pub fn execute(&mut self) -> Result<(), Error> {
        match self.instructions[self.pc] {
            Instruction::PUSH => todo!(),
            Instruction::POP => {
                self.stack.pop().ok_or(Error::StackUnderflow)?;
                self.pc += 1;
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

    pub fn stack_top(&self) -> Option<u8> {
        self.stack.last().map(|i| i.to_owned())
    }
}
