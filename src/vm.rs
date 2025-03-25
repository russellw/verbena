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

    pub fn register0(&mut self, name: &str, f: fn(&mut VM) -> Result<Val, String>) {
        self.vars.insert(name.to_string(), Val::func0(f));
    }

    pub fn register1(&mut self, name: &str, f: fn(&mut VM, Val) -> Result<Val, String>) {
        self.vars.insert(name.to_string(), Val::func1(f));
    }

    pub fn register2(&mut self, name: &str, f: fn(&mut VM, Val, Val) -> Result<Val, String>) {
        self.vars.insert(name.to_string(), Val::func2(f));
    }

    pub fn register3(&mut self, name: &str, f: fn(&mut VM, Val, Val, Val) -> Result<Val, String>) {
        self.vars.insert(name.to_string(), Val::func3(f));
    }

    pub fn registerv(&mut self, name: &str, f: fn(&mut VM, Vec<Val>) -> Result<Val, String>) {
        self.vars.insert(name.to_string(), Val::funcv(f));
    }

    fn call1(&mut self, stack: &mut Vec<Val>, f: &Val, n: usize) -> Result<Val, String> {
        match f {
            Val::Func0(f) => {
                if n != 0 {
                    return Err(format!("Expected 0 args, received {}", n));
                }
                f(self)
            }
            Val::Func1(f) => {
                if n != 1 {
                    return Err(format!("Expected 1 args, received {}", n));
                }
                let a = stack.pop().unwrap();
                f(self, a)
            }
            Val::Func2(f) => {
                if n != 2 {
                    return Err(format!("Expected 2 args, received {}", n));
                }
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                f(self, a, b)
            }
            Val::Func3(f) => {
                if n != 3 {
                    return Err(format!("Expected 3 args, received {}", n));
                }
                let c = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                f(self, a, b, c)
            }
            Val::FuncV(f) => {
                let args = stack.split_off(stack.len() - n);
                f(self, args)
            }
            Val::List(a) => {
                // TODO arity
                let i = stack.pop().unwrap();
                let i = i.to_usize()?;
                Ok(a.borrow().v[i].clone())
            }
            Val::Str(s) => {
                let chars: Vec<char> = s.chars().collect();
                let i = stack.pop().unwrap();
                let i = i.to_usize()?;
                let r = if i < chars.len() {
                    chars[i].to_string().into()
                } else {
                    String::new().into()
                };
                Ok(Val::Str(r))
            }
            _ => Err("Called a non-function".to_string()),
        }
    }

    fn call(
        &mut self,
        stack: &mut Vec<Val>,
        ec: &ErrorContext,
        f: &Val,
        n: usize,
    ) -> Result<Val, String> {
        match self.call1(stack, f, n) {
            Ok(r) => Ok(r),
            Err(s) => Err(format!("{}: {}", ec, s)),
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
                    let r = self.call(&mut stack, ec, &f, *n)?;
                    stack.push(r);
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
                Inst::StoreAt(ec) => {
                    let x = stack.pop().unwrap();
                    let i = stack.pop().unwrap();
                    let a = stack.pop().unwrap();

                    let i = i.to_usize()?;
                    match a {
                        Val::List(a) => {
                            a.borrow_mut().v[i] = x.clone();
                        }
                        _ => return Err(error(ec, "Not a list".to_string())),
                    };
                    stack.push(x);
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
