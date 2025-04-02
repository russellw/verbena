mod parser;

use clap::{Arg, Command};
use parser::*;
use std::fs::File;
use std::io::BufReader;
use std::process;

fn main() {
    // Define the command line interface using clap
    let matches = Command::new("Verbena")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Verbena compiler")
        .arg(Arg::new("file").help("Source file").required(true).index(1))
        .get_matches();

    // Get the filename from command line arguments
    // Safe to unwrap as we've marked the argument as required
    let file = matches.get_one::<String>("file").unwrap();

    // Parse
    let ast = parse(file);

    // Compile to VM instructions
    compile(&ast);
}
