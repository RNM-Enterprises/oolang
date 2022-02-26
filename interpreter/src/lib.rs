mod instruction;
mod vm;
use instruction::Instruction;
use unicode_segmentation::UnicodeSegmentation;

#[cfg(test)]
mod test;

pub fn parse(commands: &str) -> Option<Vec<Instruction>> {
    commands.graphemes(true).map(Instruction::from).collect()
}

pub fn execute(instructions: Vec<Instruction>) -> vm::Error {
    let mut vm: vm::State = vm::State::init(instructions);
    loop {
        if let Err(e) = vm.execute() {
            break e;
        }
        dbg!(&vm.stack);
        dbg!(&vm.instructions[*&vm.pc as usize]);
        dbg!(&vm.memory[1]);
        dbg!(&vm.memory[2]);
    }
}

pub fn run(program: &str) -> Option<u8> {
    let instructions: Vec<Instruction> = parse(program)?;
    match execute(instructions) {
        vm::Error::StackUnderflow => None,
        vm::Error::End(x) => x,
    }
}
