mod ast;
mod compiler;
mod parser;
use clap::{Arg, Command};
use compiler::*;
use parser::*;
use std::fs::File;
use std::io::BufReader;
use std::process;

fn main() {
    let matches = Command::new("Verbena")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Verbena compiler")
        .arg(Arg::new("file").help("Source file").required(true).index(1))
        .arg(
            Arg::new("output")
                .short('o')
                .help("Output file (default: a.js)")
                .default_value("a.js")
                .value_name("FILE"),
        )
        .get_matches();
    let file = matches.get_one::<String>("file").unwrap();
    let output = matches.get_one::<String>("output").unwrap();
    let ast = parse(file.to_string());
    compile(&ast, output);
}
