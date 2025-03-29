use crate::error_context::*;
use crate::program::*;
use crate::stdlib::*;
use crate::val::*;
use num_bigint::BigInt;
use num_traits::Zero;
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct VM {
    pub rng: ChaCha20Rng,
    pub vars: HashMap<String, Val>,
}

fn slice_index(n: usize, i: isize) -> usize {
    let i = if i < 0 { (n as isize + i).max(0) } else { i } as usize;
    i.min(n)
}

fn slice_indexes(n: usize, i: Val, j: Val) -> Result<(usize, usize), String> {
    let i = match i {
        Val::Null => 0,
        _ => i.to_isize()?,
    };
    let j = match j {
        Val::Null => n as isize,
        _ => j.to_isize()?,
    };
    Ok((slice_index(n, i), slice_index(n, j)))
}

fn error<S: AsRef<str>>(ec: &ErrorContext, msg: S) -> String {
    format!("{}: {}", ec, msg.as_ref())
}

fn sub(stack: &mut Vec<Val>) -> Result<(), String> {
    let b = stack.pop().unwrap();
    let a = stack.pop().unwrap();
    let (a, b) = num2(&a, &b)?;
    let r = match (&a, &b) {
        (Val::Int(a), Val::Int(b)) => Val::Int(a - b),
        (Val::Float(a), Val::Float(b)) => Val::Float(a - b),
        _ => panic!(),
    };
    stack.push(r);
    Ok(())
}

fn neg(stack: &mut Vec<Val>) -> Result<(), String> {
    let a = stack.pop().unwrap();
    let a = a.num()?;
    let r = match a {
        Val::Int(a) => Val::Int(-a),
        Val::Float(a) => Val::Float(-a),
        _ => panic!(),
    };
    stack.push(r);
    Ok(())
}

fn fdiv(stack: &mut Vec<Val>) -> Result<(), String> {
    let b = stack.pop().unwrap();
    let a = stack.pop().unwrap();
    let a = a.to_f64()?;
    let b = b.to_f64()?;
    let r = Val::Float(a / b);
    stack.push(r);
    Ok(())
}

fn pow(stack: &mut Vec<Val>) -> Result<(), String> {
    let b = stack.pop().unwrap();
    let a = stack.pop().unwrap();
    let (a, b) = num2(&a, &b)?;
    let r = match (&a, &b) {
        (Val::Int(a), Val::Int(_)) => {
            let b = b.to_u32()?;
            Val::Int(a.pow(b))
        }
        (Val::Float(a), Val::Float(b)) => Val::Float(a.powf(*b)),
        _ => panic!(),
    };
    stack.push(r);
    Ok(())
}

fn bit_and(stack: &mut Vec<Val>) -> Result<(), String> {
    let b = stack.pop().unwrap();
    let a = stack.pop().unwrap();
    let a = a.to_bigint()?;
    let b = b.to_bigint()?;
    let r = Val::Int(a & b);
    stack.push(r);
    Ok(())
}

fn bit_or(stack: &mut Vec<Val>) -> Result<(), String> {
    let b = stack.pop().unwrap();
    let a = stack.pop().unwrap();
    let a = a.to_bigint()?;
    let b = b.to_bigint()?;
    let r = Val::Int(a | b);
    stack.push(r);
    Ok(())
}

fn bit_xor(stack: &mut Vec<Val>) -> Result<(), String> {
    let b = stack.pop().unwrap();
    let a = stack.pop().unwrap();
    let a = a.to_bigint()?;
    let b = b.to_bigint()?;
    let r = Val::Int(a ^ b);
    stack.push(r);
    Ok(())
}

fn shl(stack: &mut Vec<Val>) -> Result<(), String> {
    let b = stack.pop().unwrap();
    let a = stack.pop().unwrap();
    let a = a.to_bigint()?;
    let b = b.to_u32()?;
    let r = Val::Int(a << b);
    stack.push(r);
    Ok(())
}

fn shr(stack: &mut Vec<Val>) -> Result<(), String> {
    let b = stack.pop().unwrap();
    let a = stack.pop().unwrap();
    let a = a.to_bigint()?;
    let b = b.to_u32()?;
    let r = Val::Int(a >> b);
    stack.push(r);
    Ok(())
}

fn idiv(stack: &mut Vec<Val>) -> Result<(), String> {
    let b = stack.pop().unwrap();
    let a = stack.pop().unwrap();
    let a = a.to_bigint()?;
    let b = b.to_bigint()?;
    let r = Val::Int(a / b);
    stack.push(r);
    Ok(())
}

