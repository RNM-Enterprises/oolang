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
}

impl Instruction {
    pub fn from(c: &str) -> Option<Self> {
        match c {
            _ => None,
        }
    }
}
