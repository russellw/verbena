use crate::error_context::*;
use crate::list::*;
use crate::object::*;
use crate::program::*;
use crate::stdlib::*;
use crate::val::*;
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct VM {
    pub rng: ChaCha20Rng,
    pub vars: HashMap<String, Val>,
}

impl Default for VM {
    fn default() -> Self {
        Self::new()
    }
}

fn index(n: usize, i: Val) -> Result<usize, String> {
    let i = i.get_isize()?;
    let i = if 0 <= i {
        let i = i as usize;
        if n <= i {
            return Err("Index out of range".to_string());
        }
        i
    } else {
        let i = n as isize + i;
        if i < 0 {
            return Err("Index out of range".to_string());
        }
        i as usize
    };
    Ok(i)
}

fn slice_index(n: usize, i: isize) -> usize {
    if 0 <= i {
        let i = i as usize;
        i.min(n)
    } else {
        let i = n as isize + i;
        let i = i.max(0);
        i as usize
    }
}

fn slice_indexes(n: usize, i: Val, j: Val) -> Result<(usize, usize), String> {
    let i = i.get_isize()?;
    let j = match j {
        Val::Null => n as isize,
        _ => j.get_isize()?,
    };
    let i = slice_index(n, i);
    let j = slice_index(n, j);
    let i = i.min(j);
    Ok((i, j))
}

fn error<S: AsRef<str>>(ec: &ErrorContext, msg: S) -> String {
    format!("{}: {}", ec, msg.as_ref())
}

fn sub(stack: &mut Vec<Val>) -> Result<(), String> {
    let b = stack.pop().unwrap().get_f64()?;
    let a = stack.pop().unwrap().get_f64()?;
    let r = Val::Num(a - b);
    stack.push(r);
    Ok(())
}

fn neg(stack: &mut Vec<Val>) -> Result<(), String> {
    let a = stack.pop().unwrap().get_f64()?;
    let r = Val::Num(-a);
    stack.push(r);
    Ok(())
}

fn div(stack: &mut Vec<Val>) -> Result<(), String> {
    let b = stack.pop().unwrap().get_f64()?;
    let a = stack.pop().unwrap().get_f64()?;
    let r = Val::Num(a / b);
    stack.push(r);
    Ok(())
}

fn pow(stack: &mut Vec<Val>) -> Result<(), String> {
    let b = stack.pop().unwrap().get_f64()?;
    let a = stack.pop().unwrap().get_f64()?;
    let r = Val::Num(a.powf(b));
    stack.push(r);
    Ok(())
}

fn bit_and(stack: &mut Vec<Val>) -> Result<(), String> {
    let b = stack.pop().unwrap().get_i64()?;
    let a = stack.pop().unwrap().get_i64()?;
    let r = a & b;
    let r = Val::Num(r as f64);
    stack.push(r);
    Ok(())
}

fn bit_or(stack: &mut Vec<Val>) -> Result<(), String> {
    let b = stack.pop().unwrap().get_i64()?;
    let a = stack.pop().unwrap().get_i64()?;
    let r = a | b;
    let r = Val::Num(r as f64);
    stack.push(r);
    Ok(())
}

fn bit_xor(stack: &mut Vec<Val>) -> Result<(), String> {
    let b = stack.pop().unwrap().get_i64()?;
    let a = stack.pop().unwrap().get_i64()?;
    let r = a ^ b;
    let r = Val::Num(r as f64);
    stack.push(r);
    Ok(())
}

fn shl(stack: &mut Vec<Val>) -> Result<(), String> {
    let b = stack.pop().unwrap().get_u32()?;
    let a = stack.pop().unwrap().get_i64()?;
    let r = a << b;
    let r = Val::Num(r as f64);
    stack.push(r);
    Ok(())
}

fn shr(stack: &mut Vec<Val>) -> Result<(), String> {
    let b = stack.pop().unwrap().get_u32()?;
    let a = stack.pop().unwrap().get_i64()?;
    let r = a >> b;
    let r = Val::Num(r as f64);
    stack.push(r);
    Ok(())
}

fn lshr(stack: &mut Vec<Val>) -> Result<(), String> {
    let b = stack.pop().unwrap().get_u32()?;
    let a = stack.pop().unwrap().get_u64()?;
    let r = a >> b;
    let r = Val::Num(r as f64);
    stack.push(r);
    Ok(())
}

