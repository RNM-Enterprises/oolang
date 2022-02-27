mod instruction;
use std::env;
use std::io::{stdin, BufRead};
use std::path::Path;

//get all input from stdin
fn get_input() -> String {
    let mut buf = String::new();
    let stdin = stdin();
    for line in stdin.lock().lines().flatten() {
        buf.push_str(&line);
    }
    buf
}

fn main() {
    //get command line args and make sure the right number of them exist
    let cli_args: Vec<String> = env::args().collect();
    if cli_args.len() < 2 {
        println!("Please specify an OOLANG file");
        return;
    }

    //check file exists
    let path = Path::new(&cli_args[1]);
    if !path.exists() {
        println!("File {} does not exist", path.display());
        return;
    }
    //get filename and extension
    let filename = path
        .file_name()
        .and_then(|c| c.to_str())
        .expect("Could not read file name");

    let fileext = path.extension().expect("Could not read file extension");
    if fileext != "oo" {
        println!("Not a valid OOLANG file. OOLANG files should end in .oo ",);
        return;
    }
    //read the file
    let file = std::fs::read_to_string(path);
    if file.is_err() {
        println!("Could not open file: {} ", path.display())
    }
    let file = file.unwrap();

    //run the program
    println!("Running OOLANG file: {filename}...");
    if let Some(i) = oolang::run_cli(&file, &get_input()) {
        println!("Result: {}", i)
    } else {
        println!("Your program has no output. Perhaps there was an error?")
    }
}
