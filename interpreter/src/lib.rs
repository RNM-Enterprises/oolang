mod instruction;
mod vm;
use instruction::Instruction;
use unicode_segmentation::UnicodeSegmentation;

#[cfg(test)]
mod test;

pub fn parse(commands: &str) -> Option<Vec<Instruction>> {
    commands.graphemes(true).map(Instruction::from).collect()
}

pub fn execute(instructions: Vec<Instruction>) -> Result<u8, vm::Error> {
    let mut vm: vm::State = vm::State::init(instructions);
    loop {
        if let Err(e) = vm.execute() {
            break Err(e);
        }
    }
}

pub fn run(program: &str) -> Option<u8> {
    let instructions: Vec<Instruction> = parse(program)?;
    execute(instructions).ok()
}
