use fastnum::decimal::Context;
use fastnum::{D256, dec256};
use num_traits::FromPrimitive;
use num_traits::ToPrimitive;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

// fastnum provides Inf, so we might as well use it
pub const NO_TRAPS: Context = Context::default().without_traps();

#[derive(Clone, Debug, PartialEq)]
pub enum Val {
    Num(D256),
    Str(Rc<String>),
}

impl Val {
    pub fn string<S: Into<String>>(s: S) -> Self {
        Val::Str(Rc::new(s.into()))
    }

    pub fn as_string(&self) -> String {
        match self {
            Val::Num(a) => a.to_string(),
            Val::Str(s) => s.to_string(),
        }
    }

    pub fn truth(&self) -> bool {
        match self {
            Val::Num(a) => !a.is_zero(),
            Val::Str(s) => s.len() != 0,
        }
    }
}

pub const ZERO: Val = Val::Num(dec256!(0).with_ctx(NO_TRAPS));
pub const ONE: Val = Val::Num(dec256!(1).with_ctx(NO_TRAPS));

fn lt(a: &Val, b: &Val) -> bool {
    match (a, b) {
        (Val::Num(a), Val::Num(b)) => a < b,
        (Val::Str(a), Val::Str(b)) => a < b,
        _ => {
            let a = a.as_string();
            let b = b.as_string();
            a < b
        }
    }
}

fn le(a: &Val, b: &Val) -> bool {
    match (a, b) {
        (Val::Num(a), Val::Num(b)) => a <= b,
        (Val::Str(a), Val::Str(b)) => a <= b,
        _ => {
            let a = a.as_string();
            let b = b.as_string();
            a <= b
        }
    }
}

impl fmt::Display for Val {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Val::Num(a) => write!(f, "{}", a),
            Val::Str(s) => write!(f, "{}", s),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Inst {
    Pow,
    Floor,
    Sqrt,
    BitNot,
    Not,
    Neg,
    Mod,
    IDiv,
    Gt,
    Ge,
    Ne,
    And,
    Or,
    Xor,
    Add,
    Shl,
    Shr,
    Sub,
    Mul,
    Div,
    BrFalse(usize),
    Br(usize),
    Exit,
    Const(Val),
    Print,
    Eq,
    Lt,
    Le,
    Load(String),
    Store(String),
}

#[derive(Debug)]
pub struct VM {
    code: Vec<Inst>,
    pc: usize,
    stack: Vec<Val>,
    vars: HashMap<String, Val>,
}

impl VM {
    pub fn new(code: Vec<Inst>) -> Self {
        VM {
            code,
            pc: 0,
            stack: Vec::new(),
            vars: HashMap::new(),
        }
    }

    fn push(&mut self, val: Val) {
        self.stack.push(val);
    }

    fn pop(&mut self) -> Val {
        self.stack.pop().unwrap()
    }

