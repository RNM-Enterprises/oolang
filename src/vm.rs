use std::num::Wrapping;

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
    OutOfBounds(usize),
    End(Option<u8>),
    Output(char),
}

impl State {
    //create a new vm with empty input
    //only really used in tests
    #[allow(unused)]
    pub fn init(instructions: Vec<Instruction>) -> Self {
        Self::init_with_input(instructions, &[]).unwrap()
    }
    //create a new vm with the given input to the program
    pub fn init_with_input(instructions: Vec<Instruction>, input: &[u8]) -> Option<Self> {
        Some(State {
            memory: [0; 256],
            stack: Vec::new(),
            pc: 0,
            instructions,
            input: input.to_vec(),
        })
    }

    //execute a single instruction
    pub fn step(&mut self) -> Result<(), Interrupt> {
        //return end of program if pc is past end of instructions
        if self.pc >= self.instructions.len() {
            return Err(Interrupt::End(self.stack_top()));
        }

        match self
            .instructions
            .get(self.pc)
            .ok_or(Interrupt::OutOfBounds(self.pc))?
        {
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
                self.stack.push((Wrapping(top) + Wrapping(1_u8)).0);
                self.pc += 1;
            }
            Instruction::Dec => {
                let top: u8 = self.stack.pop().ok_or(Interrupt::StackUnderflow)?;
                self.stack.push((Wrapping(top) - Wrapping(1_u8)).0);

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
                self.stack.push((Wrapping(a) + Wrapping(b)).0);
                self.pc += 1;
            }
        };
        Ok(())
    }
    //peek the top of the stack
    pub fn stack_top(&self) -> Option<u8> {
        self.stack.last().map(|i| i.to_owned())
    }
}
