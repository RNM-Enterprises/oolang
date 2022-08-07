//! The OOLANG library exports a single function, [`run_buffered`][run_buffered], that executes an OOLANG program and returns the result and output.

mod instruction;
mod vm;

use instruction::Instruction;
use unicode_segmentation::UnicodeSegmentation;

mod test;

/// Run the program with the input given, buffering all output data.
/// Assumes `program` is a valid OOLANG program.
/// The output buffer is then returned, along with the return value from the top of the stack.
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
