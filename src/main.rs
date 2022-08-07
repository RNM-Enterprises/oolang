mod instruction;
use std::env;
use std::io;
use std::io::BufRead;
use std::path::Path;

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
        println!("Could not open file: {} ", path.display());
        return;
    }
    let file = file.unwrap();

    //run the program
    println!("Running OOLANG file: {filename}...");
    let input = get_input();

    if input.is_err() {
        println!("Could not read from stdin: {}", input.unwrap_err());
        return;
    }

    println!("{:?}", input);

    if let Some(i) = oolang::run_cli(&file, &input.unwrap()) {
        println!("Result: {}", i)
    } else {
        println!("Your program has no output. Perhaps there was an error?")
    }
}
