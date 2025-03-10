use fastnum::dec256;

mod vm;
use vm::*;

fn main() {
    let mut code = Vec::<Inst>::new();
    code.push(Inst::Const(Val::Num(dec256!(1))));
    code.push(Inst::Const(Val::Num(dec256!(2))));
    code.push(Inst::Add);
    code.push(Inst::End);
    let mut vm = VM::new(code);
    if let Err(err_msg) = vm.run() {
        eprintln!("{}", err_msg);
        std::process::exit(1);
    }
}
