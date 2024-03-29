# OOLANG
[![GitHub Workflow Status (branch)](https://img.shields.io/github/workflow/status/RNM-Enterprises/oolang/CI/main?style=for-the-badge)](https://github.com/RNM-Enterprises/oolang/actions)
[![GitHub](https://img.shields.io/github/license/RNM-Enterprises/oolang?style=for-the-badge)](./LICENSE)
[![Crates.io](https://img.shields.io/crates/d/OOLANG?style=for-the-badge)](https://crates.io/crates/OOLANG)
[![Crates.io](https://img.shields.io/crates/v/OOLANG?style=for-the-badge)](https://crates.io/crates/OOLANG)
[![Docs.rs](https://img.shields.io/docsrs/OOLANG/latest?style=for-the-badge)](https://docs.rs/OOLANG/latest/oolang/)

OOLANG is an esoteric stack-based programming language where all instructions/commands are differnet unicode O characters

## Usage

`oorun`, a command line runner, is available on [crates.io](https://crates.io/crates/OOLANG), and can be installed with Cargo:

```rust
cargo install OOLANG
```

OOLANG files end in `.oo`, and consist of a number of commands specified each by a single unicode glyph. Any other character is ignored. Comments are supported using the `#` character, which comments out up to the end of the line.

`oorun file.oo` will execute your OOLANG program, reading from `stdin` and printing any output to the console. The return value is also shown. For example,

```
$ echo -n "Hello, World!" | oorun echo.oo
Running OOLANG file: echo.oo...
Output: Hello, World!
Return value: 13
```

## Language Specification

OOLANG emulates a [stack machine](https://en.wikipedia.org/wiki/Stack_machine), not dissimilar to how the JVM or Python's bytecode interpreter works.

The virtual machine consists of a stack of bytes, and 256 bytes of addressable memory. A valid OOLANG program may consist of any combination of the the following 10 commands:

| Command | Unicode Character Name                       | Command Name | Description                                                                                 |
| ------- | -------------------------------------------- | ------------ | ------------------------------------------------------------------------------------------- |
| `O`     | Latin Capital Letter O                       | `PUSH`       | Push the value `1` onto the top of the stack.                                                |
| `0`     | Digit Zero                                   | `POP`        | Pop the value from the top of the stack, discarding it.                                      |
| `Ǿ`     | Latin Capital Letter O with Stroke and Acute | `INC`        | Increment the value on the top of the stack.                                      |
| `Ꮻ`     | Cherokee Letter Wi                           | `DEC`        | Decrement the value on the top of the stack.                                    |
| `⭕`    | Heavy Large Circle                           | `ADD`        | Pop the top two stack values, add them, push the result.                                     |
| `𐍉`     | Gothic Letter Othal                          | `JNZ`        | Jump to the address at the top of the stack, if the 2nd-top value is non-zero.               |
| `Ꝍ`     | Latin Capital Letter O with Loop             | `JZ`         | Jump to the address at the top of the stack, if the 2nd-top value is zero.                   |
| `◎`     | Bullseye                                     | `LOAD`       | Pop a memory address from the stack, and push the value at that address.                      |
| `◯`     | Large Circle                                 | `STORE`      | Pop the top two values from the stack, storing the 2nd value at the address specified by the 1st.  |
| `⒪`     | Parenthesized Latin Small Letter O           | `READ`       | Read the next byte from stdin, pushing it to the top of the stack, or `0` if stdin is empty. |
| `ₒ`     | Latin Subscript Small Letter O               | `WRITE`      | Pop from the stack and write the value to stdout as an ascii character.                       |

The return value of an OOLANG program is the value at the top of the stack when the program exits.

## Architecture

The language and interpreter are written in Rust. The compiled binary executable acts as a command line program, but the interpreter crate also exposes a public API. When compiled to webassembly, this API is used to provide functionality for the Web IDE within the browser.
