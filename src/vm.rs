use crate::error_context::*;
use crate::program::*;
use crate::stdlib::*;
use crate::val::*;
use num_bigint::BigInt;
use num_traits::Zero;
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;
use std::collections::HashMap;

pub struct VM {
    pub rng: ChaCha20Rng,
    pub vars: HashMap<String, Val>,
}

fn error<S: AsRef<str>>(ec: &ErrorContext, msg: S) -> String {
    format!("{}:{}: {}", ec.file, ec.line, msg.as_ref())
}

impl VM {
    pub fn new() -> Self {
        let mut vm = VM {
            rng: ChaCha20Rng::seed_from_u64(0),
            vars: HashMap::new(),
        };
        register_all(&mut vm);
        vm
    }

    fn call(&mut self, f: &Val, n: usize) -> Result<Val, String> {
        match f {
            Val::Func(f) => {
                if n != 0 {
                    return Err(format!("Expected 0 args, received {}", n));
                }
                f(self)
            }
            _ => Err("Called a non-function".to_string()),
        }
    }

    pub fn run(&mut self, program: Program) -> Result<Val, String> {
        let mut stack = Vec::<Val>::new();
        let mut pc = 0usize;
        while pc < program.code.len() {
            match &program.code[pc] {
                Inst::Call(ec, name, n) => {
                    let f = match self.vars.get(name) {
                        Some(a) => a.clone(),
                        None => {
                            return Err(error(ec, format!("'{}' is not defined", name)));
                        }
                    };
                    let _r = self.call(&f, *n);
                }
                Inst::Const(a) => {
                    stack.push(a.clone());
                }
                Inst::Pop => {
                    stack.pop().unwrap();
                }
                Inst::BrFalse(target) => {
                    let a = stack.pop().unwrap();
                    if !a.truth() {
                        pc = *target;
                        continue;
                    }
                }
                Inst::DupBrFalse(target) => {
                    let a = stack.last().unwrap().clone();
                    if !a.truth() {
                        pc = *target;
                        continue;
                    }
                }
                Inst::DupBrTrue(target) => {
                    let a = stack.last().unwrap().clone();
                    if a.truth() {
                        pc = *target;
                        continue;
                    }
                }
                Inst::Load(ec, name) => {
                    let a = match self.vars.get(name) {
                        Some(a) => a,
                        None => {
                            return Err(error(ec, format!("'{}' is not defined", name)));
                        }
                    };
                    stack.push(a.clone());
                }
                Inst::Store(name) => {
                    let a = stack.pop().unwrap();
                    self.vars.insert(name.clone(), a);
                }
                Inst::Br(target) => {
                    pc = *target;
                    continue;
                }
                Inst::Return => {
                    return Ok(Val::Int(BigInt::zero()));
                }
                Inst::Exit => {
                    let a = stack.pop().unwrap();
                    return Ok(a);
                }
            }
            pc += 1;
        }
        Ok(Val::Int(BigInt::zero()))
    }
}
