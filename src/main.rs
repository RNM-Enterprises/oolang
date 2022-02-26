mod instruction;
use std::env;
use std::io::Read;
use std::path::Path;

fn main() {
    let cli_args: Vec<String> = env::args().collect();
    if cli_args.len() < 2 {
        println!("Please specify an OOLANG file");
        return;
    }

    let path = Path::new(&cli_args[1]);
    if !path.exists() {
        println!("File {} does not exist", path.display());
        return;
    }
    let filename = path
        .file_name()
        .and_then(|c| c.to_str())
        .expect("Could not read file name");

    let fileext = path.file_name().expect("Could not read file extension");
    if fileext != "oo" {
        println!("Not a valid OOLANG file. OOLANG files should end in .oo ",);
        return;
    }

    let file = std::fs::read_to_string(path);
    if file.is_err() {
        println!("Could not open file: {} ", path.display())
    }
}
