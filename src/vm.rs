use crate::instruction::Instruction;

//represents the state of the stack machine
//generic over an iterator for input

#[derive(Debug)]
pub struct State {
    memory: [u8; 256],
    stack: Vec<u8>,
    pc: usize,
    instructions: Vec<Instruction>,
    input: Vec<u8>,
}

//VM Interrupts
#[derive(PartialEq, Eq, Debug)]
pub enum Interrupt {
    StackUnderflow,
    End(Option<u8>),
    Output(char),
}

impl State {
    pub fn init(instructions: Vec<Instruction>) -> Self {
        State {
            memory: [0; 256],
            stack: Vec::new(),
            pc: 0,
            instructions,
            input: Vec::new(),
        }
    }

    pub fn init_with_input(instructions: Vec<Instruction>, input: &str) -> Option<Self> {
        //check all the chars can convert to u8s
        if input
            .chars()
            .map(|c| c.try_into())
            .collect::<Result<Vec<u8>, _>>()
            .is_err()
        {
            None
        } else {
            // if we're all good then create the struct with panicking conversion
            Some(State {
                memory: [0; 256],
                stack: Vec::new(),
                pc: 0,
                instructions,
                input: input.chars().rev().map(|c| c as u8).collect(),
            })
        }
    }

    pub fn step(&mut self) -> Result<(), Interrupt> {
        if self.pc >= self.instructions.len() {
            return Err(Interrupt::End(self.stack_top()));
        }

        match self.instructions[self.pc] {
            Instruction::Push => {
                self.stack.push(1);
                self.pc += 1;
            }
            Instruction::Pop => {
                self.stack.pop().ok_or(Interrupt::StackUnderflow)?;
                self.pc += 1;
            }
            Instruction::Inc => {
                let top: u8 = self.stack.pop().ok_or(Interrupt::StackUnderflow)?;
                self.stack.push(top + 1);
                self.pc += 1;
            }
            Instruction::Dec => {
                let top: u8 = self.stack.pop().ok_or(Interrupt::StackUnderflow)?;
                self.stack.push(top - 1);
                self.pc += 1;
            }
            Instruction::Jnz => {
                let instr: u8 = self.stack.pop().ok_or(Interrupt::StackUnderflow)?;
                let val: u8 = self.stack.pop().ok_or(Interrupt::StackUnderflow)?;
                if val != 0 {
                    self.pc = instr as usize;
                } else {
                    self.pc += 1;
                }

                self.stack.push(val);
            }
            Instruction::Jz => {
                let instr: u8 = self.stack.pop().ok_or(Interrupt::StackUnderflow)?;
                let val: u8 = self.stack.pop().ok_or(Interrupt::StackUnderflow)?;
                if val == 0 {
                    self.pc = instr as usize;
                } else {
                    self.pc += 1;
                }

                self.stack.push(val);
            }
            Instruction::Read => {
                self.stack.push(self.input.pop().unwrap_or(0));
                self.pc += 1;
            }

            Instruction::Write => {
                let top: char = self.stack.pop().ok_or(Interrupt::StackUnderflow)?.into();
                self.pc += 1;
                return Err(Interrupt::Output(top));
            }
            Instruction::Store => {
                let addr: u8 = self.stack.pop().ok_or(Interrupt::StackUnderflow)?;
                let stored_val: u8 = self.stack.pop().ok_or(Interrupt::StackUnderflow)?;

                self.memory[addr as usize] = stored_val;
                self.pc += 1;
            }
            Instruction::Load => {
                let addr: u8 = self.stack.pop().ok_or(Interrupt::StackUnderflow)?;
                self.stack.push(self.memory[addr as usize]);
                self.pc += 1;
            }
            Instruction::Add => {
                let a: u8 = self.stack.pop().ok_or(Interrupt::StackUnderflow)?;
                let b: u8 = self.stack.pop().ok_or(Interrupt::StackUnderflow)?;
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
