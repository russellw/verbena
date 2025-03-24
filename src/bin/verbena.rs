use std::env;
use std::fs::File;
use std::io::BufReader;
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

    // Open the file
    let f = match File::open(file) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("{}: {}", file, e);
            process::exit(1);
        }
    };

    // Parse
    let reader = BufReader::new(f);
    let ast = match parse(file, reader) {
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
        Ok(a) => a,
    };

    // Compile to VM instructions
    let program = match compile(&ast) {
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
        Ok(a) => a,
    };

    // Run
    let mut vm = VM::new();
    if let Err(e) = vm.run(program) {
        eprintln!("{}", e);
        process::exit(1);
    }
}
