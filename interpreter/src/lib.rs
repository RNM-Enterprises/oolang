mod instruction;
mod vm;
use instruction::Instruction;

pub fn parse(_commands: &str) -> Vec<Instruction> {
    Vec::new()
}

pub fn run(_program: &str) -> Option<u8> {
    None
}
