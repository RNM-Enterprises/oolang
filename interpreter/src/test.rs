use crate::instruction::Instruction::*;
use crate::instruction::Instruction;

use crate::vm::Error;



#[test]
fn test_empty_instructions() {
    // tests that empty stack returns an error
    let instructions: Vec<Instruction> = Vec::new();

    let result = crate::execute(instructions);
    assert_eq!(result, Err(Error::End))
}

#[test]
fn test_stack_underflow() {
    // tests that pop from empty stack gives underflow error
    let instructions: Vec<Instruction> = Vec::new();
    instructions.push(POP);

    let result = crate::execute(instructions);
    assert_eq!(result, Err(Error::StackUnderflow))
}

#[test]
fn test_addition() {
    // tests that 10 + 5 = 15
    let instructions: Vec<Instruction> = Vec::new();
    instructions.push(PUSH);
    instructions.append(&mut vec![INC;4]);
    instructions.push(PUSH);
    instructions.push(&mut vec![INC;9]);
    instructions.push(ADD);

    let result = crate::execute(instructions);
    assert_eq!(result, 15)
}

#[test]
fn test_multiplication() {
    // tests that program for 6 * 9 returns 54
    let instructions: Vec<Instruction> = Vec::new();
    instructions.push(PUSH);
    instructions.push(&mut vec![INC;5]);
    instructions.push(DEC);
    instructions.push(PUSH);
    instructions.push(&mut vec![INC;8]);
    instructions.push(PUSH);
    instructions.push(STORE);
    instructions.push(PUSH);
    instructions.push(INC);
    instructions.push(STORE);

    instructions.push(PUSH);
    instructions.push(LOAD);
    instructions.push(PUSH);
    instructions.push(INC);
    instructions.push(LOAD);
    instructions.push(ADD);
    instructions.push(PUSH);
    instructions.push(INC);
    instructions.push(STORE);
    instructions.push(DEC);
    instructions.push(PUSH);
    instructions.push(&mut vec![INC;16]);
    instructions.push(BNZ);
    instructions.push(PUSH);
    instructions.push(INC);
    instructions.push(LOAD);

    let result = crate::execute(instructions);
    assert_eq!(result, 54)
}
