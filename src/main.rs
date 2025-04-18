mod ast;
mod compiler;
mod parser;
use clap::{Arg, Command};
use compiler::*;
use parser::*;

fn main() {
    let matches = Command::new("Verbena")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Verbena compiler")
        .arg(Arg::new("file").help("Source file").required(true).index(1))
        .arg(
            Arg::new("output")
                .short('o')
                .help("Output file")
                .default_value("a.mjs")
                .value_name("file"),
        )
        .get_matches();
    let file = matches.get_one::<String>("file").unwrap();
    let output = matches.get_one::<String>("output").unwrap();
    let ast = parse(file);
    compile(&ast, output);
}
