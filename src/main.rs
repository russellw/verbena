mod parser;
mod vm;

use parser::*;
use vm::*;

fn main() {
    let text = "42";
    let code = parse(text).expect("TODO");
    let mut vm = VM::new(code);
    if let Err(err_msg) = vm.run() {
        eprintln!("{}", err_msg);
        std::process::exit(1);
    }
}
