use crate::instruction::Instruction;
#[derive(Debug)]
pub struct State {
    memory: [u8; 256],
    stack: Vec<u8>,
    pc: usize,
    instructions: Vec<Instruction>,
}

#[derive(PartialEq, Eq, Debug)]
pub enum Error {
    StackUnderflow,
    End(Option<u8>),
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
            return Err(Error::End(self.stack_top()));
        }

        match self.instructions[self.pc] {
            Instruction::Push => {
                self.stack.push(1);
                self.pc += 1;
            }
            Instruction::Pop => {
                self.stack.pop().ok_or(Error::StackUnderflow)?;
                self.pc += 1;
            }
            Instruction::Inc => {
                let top: u8 = self.stack.pop().ok_or(Error::StackUnderflow)?;
                self.stack.push(top + 1);
                self.pc += 1;
            }
            Instruction::Dec => {
                let top: u8 = self.stack.pop().ok_or(Error::StackUnderflow)?;
                self.stack.push(top - 1);
                self.pc += 1;
            }
            Instruction::Jnz => {
                let instr: u8 = self.stack.pop().ok_or(Error::StackUnderflow)?;
                let val: u8 = self.stack.pop().ok_or(Error::StackUnderflow)?;
                if val != 0 {
                    self.pc = instr as usize;
                } else {
                    self.pc += 1;
                }

                self.stack.push(val);
            }
            Instruction::Jz => {
                let instr: u8 = self.stack.pop().ok_or(Error::StackUnderflow)?;
                let val: u8 = self.stack.pop().ok_or(Error::StackUnderflow)?;
                if val == 0 {
                    self.pc = instr as usize;
                } else {
                    self.pc += 1;
                }

                self.stack.push(val);
            }
            Instruction::Read => todo!(),
            Instruction::Write => todo!(),
            Instruction::Store => {
                let addr: u8 = self.stack.pop().ok_or(Error::StackUnderflow)?;
                let stored_val: u8 = self.stack.pop().ok_or(Error::StackUnderflow)?;

                self.memory[addr as usize] = stored_val;
                self.pc += 1;
            }
            Instruction::Load => {
                let addr: u8 = self.stack.pop().ok_or(Error::StackUnderflow)?;
                self.stack.push(self.memory[addr as usize]);
                self.pc += 1;
            }
            Instruction::Add => {
                let a: u8 = self.stack.pop().ok_or(Error::StackUnderflow)?;
                let b: u8 = self.stack.pop().ok_or(Error::StackUnderflow)?;
                self.stack.push(a + b);
                self.pc += 1;
            }
        };
        Ok(())
    }

    pub fn stack_top(&self) -> Option<u8> {
        self.stack.last().map(|i| i.to_owned())
    }
}
