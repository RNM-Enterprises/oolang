mod instruction;
mod vm;
use instruction::Instruction;
use unicode_segmentation::UnicodeSegmentation;

#[cfg(test)]
mod test;

pub fn parse(commands: &str) -> Vec<Instruction> {
    commands
        .graphemes(true)
        .filter_map(Instruction::from)
        .collect()
}

pub fn execute(instructions: Vec<Instruction>) -> vm::Error {
    let mut vm: vm::State = vm::State::init(instructions);
    loop {
        if let Err(e) = vm.execute() {
            break e;
        }
    }
}

pub fn run(program: &str) -> Option<u8> {
    let instructions: Vec<Instruction> = parse(program);
    match execute(instructions) {
        vm::Error::StackUnderflow => None,
        vm::Error::End(x) => x,
    }
}