fn mod_(stack: &mut Vec<Val>) -> Result<(), String> {
    let b = stack.pop().unwrap();
    let a = stack.pop().unwrap();
    let (a, b) = num2(&a, &b)?;
    let r = match (&a, &b) {
        (Val::Int(a), Val::Int(b)) => Val::Int(a % b),
        (Val::Float(a), Val::Float(b)) => Val::Float(a % b),
        _ => panic!(),
    };
    stack.push(r);
    Ok(())
}

fn bit_not(stack: &mut Vec<Val>) -> Result<(), String> {
    let a = stack.pop().unwrap();
    let a = a.to_bigint()?;
    let r = Val::Int(!a);
    stack.push(r);
    Ok(())
}

fn mul(stack: &mut Vec<Val>) -> Result<(), String> {
    let b = stack.pop().unwrap();
    let a = stack.pop().unwrap();
    let (a, b) = num2_loose(&a, &b);
    let r = match (&a, &b) {
        (Val::Int(a), Val::Int(b)) => Val::Int(a.clone() * b.clone()),
        (Val::Float(a), Val::Float(b)) => Val::Float(*a * *b),
        (Val::Int(_), Val::Str(s)) => {
            let n = a.to_usize()?;
            Val::Str(s.repeat(n))
        }
        (Val::Str(s), Val::Int(_)) => {
            let n = b.to_usize()?;
            Val::Str(s.repeat(n))
        }
        (Val::Int(_), Val::List(v)) => {
            let n = a.to_usize()?;
            Val::List(Rc::new(RefCell::new(v.borrow().repeat(n))))
        }
        (Val::List(v), Val::Int(_)) => {
            let n = b.to_usize()?;
            Val::List(Rc::new(RefCell::new(v.borrow().repeat(n))))
        }
        _ => {
            return Err("Not numbers".to_string());
        }
    };
    stack.push(r);
    Ok(())
}

impl VM {
    pub fn new() -> Self {
        let mut vm = VM {
            rng: ChaCha20Rng::seed_from_u64(0),
            vars: HashMap::new(),
        };
        vm.register("inf", Val::Float(std::f64::INFINITY));
        vm.register("nan", Val::Float(std::f64::NAN));
        vm.register("pi", Val::Float(std::f64::consts::PI));
        register_all(&mut vm);
        vm
    }

