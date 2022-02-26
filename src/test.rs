#![cfg(test)]
use crate::execute_print;
use crate::instruction::Instruction;
use crate::instruction::Instruction::*;

use crate::vm::State;

#[test]
fn test_empty_instructions() {
    // tests that empty stack returns an error
    let instructions: Vec<Instruction> = Vec::new();

    let mut vm = State::init(instructions);
    let result = execute_print(&mut vm);
    assert_eq!(result, None)
}

#[test]
fn test_stack_underflow() {
    // tests that pop from empty stack gives underflow error
    let instructions: Vec<Instruction> = vec![Pop];

    let mut vm = State::init(instructions);
    let result = execute_print(&mut vm);
    assert_eq!(result, None)
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
    let result = execute_print(&mut vm);
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
    let result = execute_print(&mut vm);
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
