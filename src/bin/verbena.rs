use verbena::*;

fn main() {
    let text = "print \"abc\"";
    match parse(text) {
        Ok(code) => {
            let mut vm = VM::new(code);
            if let Err(err) = vm.run() {
                eprintln!("{}", err);
                std::process::exit(1);
            }
        }
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    }
}
