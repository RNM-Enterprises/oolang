use crate::instruction::Instruction;
pub struct State {
    memory: [u8; 256],
    stack: Vec<u8>,
    pc: usize,
    instructions: Vec<Instruction>,
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
}
