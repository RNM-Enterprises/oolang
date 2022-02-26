mod instruction;
mod vm;

use instruction::Instruction;
use unicode_segmentation::UnicodeSegmentation;

mod test;

//parse the unicode graphemes into instruction vec
fn parse(commands: &str) -> Vec<Instruction> {
    commands
        .graphemes(true)
        .filter_map(Instruction::from_glyph)
        .collect()
}

//execute, priniting to stdout
fn execute_print(vm: &mut vm::State) -> Option<u8> {
    loop {
        match vm.step() {
            Ok(()) => (),
            Err(vm::Interrupt::End(top)) => break top,
            Err(vm::Interrupt::Output(c)) => print!("{c}"),
            Err(vm::Interrupt::StackUnderflow) => break None,
        }
    }
}
//execute, saving output to a buffer
fn execute_buffer(vm: &mut vm::State, output_buffer: &mut String) -> Option<u8> {
    loop {
        match vm.step() {
            Ok(()) => (),
            Err(vm::Interrupt::End(top)) => break top,
            Err(vm::Interrupt::Output(c)) => output_buffer.push(c),
            Err(vm::Interrupt::StackUnderflow) => break None,
        }
    }
}

//run the program using execute_print, designed for cli use
pub fn run_cli(program: &str, input: &str) -> Option<u8> {
    let instructions: Vec<Instruction> = parse(program);
    let mut vm = vm::State::init_with_input(instructions, input)?;
    execute_print(&mut vm)
}

//run the program, returning the output buffer when done
pub fn run_buffered(program: &str, input: &str) -> (Option<u8>, String) {
    let instructions: Vec<Instruction> = parse(program);
    let mut output_buffer = String::new();

    (
        vm::State::init_with_input(instructions, input)
            .and_then(|mut vm| execute_buffer(&mut vm, &mut output_buffer)),
        output_buffer,
    )
}
