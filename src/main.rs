mod instruction;
use clap::Parser;
use std::io::{self, BufRead};

fn main() {
    let cli = Cli::parse();

    //read the file
    let file = std::fs::read_to_string(&cli.file);
    if file.is_err() {
        println!("Could not open file: {} ", &cli.file);
        return;
    }

    //run the program
    println!("Running OOLANG file: {}...", &cli.file);
    let input = get_input();

    if input.is_err() {
        println!("Could not read from stdin: {}", input.unwrap_err());
        return;
    }

    if let Some(i) = oolang::run(&cli.file, input.unwrap().as_bytes()) {
        println!("Top of stack: {}", i)
    } else {
        println!("Stack empty on program exit.")
    }
}

//get all input from stdin
fn get_input() -> Result<String, io::Error> {
    Ok(if atty::is(atty::Stream::Stdin) {
        String::new()
    } else {
        let mut input = String::new();
        while io::stdin().lock().read_line(&mut input)? != 0 {}
        input
    })
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// A CHIP-8 ROM to load into the interpreter
    #[clap(validator = file_valid)]
    file: String,
}

fn file_valid(f: &str) -> Result<(), &'static str> {
    let p = std::path::Path::new(f);
    if !p.is_file() {
        Err("File does not exist.")
    } else {
        Ok(())
    }
}