fn idiv(stack: &mut Vec<Val>) -> Result<(), String> {
    let b = stack.pop().unwrap().get_i64()?;
    let a = stack.pop().unwrap().get_i64()?;
    let r = a / b;
    let r = Val::Num(r as f64);
    stack.push(r);
    Ok(())
}

fn mod_(stack: &mut Vec<Val>) -> Result<(), String> {
    let b = stack.pop().unwrap().get_f64()?;
    let a = stack.pop().unwrap().get_f64()?;
    let r = Val::Num(a % b);
    stack.push(r);
    Ok(())
}

fn bit_not(stack: &mut Vec<Val>) -> Result<(), String> {
    let a = stack.pop().unwrap().get_i64()?;
    let r = !a;
    let r = Val::Num(r as f64);
    stack.push(r);
    Ok(())
}

fn add(stack: &mut Vec<Val>) {
    let b = stack.pop().unwrap();
    let a = stack.pop().unwrap();
    let r = match (&a, &b) {
        (Val::Num(a), Val::Num(b)) => Val::Num(a + b),
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

fn mul(stack: &mut Vec<Val>) -> Result<(), String> {
    let b = stack.pop().unwrap();
    let a = stack.pop().unwrap();
    let r = match (&a, &b) {
        (Val::Num(a), Val::Num(b)) => Val::Num(*a * *b),
        (Val::Num(_), Val::Str(s)) => {
            let n = a.get_usize()?;
            Val::Str(s.repeat(n))
        }
        (Val::Str(s), Val::Num(_)) => {
            let n = b.get_usize()?;
            Val::Str(s.repeat(n))
        }
        (Val::Num(_), Val::List(v)) => {
            let n = a.get_usize()?;
            Val::List(Rc::new(RefCell::new(v.borrow().repeat(n))))
        }
        (Val::List(v), Val::Num(_)) => {
            let n = b.get_usize()?;
            Val::List(Rc::new(RefCell::new(v.borrow().repeat(n))))
        }
        _ => {
            return Err("Not numbers".to_string());
        }
    };
    stack.push(r);
    Ok(())
}

fn subscript(stack: &mut Vec<Val>) -> Result<(), String> {
    let i = stack.pop().unwrap();
    let a = stack.pop().unwrap();
    let r = match a {
        Val::List(a) => {
            let a = &a.borrow();
            let i = index(a.len(), i)?;
            a[i].clone()
        }
        Val::Object(a) => {
            let a = &a.borrow();
            let k = i.get_string()?;
            a.get(k)
        }
        Val::Str(s) => {
            let i = index(s.len(), i)?;
            let r = s.as_bytes()[i] as char;
            let r = r.to_string();
            Val::Str(r)
        }
        _ => return Err("Expected a collection".to_string()),
    };
    stack.push(r);
    Ok(())
}

fn slice(stack: &mut Vec<Val>) -> Result<(), String> {
    let j = stack.pop().unwrap();
    let i = stack.pop().unwrap();
    let a = stack.pop().unwrap();
    let r = match a {
        Val::List(a) => {
            let a = &a.borrow();
            let (i, j) = slice_indexes(a.len(), i, j)?;
            let r = &a[i..j];
            let r = r.to_vec();
            let r = List::from(r);
            Val::List(Rc::new(RefCell::new(r)))
        }
        Val::Str(s) => {
            let (i, j) = slice_indexes(s.len(), i, j)?;
            let r = &s[i..j];
            let r = r.to_string();
            Val::Str(r)
        }
        _ => return Err("Expected a sequence".to_string()),
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

    fn call(&mut self, stack: &mut Vec<Val>, n: usize) -> Result<(), String> {
        let f = stack[stack.len() - 1 - n].clone();
        let r = match f {
            Val::Func0(f) => {
                if 0 < n {
                    return Err(format!("Expected 0 args, received {}", n));
                }
                f(self)?
            }
            Val::Func1(f) => {
                if 1 < n {
                    return Err(format!("Expected 1 args, received {}", n));
                }
                let a = if 0 < n {
                    stack.pop().unwrap()
                } else {
                    Val::Null
                };
                f(self, a)?
            }
            Val::Func2(f) => {
                if 2 < n {
                    return Err(format!("Expected 2 args, received {}", n));
                }
                let b = if 1 < n {
                    stack.pop().unwrap()
                } else {
                    Val::Null
                };
                let a = if 0 < n {
                    stack.pop().unwrap()
                } else {
                    Val::Null
                };
                f(self, a, b)?
            }
            Val::Func3(f) => {
                if 3 < n {
                    return Err(format!("Expected 3 args, received {}", n));
                }
                let c = if 2 < n {
                    stack.pop().unwrap()
                } else {
                    Val::Null
                };
                let b = if 1 < n {
                    stack.pop().unwrap()
                } else {
                    Val::Null
                };
                let a = if 0 < n {
                    stack.pop().unwrap()
                } else {
                    Val::Null
                };
                f(self, a, b, c)?
            }
            Val::FuncV(f) => {
                let args = stack.split_off(stack.len() - n);
                f(self, args)?
            }
            _ => return Err("Called a non-function".to_string()),
        };
        let i = stack.len() - 1;
        stack[i] = r;
        Ok(())
    }

    pub fn run(&mut self, program: Program) -> Result<Val, String> {
        let mut stack = Vec::<Val>::new();
        let mut pc = 0usize;
        while pc < program.code.len() {
            let ec = &program.ecs[pc];
            match &program.code[pc] {
                Inst::Object(n) => {
                    let n = *n;
                    let mut r = Object::new();
                    for i in (0..n).step_by(2) {
                        let j = stack.len() - n + i;
                        let k = stack[j].clone();
                        let k = k.get_string().unwrap();
                        let x = stack[j + 1].clone();
                        r.insert(k, x);
                    }
                    let r = Val::Object(Rc::new(RefCell::new(r)));
                    stack.truncate(stack.len() - n);
                    stack.push(r);
                }
                Inst::List(n) => {
                    let r = stack.split_off(stack.len() - n);
                    let r = List::from(r);
                    let r = Val::List(Rc::new(RefCell::new(r)));
                    stack.push(r);
                }
                Inst::Call(n) => match self.call(&mut stack, *n) {
                    Ok(_) => {}
                    Err(s) => return Err(format!("{}: {}", ec, s)),
                },
                Inst::Const(a) => {
                    stack.push(a.clone());
                }
                Inst::Pop => {
                    stack.pop().unwrap();
                }
                Inst::Add => {
                    add(&mut stack);
                }
                Inst::Subscript => match subscript(&mut stack) {
                    Ok(_) => {}
                    Err(s) => return Err(format!("{}: {}", ec, s)),
                },
                Inst::Dup2Subscript => {
                    let i = stack.len() - 2;
                    let a = stack[i].clone();
                    stack.push(a);

                    let i = stack.len() - 2;
                    let a = stack[i].clone();
                    stack.push(a);

                    match subscript(&mut stack) {
                        Ok(_) => {}
                        Err(s) => return Err(format!("{}: {}", ec, s)),
                    }
                }
                Inst::Slice => match slice(&mut stack) {
                    Ok(_) => {}
                    Err(s) => return Err(format!("{}: {}", ec, s)),
                },
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
                Inst::Div => match div(&mut stack) {
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
                    let r = Val::from_bool(a == b);
                    stack.push(r);
                }
                Inst::Prin => {
                    let a = stack.pop().unwrap();
                    print!("{}", a);
                }
                Inst::Ne => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    let r = Val::from_bool(a != b);
                    stack.push(r);
                }
                Inst::Lt => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    let r = Val::from_bool(lt(&a, &b));
                    stack.push(r);
                }
                Inst::Gt => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    let r = Val::from_bool(lt(&b, &a));
                    stack.push(r);
                }
                Inst::Le => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    let r = Val::from_bool(le(&a, &b));
                    stack.push(r);
                }
                Inst::Ge => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    let r = Val::from_bool(le(&b, &a));
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
                Inst::LShr => match lshr(&mut stack) {
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
                    match a {
                        Val::List(a) => {
                            let a = &mut a.borrow_mut();
                            let i = index(a.len(), i)?;
                            a[i] = x.clone();
                        }
                        Val::Object(a) => {
                            let a = &mut a.borrow_mut();
                            let k = i.get_string()?;
                            a.insert(k, x.clone());
                        }
                        _ => return Err(error(ec, "Expected a collection")),
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
                    let a = stack.pop().unwrap();
                    return Ok(a);
                }
                Inst::Exit => {
                    let a = stack.pop().unwrap();
                    return Ok(a);
                }
            }
            pc += 1;
        }
        Ok(Val::Null)
    }
}
