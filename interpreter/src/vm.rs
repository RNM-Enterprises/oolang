use crate::instruction::Instruction;
pub struct State {
    memory: [u8; 256],
    stack: Vec<u8>,
    pc: usize,
    instructions: Vec<Instruction>,
}

pub enum Error {
    StackUnderflow,
    End,
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
        if self.pc >= self.instructions.len() {
            return Err(Error::End);
        }

        match self.instructions[self.pc] {
            Instruction::PUSH => {
                self.stack.push(1);
                self.pc += 1;
            }
            Instruction::POP => {
                self.stack.pop().ok_or(Error::StackUnderflow)?;
                self.pc += 1;
            }
            Instruction::INC => {
                let top: u8 = self.stack.pop().ok_or(Error::StackUnderflow)?;
                self.stack.push(top + 1);
                self.pc += 1;
            }
            Instruction::DEC => {
                let top: u8 = self.stack.pop().ok_or(Error::StackUnderflow)?;
                self.stack.push(top - 1);
                self.pc += 1;
            }
            Instruction::JNZ => {
                let i: u8 = self.stack.pop().ok_or(Error::StackUnderflow)?;
                let eq: u8 = self.stack.pop().ok_or(Error::StackUnderflow)?;
                if i != eq {
                    self.pc = i as usize;
                }

                self.stack.push(eq);
            }
            Instruction::JZ => {
                let i: u8 = self.stack.pop().ok_or(Error::StackUnderflow)?;
                let eq: u8 = self.stack.pop().ok_or(Error::StackUnderflow)?;
                if i == eq {
                    self.pc = i as usize;
                }

                self.stack.push(eq);
            }
            Instruction::READ => todo!(),
            Instruction::WRITE => todo!(),
            Instruction::STORE => {
                let i: u8 = self.stack.pop().ok_or(Error::StackUnderflow)?;
                let stored_val: u8 = self.stack.pop().ok_or(Error::StackUnderflow)?;

                self.memory[i as usize] = stored_val;
                self.pc += 1;
            }
            Instruction::LOAD => {
                let i: u8 = self.stack.pop().ok_or(Error::StackUnderflow)?;
                self.stack.push(self.memory[i as usize]);
                self.pc += 1;
            }
        };
        Ok(())
    }

    pub fn stack_top(&self) -> Option<u8> {
        self.stack.last().map(|i| i.to_owned())
    }
}
