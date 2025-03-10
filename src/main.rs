mod vm;
use vm::*;

fn main() {
    let mut code = Vec::<Inst>::new();
    code.push(Inst::Add);
    let mut vm = VM::new(code);
    vm.run();
    println!("{:?}", vm);
}
