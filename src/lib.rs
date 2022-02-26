mod instruction;
mod vm;

use instruction::Instruction;
use unicode_segmentation::UnicodeSegmentation;

#[cfg(test)]
mod test;

fn parse(commands: &str) -> Vec<Instruction> {
    commands
        .graphemes(true)
        .filter_map(Instruction::from_glyph)
        .collect()
}

fn execute<I>(vm: &mut vm::State<I>) -> Option<u8>
where
    I: Iterator<Item = u8>,
{
    loop {
        match vm.step() {
            Ok(()) => (),
            Err(vm::Interrupt::End(top)) => break top,
            Err(vm::Interrupt::Output(c)) => print!("{c}"),
            Err(vm::Interrupt::StackUnderflow) => break None,
        }
    }
}

pub fn run(program: &str) -> Option<u8> {
    let instructions: Vec<Instruction> = parse(program);
    let mut vm = vm::State::init(instructions);
    execute(&mut vm)
}

pub fn run_with_input<I: Iterator<Item = u8>>(program: &str, input: I) -> Option<u8>
where
    I: Iterator<Item = u8>,
{
    let instructions: Vec<Instruction> = parse(program);
    let mut vm = vm::State::init_with_input(instructions, input);
    execute(&mut vm)
}
