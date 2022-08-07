//! The OOLANG library, exporting two functions, for two different ways to run your OOLANG program.

mod instruction;
mod vm;

use instruction::Instruction;
use unicode_segmentation::UnicodeSegmentation;

/// Run the program with the input given. Output is printed to `stdout`.
/// All non-OOLANG characters in the input are ignored, along with any characters on a line after a '#' (indicating a comment).
/// The return value from the top if the stack is returned (if there is one).
/// If there is an error during execution, then the function will panic with the error.
pub fn run(program: &str, input: &[u8]) -> Option<u8> {
    let instructions: Vec<Instruction> = parse(program);

    vm::State::init_with_input(instructions, input).and_then(|mut vm| loop {
        match vm.step() {
            Ok(()) => (),
            Err(vm::Interrupt::End(top)) => break top,
            Err(vm::Interrupt::Output(c)) => println!("{c}"),
            Err(vm::Interrupt::StackUnderflow) => panic!("Stack underflow, aborting."),
            Err(vm::Interrupt::OutOfBounds(i)) => panic!("Attempted to read instruction out of bounds (index {}, max program length {}) , aborting.", i, program.len()),
        }
    })
}

/// Run the program with the input given, buffering all output data.
/// All non-OOLANG characters in the input are ignored, along with any characters on a line after a '#' (indicating a comment).
/// The output buffer is then returned, along with the return value from the top of the stack (if there is one).
/// If there is an error during execution, then the buffer is returned from execution up to that point, and the return value is None.
pub fn run_buffered(program: &str, input: &[u8]) -> (Option<u8>, String) {
    let instructions: Vec<Instruction> = parse(program);
    let mut output_buffer = String::new();

    (
        vm::State::init_with_input(instructions, input).and_then(|mut vm| loop {
            match vm.step() {
                Ok(()) => (),
                Err(vm::Interrupt::End(top)) => break top,
                Err(vm::Interrupt::Output(c)) => output_buffer.push(c),
                Err(vm::Interrupt::StackUnderflow) | Err(vm::Interrupt::OutOfBounds(_)) => {
                    break None
                }
            }
        }),
        output_buffer,
    )
}

//remove comments from input string
fn clean_file(file: &str) -> String {
    file.lines()
        .filter_map(|line: &str| line.split('#').next())
        .collect::<Vec<&str>>()
        .join("")
}

//parse the unicode graphemes into instruction vec
fn parse(commands: &str) -> Vec<Instruction> {
    clean_file(commands)
        .graphemes(true)
        .filter_map(Instruction::from_glyph)
        .collect()
}
