use crate::instruction::Instruction;
use std::iter::{empty, Empty};

//represents the state of the stack machine
//generic over an iterator for input

#[derive(Debug)]
pub struct State<I: Iterator<Item = u8>> {
    memory: [u8; 256],
    stack: Vec<u8>,
    pc: usize,
    instructions: Vec<Instruction>,
    input: I,
}

//VM Interrupts
#[derive(PartialEq, Eq, Debug)]
pub enum Interrupt {
    StackUnderflow,
    End(Option<u8>),
    Output(char),
}

impl State<Empty<u8>> {
    pub fn init(instructions: Vec<Instruction>) -> Self {
        State {
            memory: [0; 256],
            stack: Vec::new(),
            pc: 0,
            instructions,
            input: empty(),
        }
    }
}

impl<I: Iterator<Item = u8>> State<I> {
    pub fn init_with_input(instructions: Vec<Instruction>, input: I) -> Self {
        State {
            memory: [0; 256],
            stack: Vec::new(),
            pc: 0,
            instructions,
            input,
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
                self.stack.push(self.input.next().unwrap_or(0));
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