    pub fn register(&mut self, name: &str, a: Val) {
        self.vars.insert(name.to_string(), a);
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
            Val::Str(s) => match n {
                1 => {
                    let i = stack.pop().unwrap();
                    let i = i.to_isize()?;
                    let i = slice_index(s.len(), i);
                    let c = s.as_bytes()[i] as char;
                    let s = c.to_string();
                    Ok(Val::Str(s))
                }
                2 => {
                    let j = stack.pop().unwrap();
                    let i = stack.pop().unwrap();
                    let (i, j) = slice_indexes(s.len(), i, j)?;
                    let s = &s[i..j];
                    Ok(Val::Str(s.to_string()))
                }
                _ => Err("String expects 1 or 2 indexes".to_string()),
            },
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
            let ec = &program.ecs[pc];
            match &program.code[pc] {
                Inst::Call(name, n) => {
                    let f = match self.vars.get(name) {
                        Some(a) => a.clone(),
                        None => {
                            return Err(error(ec, format!("'{}' is not defined", name)));
                        }
                    };
                    let r = self.call(&mut stack, ec, &f, *n)?;
                    stack.push(r);
                }
                Inst::CallIndirect(n) => {
                    let f = stack[stack.len() - 1 - n].clone();
                    let r = self.call(&mut stack, ec, &f, *n)?;
                    let i = stack.len() - 1;
                    stack[i] = r;
                }
                Inst::Const(a) => {
                    stack.push(a.clone());
                }
                Inst::Pop => {
                    stack.pop().unwrap();
                }
                Inst::Add => {
                    // TODO: fn?
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    let (a, b) = num2_loose(&a, &b);
                    let r = match (&a, &b) {
                        (Val::Int(a), Val::Int(b)) => Val::Int(a + b),
                        (Val::Float(a), Val::Float(b)) => Val::Float(a + b),
                        _ => {
                            let a = a.to_string();
                            let b = b.to_string();
                            let mut r = String::with_capacity(a.len() + b.len());
                            r.push_str(&a);
                            r.push_str(&b);
                            Val::Str(r)
                        }
                    };
                    stack.push(r);
                }
                Inst::Sub => match sub(&mut stack) {
                    Ok(_) => {}
                    Err(s) => return Err(format!("{}: {}", ec, s)),
                },
                Inst::Mul => match mul(&mut stack) {
                    Ok(_) => {}
                    Err(s) => return Err(format!("{}: {}", ec, s)),
                },
                Inst::IDiv => match idiv(&mut stack) {
                    Ok(_) => {}
                    Err(s) => return Err(format!("{}: {}", ec, s)),
                },
                Inst::FDiv => match fdiv(&mut stack) {
                    Ok(_) => {}
                    Err(s) => return Err(format!("{}: {}", ec, s)),
                },
                Inst::Mod => match mod_(&mut stack) {
                    Ok(_) => {}
                    Err(s) => return Err(format!("{}: {}", ec, s)),
                },
                Inst::Eq => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    let r = Val::from_bool(eq_loose(&a, &b));
                    stack.push(r);
                }
                Inst::Ne => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    let r = Val::from_bool(!eq_loose(&a, &b));
                    stack.push(r);
                }
                Inst::Lt => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    let r = Val::from_bool(lt_loose(&a, &b));
                    stack.push(r);
                }
                Inst::Gt => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    let r = Val::from_bool(lt_loose(&b, &a));
                    stack.push(r);
                }
                Inst::Le => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    let r = Val::from_bool(le_loose(&a, &b));
                    stack.push(r);
                }
                Inst::Ge => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    let r = Val::from_bool(le_loose(&b, &a));
                    stack.push(r);
                }
                Inst::Shl => match shl(&mut stack) {
                    Ok(_) => {}
                    Err(s) => return Err(format!("{}: {}", ec, s)),
                },
                Inst::Shr => match shr(&mut stack) {
                    Ok(_) => {}
                    Err(s) => return Err(format!("{}: {}", ec, s)),
                },
                Inst::BitAnd => match bit_and(&mut stack) {
                    Ok(_) => {}
                    Err(s) => return Err(format!("{}: {}", ec, s)),
                },
                Inst::BitXor => match bit_xor(&mut stack) {
                    Ok(_) => {}
                    Err(s) => return Err(format!("{}: {}", ec, s)),
                },
                Inst::BitOr => match bit_or(&mut stack) {
                    Ok(_) => {}
                    Err(s) => return Err(format!("{}: {}", ec, s)),
                },
                Inst::Pow => match pow(&mut stack) {
                    Ok(_) => {}
                    Err(s) => return Err(format!("{}: {}", ec, s)),
                },
                Inst::Neg => match neg(&mut stack) {
                    Ok(_) => {}
                    Err(s) => return Err(format!("{}: {}", ec, s)),
                },
                Inst::Not => {
                    let a = stack.pop().unwrap();
                    let r = Val::from_bool(!a.truth());
                    stack.push(r);
                }
                Inst::BitNot => match bit_not(&mut stack) {
                    Ok(_) => {}
                    Err(s) => return Err(format!("{}: {}", ec, s)),
                },
                Inst::BrFalse(target) => {
                    let cond = stack.pop().unwrap();
                    if !cond.truth() {
                        pc = *target;
                        continue;
                    }
                }
                Inst::BrTrue(target) => {
                    let cond = stack.pop().unwrap();
                    if cond.truth() {
                        pc = *target;
                        continue;
                    }
                }
                Inst::Assert(msg) => {
                    let cond = stack.pop().unwrap();
                    if !cond.truth() {
                        return Err(error(ec, msg));
                    }
                }
                Inst::DupBrFalse(target) => {
                    let cond = stack.last().unwrap().clone();
                    if !cond.truth() {
                        pc = *target;
                        continue;
                    }
                }
                Inst::DupBrTrue(target) => {
                    let cond = stack.last().unwrap().clone();
                    if cond.truth() {
                        pc = *target;
                        continue;
                    }
                }
                Inst::Load(name) => {
                    let a = match self.vars.get(name) {
                        Some(a) => a,
                        None => {
                            return Err(error(ec, format!("'{}' is not defined", name)));
                        }
                    };
                    stack.push(a.clone());
                }
                Inst::StoreAt => {
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
                    let a = stack.last().unwrap().clone();
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
