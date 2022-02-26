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
    instructions.append(Instruction::POP);
    let result = crate::execute(instructions);
    assert_eq!(result, Err(Error::StackUnderflow))
}

#[test]
fn test_addition() {
    // tests that 10 + 5 = 15

    let instructions: Vec<Instruction> = Vec::new();
    instructions.append(Instruction::PUSH);
    for a in 1..5 {
        instructions.append(Instruction::INC);
    }
    instructions.append(Instruction::PUSH);
    for a in 1..10 {
        instructions.append(Instructon::INC);
    }
    instructions.append(Instruction::ADD);
    let result = crate::execute(instructions);
    assert_eq!(result, 15)
}

#[test]
fn test_multiplication() {
    // tests that program for 6 * 9 returns 54
    // TODO

    let instructions: Vec<Instruction> = Vec::new();
    instructions.push(Instruction::LOAD);
    instructions.push(Instruction::LOAD);
    let result = crate::execute(instructions);

}
