mod parser;
mod vm;

use parser::*;
use vm::*;

fn main() {
    let text = "\u{fffff}";
    match parse(text) {
        Ok(code) => {
            let mut vm = VM::new(code);
            if let Err(err_msg) = vm.run() {
                eprintln!("{}", err_msg);
                std::process::exit(1);
            }
        }
        Err(err_msg) => {
            eprintln!("{}", err_msg);
            std::process::exit(1);
        }
    }
}