    pub fn run(&mut self) -> Result<(), String> {
        while self.pc < self.code.len() {
            let i = self.pc;
            self.pc += 1;
            let inst = self.code[i].clone();
            match inst {
                Inst::Print => {
                    let a = self.pop();
                    print!("{}", a)
                }
                Inst::Const(a) => {
                    self.push(a.clone());
                }
                Inst::Add => {
                    let b = self.pop();
                    let a = self.pop();
                    let r = match (&a, &b) {
                        (Val::Num(a), Val::Num(b)) => Val::Num(*a + *b),
                        (Val::Str(a), Val::Str(b)) => {
                            let mut r = String::with_capacity(a.len() + b.len());
                            r.push_str(a);
                            r.push_str(b);
                            Val::string(r)
                        }
                        _ => {
                            let a = a.as_string();
                            let b = b.as_string();
                            let mut r = String::with_capacity(a.len() + b.len());
                            r.push_str(&a);
                            r.push_str(&b);
                            Val::string(r)
                        }
                    };
                    self.push(r);
                }
                Inst::Eq => {
                    let b = self.pop();
                    let a = self.pop();
                    let r = if a == b { ONE } else { ZERO };
                    self.push(r);
                }
                Inst::Ne => {
                    let b = self.pop();
                    let a = self.pop();
                    let r = if a != b { ONE } else { ZERO };
                    self.push(r);
                }
                Inst::Lt => {
                    let b = self.pop();
                    let a = self.pop();
                    let r = if lt(&a, &b) { ONE } else { ZERO };
                    self.push(r);
                }
                Inst::Gt => {
                    let b = self.pop();
                    let a = self.pop();
                    let r = if lt(&b, &a) { ONE } else { ZERO };
                    self.push(r);
                }
                Inst::Le => {
                    let b = self.pop();
                    let a = self.pop();
                    let r = if le(&a, &b) { ONE } else { ZERO };
                    self.push(r);
                }
                Inst::Ge => {
                    let b = self.pop();
                    let a = self.pop();
                    let r = if le(&b, &a) { ONE } else { ZERO };
                    self.push(r);
                }
                Inst::Sub => {
                    let b = self.pop();
                    let a = self.pop();
                    let r = match (&a, &b) {
                        (Val::Num(a), Val::Num(b)) => Val::Num(*a - *b),
                        _ => {
                            return Err("-: expected numbers".to_string());
                        }
                    };
                    self.push(r);
                }
                Inst::Neg => {
                    let a = self.pop();
                    let r = match &a {
                        Val::Num(a) => Val::Num(-*a),
                        _ => {
                            return Err("Expected number".to_string());
                        }
                    };
                    self.push(r);
                }
                Inst::Div => {
                    let b = self.pop();
                    let a = self.pop();
                    let r = match (&a, &b) {
                        (Val::Num(a), Val::Num(b)) => Val::Num(*a / *b),
                        _ => {
                            return Err("/: expected numbers".to_string());
                        }
                    };
                    self.push(r);
                }
                Inst::Pow => {
                    let b = self.pop();
                    let a = self.pop();
                    let r = match (&a, &b) {
                        (Val::Num(a), Val::Num(b)) => Val::Num(a.pow(*b)),
                        _ => {
                            return Err("Expected numbers".to_string());
                        }
                    };
                    self.push(r);
                }
                Inst::And => {
                    let b = self.pop();
                    let a = self.pop();
                    let r = match (&a, &b) {
                        (Val::Num(a), Val::Num(b)) => {
                            let a = match a.to_i128() {
                                Some(a) => a,
                                None => return Err(format!("Cannot convert {} to integer", a)),
                            };
                            let b = match b.to_i128() {
                                Some(b) => b,
                                None => return Err(format!("Cannot convert {} to integer", b)),
                            };
                            let r = a & b;
                            Val::Num(D256::from_i128(r).unwrap())
                        }
                        _ => {
                            return Err("Expected numbers".to_string());
                        }
                    };
                    self.push(r);
                }
                Inst::Or => {
                    let b = self.pop();
                    let a = self.pop();
                    let r = match (&a, &b) {
                        (Val::Num(a), Val::Num(b)) => {
                            let a = match a.to_i128() {
                                Some(a) => a,
                                None => return Err(format!("Cannot convert {} to integer", a)),
                            };
                            let b = match b.to_i128() {
                                Some(b) => b,
                                None => return Err(format!("Cannot convert {} to integer", b)),
                            };
                            let r = a | b;
                            Val::Num(D256::from_i128(r).unwrap())
                        }
                        _ => {
                            return Err("Expected numbers".to_string());
                        }
                    };
                    self.push(r);
                }
                Inst::Xor => {
                    let b = self.pop();
                    let a = self.pop();
                    let r = match (&a, &b) {
                        (Val::Num(a), Val::Num(b)) => {
                            let a = match a.to_i128() {
                                Some(a) => a,
                                None => return Err(format!("Cannot convert {} to integer", a)),
                            };
                            let b = match b.to_i128() {
                                Some(b) => b,
                                None => return Err(format!("Cannot convert {} to integer", b)),
                            };
                            let r = a ^ b;
                            Val::Num(D256::from_i128(r).unwrap())
                        }
                        _ => {
                            return Err("Expected numbers".to_string());
                        }
                    };
                    self.push(r);
                }
                Inst::Shl => {
                    let b = self.pop();
                    let a = self.pop();
                    let r = match (&a, &b) {
                        (Val::Num(a), Val::Num(b)) => {
                            let a = match a.to_i128() {
                                Some(a) => a,
                                None => return Err(format!("Cannot convert {} to integer", a)),
                            };
                            let b = match b.to_i128() {
                                Some(b) => b,
                                None => return Err(format!("Cannot convert {} to integer", b)),
                            };
                            let r = a << b;
                            Val::Num(D256::from_i128(r).unwrap())
                        }
                        _ => {
                            return Err("Expected numbers".to_string());
                        }
                    };
                    self.push(r);
                }
                Inst::Shr => {
                    let b = self.pop();
                    let a = self.pop();
                    let r = match (&a, &b) {
                        (Val::Num(a), Val::Num(b)) => {
                            let a = match a.to_i128() {
                                Some(a) => a,
                                None => return Err(format!("Cannot convert {} to integer", a)),
                            };
                            let b = match b.to_i128() {
                                Some(b) => b,
                                None => return Err(format!("Cannot convert {} to integer", b)),
                            };
                            let r = a >> b;
                            Val::Num(D256::from_i128(r).unwrap())
                        }
                        _ => {
                            return Err("Expected numbers".to_string());
                        }
                    };
                    self.push(r);
                }
                Inst::IDiv => {
                    let b = self.pop();
                    let a = self.pop();
                    let r = match (&a, &b) {
                        (Val::Num(a), Val::Num(b)) => {
                            let a = match a.to_i128() {
                                Some(a) => a,
                                None => return Err(format!("Cannot convert {} to integer", a)),
                            };
                            let b = match b.to_i128() {
                                Some(b) => b,
                                None => return Err(format!("Cannot convert {} to integer", b)),
                            };
                            let r = a / b;
                            Val::Num(D256::from_i128(r).unwrap())
                        }
                        _ => {
                            return Err("Expected numbers".to_string());
                        }
                    };
                    self.push(r);
                }
                Inst::Mod => {
                    let b = self.pop();
                    let a = self.pop();
                    let r = match (&a, &b) {
                        (Val::Num(a), Val::Num(b)) => {
                            let a = match a.to_i128() {
                                Some(a) => a,
                                None => return Err(format!("Cannot convert {} to integer", a)),
                            };
                            let b = match b.to_i128() {
                                Some(b) => b,
                                None => return Err(format!("Cannot convert {} to integer", b)),
                            };
                            let r = a % b;
                            Val::Num(D256::from_i128(r).unwrap())
                        }
                        _ => {
                            return Err("Expected numbers".to_string());
                        }
                    };
                    self.push(r);
                }
                Inst::BitNot => {
                    let a = self.pop();
                    let r = match &a {
                        Val::Num(a) => {
                            let a = match a.to_i128() {
                                Some(a) => a,
                                None => return Err(format!("Cannot convert {} to integer", a)),
                            };
                            let r = !a;
                            Val::Num(D256::from_i128(r).unwrap())
                        }
                        _ => {
                            return Err("Expected number".to_string());
                        }
                    };
                    self.push(r);
                }
                Inst::Sqrt => {
                    let a = self.pop();
                    let r = match &a {
                        Val::Num(a) => Val::Num(a.sqrt().reduce()),
                        _ => {
                            return Err("Expected number".to_string());
                        }
                    };
                    self.push(r);
                }
                Inst::Floor => {
                    let a = self.pop();
                    let r = match &a {
                        Val::Num(a) => Val::Num(a.floor()),
                        _ => {
                            return Err("Expected number".to_string());
                        }
                    };
                    self.push(r);
                }
                Inst::BrFalse(target) => {
                    let a = self.pop();
                    if !a.truth() {
                        self.pc = target;
                    }
                }
                Inst::Not => {
                    let a = self.pop();
                    let r = if !a.truth() { ONE } else { ZERO };
                    self.push(r);
                }
                Inst::Load(name) => {
                    let a = match self.vars.get(&name) {
                        Some(a) => a,
                        None => {
                            return Err(format!("'{}' is not defined", name));
                        }
                    };
                    self.push(a.clone());
                }
                Inst::Store(name) => {
                    let a = self.pop();
                    self.vars.insert(name.clone(), a);
                }
                Inst::Mul => {
                    let b = self.pop();
                    let a = self.pop();
                    let r = match (&a, &b) {
                        (Val::Num(a), Val::Num(b)) => Val::Num(*a * *b),
                        (Val::Num(a), Val::Str(b)) => {
                            let n = match usize::try_from(*a) {
                                Ok(n) => n,
                                Err(_) => {
                                    return Err(
                                        "*: cannot convert number to repeat count".to_string()
                                    );
                                }
                            };
                            Val::string(b.repeat(n))
                        }
                        (Val::Str(a), Val::Num(b)) => {
                            let n = match usize::try_from(*b) {
                                Ok(n) => n,
                                Err(_) => {
                                    return Err(
                                        "*: cannot convert number to repeat count".to_string()
                                    );
                                }
                            };
                            Val::string(a.repeat(n))
                        }
                        _ => {
                            return Err("*: expected numbers".to_string());
                        }
                    };
                    self.push(r);
                }
                Inst::Br(target) => {
                    self.pc = target;
                }
                Inst::Exit => {
                    return Ok(());
                }
            }
        }
        Ok(())
    }
}
