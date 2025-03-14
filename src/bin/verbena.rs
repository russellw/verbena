use std::env;
use std::fs;
use std::process;
use verbena::*;

fn main() {
    // Get command line arguments
    let args: Vec<String> = env::args().collect();

    // Check if a filename was provided
    if args.len() < 2 {
        eprintln!("Usage: {} <file>", args[0]);
        process::exit(1);
    }

    // Get the filename from command line arguments
    let file = &args[1];

    // Read the file contents
    let text = match fs::read_to_string(file) {
        Ok(text) => text,
        Err(e) => {
            eprintln!("Error reading {}: {}", file, e);
            process::exit(1);
        }
    };

    // Run the program
    match parse(&text) {
        Ok(code) => {
            let mut vm = VM::new(code);
            if let Err(e) = vm.run() {
                eprintln!("{}", e);
                process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("{}:{}:", file, e.line);
            eprintln!("{}", e.text);
            eprintln!("{}^", " ".repeat(e.caret));
            eprintln!("{}", e.msg);
            process::exit(1);
        }
    }
}
