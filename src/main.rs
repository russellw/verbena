use clap::{Arg, Command};
use std::fs::File;
use std::io::BufReader;
use std::process;
use verbena::*;

fn main() {
    // Define the command line interface using clap
    let matches = Command::new("Verbena")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Verbena language processor")
        .arg(
            Arg::new("file")
                .help("Input file to process")
                .required(true)
                .index(1),
        )
        .get_matches();

    // Get the filename from command line arguments
    // Safe to unwrap as we've marked the argument as required
    let file = matches.get_one::<String>("file").unwrap();

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
