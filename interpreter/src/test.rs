use crate::instruction::Instruction;
use crate::vm::Error;
#[test]
fn test_1() {
    let instructions: Vec<Instruction> = Vec::new();
    let result = crate::execute(instructions);
    assert_eq!(result, Err(Error::End))
}
