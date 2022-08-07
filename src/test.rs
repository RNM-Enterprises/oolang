#![cfg(test)]
use crate::instruction::Instruction;
use crate::instruction::Instruction::*;
use crate::vm::*;

//executes a vm
//used for testing
//basically the same as crate::run
fn execute(vm: &mut State) -> Option<u8> {
    loop {
        match vm.step() {
            Ok(()) => (),
            Err(Interrupt::End(top)) => break top,
            Err(Interrupt::Output(c)) => println!("{c}"),
            Err(Interrupt::StackUnderflow) => panic!("Stack underflow, aborting."),
            Err(Interrupt::OutOfBounds(i)) => {
                panic!("Attempted to read instruction out of bounds (index {}, max program length {}) , aborting.", i, vm.instructions_len())
            }
        }
    }
}

#[test]
fn test_empty_instructions() {
    // tests that empty stack returns an error
    let instructions: Vec<Instruction> = Vec::new();

    let mut vm = State::init(instructions);
    let result = execute(&mut vm);
    assert_eq!(result, None)
}

#[test]
#[should_panic]
fn test_stack_underflow() {
    // tests that pop from empty stack gives underflow error
    let instructions: Vec<Instruction> = vec![Pop];

    let mut vm = State::init(instructions);
    execute(&mut vm);
}

#[test]
fn test_addition() {
    // tests that 10 + 5 = 15
    let mut instructions: Vec<Instruction> = vec![Push];
    instructions.append(&mut vec![Inc; 4]);
    instructions.push(Push);
    instructions.append(&mut vec![Inc; 9]);
    instructions.push(Add);

    let mut vm = State::init(instructions);
    let result = execute(&mut vm);
    assert_eq!(result, Some(15))
}

#[test]
fn test_multiplication() {
    // tests that program for 6 * 9 returns 54
    let mut instructions: Vec<Instruction> = vec![Push];
    instructions.append(&mut vec![Inc; 5]);
    instructions.push(Dec);
    instructions.push(Push);
    instructions.append(&mut vec![Inc; 8]);
    instructions.push(Push);
    instructions.push(Store);
    instructions.push(Push);
    instructions.push(Load);
    instructions.push(Push);
    instructions.push(Inc);
    instructions.push(Store);

    instructions.push(Push);
    instructions.push(Load);
    instructions.push(Push);
    instructions.push(Inc);
    instructions.push(Load);
    instructions.push(Add);
    instructions.push(Push);
    instructions.push(Inc);
    instructions.push(Store);
    instructions.push(Dec);
    instructions.push(Push);
    instructions.append(&mut vec![Inc; 22]);
    instructions.push(Jnz);
    instructions.push(Push);
    instructions.push(Inc);
    instructions.push(Load);

    let mut vm = State::init(instructions);
    let result = execute(&mut vm);
    assert_eq!(result, Some(54))
}

use crate::parse;
#[test]
fn parse_test() {
    let instructions = parse("O0ǾᏫ𐍉Ꝍ⒪ₒ⭕◎◯");
    assert_eq!(
        instructions,
        vec![Push, Pop, Inc, Dec, Jnz, Jz, Read, Write, Add, Load, Store]
    )
}
#[test]
fn parse_test_with_spaces() {
    let instructions = parse("O0   ǾᏫ   𐍉Ꝍ   ⒪  ₒ⭕\n◎◯");
    assert_eq!(
        instructions,
        vec![Push, Pop, Inc, Dec, Jnz, Jz, Read, Write, Add, Load, Store]
    )
}

#[test]
fn test_io() {
    //read and write in a loop
    let s = "Hello, World!\n";
    let (_, output) = crate::run_buffered("⒪ O◯ O◎ ₒ O◎ OᏫ𐍉 ", s.as_bytes());
    assert_eq!(s, output);
}
