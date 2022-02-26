#![allow(dead_code)]
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum Instruction {
    Push,  //Push a 1 onto the stack
    Pop,   //Pop the top value from the stack, discarding it
    Inc,   //Increment the value on top of the stack
    Dec,   //Decrement the value on top of the stack
    Jnz, //Jump to the instruction number of the top stack value, if the 2nd-top stack value is non-zero
    Jz,  //Jump to the instruction number of the top stack value, if the 2nd-top stack value is zero
    Read, //Read a byte from stdin and push it to stack
    Write, //Pop the top stack value, writing to stdout
    Store, //Store the 2nd top stack value at the address specified by the top stack value
    Load, //Replace the top stack value with the value at the address specified by it
    Add, // Add the top two numbers from the stack
}

use Instruction::*;

impl Instruction {
    pub fn from_glyph(c: &str) -> Option<Self> {
        match c {
            "O" => Some(Push),    // Latin Capital letter O
            "0" => Some(Pop),     // Digit Zero
            "Ǿ" => Some(Inc),    // LATIN CAPITAL LETTER O WITH STROKE AND ACUTE
            "Ꮻ" => Some(Dec),   //CHEROKEE LETTER WI
            "𐍉" => Some(Jnz),  // GOTHIC LETTER OTHAL
            "Ꝍ" => Some(Jz),    // LATIN CAPITAL LETTER O WITH LOOP
            "⒪" => Some(Read),  //parenthesized latin small letter o
            "ₒ" => Some(Write), //Latin Subscript Small Letter O
            "⭕" => Some(Add),   //Hollow Red Circle
            "◎" => Some(Load),  // Bullseye
            "◯" => Some(Store), // Large Circle
            _ => None,
        }
    }
}
