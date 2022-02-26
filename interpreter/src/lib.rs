mod instruction;
mod vm;
use instruction::Instruction;
use unicode_segmentation::UnicodeSegmentation;

pub fn parse(commands: &str) -> Option<Vec<Instruction>> {
    commands.graphemes(true).map(Instruction::from).collect()
}

pub fn run(program: &str) -> Option<u8> {
    let instructions: Vec<Instruction> = parse(program)?;
    let mut vm: vm::State = vm::State::init(instructions);
    loop {
        match vm.execute() {
            Ok(()) => (),
            Err(vm::Error::End) => break vm.stack_top(),
            Err(vm::Error::StackUnderflow) => break None,
        }
    }
}
