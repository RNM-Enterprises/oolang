#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum Instruction {
    PUSH,  //Push a 1 onto the stack
    POP,   //Pop the top value from the stack, discarding it
    INC,   //Increment the value on top of the stack
    DEC,   //Decrement the value on top of the stack
    JNZ, //Jump to the instruction number of the top stack value, if the 2nd-top stack value is non-zero
    JZ,  //Jump to the instruction number of the top stack value, if the 2nd-top stack value is zero
    READ, //Read a byte from stdin and push it to stack
    WRITE, //Pop the top stack value, writing to stdout
    STORE, //Store the 2nd top stack value at the address specified by the top stack value
    LOAD, //Replace the top stack value with the value at the address specified by it
    ADD, // Add the top two numbers from the stack
}

use Instruction::*;

impl Instruction {
    pub fn from(c: &str) -> Option<Self> {
        match c {
            "O" => Some(PUSH),    // Latin Capital letter O
            "0" => Some(POP),     // Digit Zero
            "Ǿ" => Some(INC),    // LATIN CAPITAL LETTER O WITH STROKE AND ACUTE
            "Ꮻ" => Some(DEC),   //CHEROKEE LETTER WI
            "𐍉" => Some(JNZ),  // GOTHIC LETTER OTHAL
            "Ꝍ" => Some(JZ),    // LATIN CAPITAL LETTER O WITH LOOP
            "⒪" => Some(READ),  //parenthesized latin small letter o
            "ₒ" => Some(WRITE), //Latin Subscript Small Letter O
            "⭕" => Some(ADD),   //Hollow Red Circle
            "◎" => Some(LOAD),  // Bullseye
            "⎈" => Some(STORE), // Helm Symbol
            _ => None,
        }
    }
}
