use num_bigint::BigInt;
use num_traits::One;
use num_traits::ToPrimitive;
use num_traits::Zero;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub enum Val {
    Int(BigInt),
    Float(f64),
    Str(Rc<String>),
}

impl Val {
    pub fn string<S: Into<String>>(s: S) -> Self {
        Val::Str(Rc::new(s.into()))
    }

    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Val::Int(a) => a.to_f64(),
            Val::Float(a) => Some(*a),
            Val::Str(s) => s.parse::<f64>().ok(),
        }
    }

    pub fn truth(&self) -> bool {
        match self {
            Val::Int(a) => !a.is_zero(),
            Val::Float(a) => *a != 0.0,
            Val::Str(s) => !s.is_empty(),
        }
    }
}

fn eq(a: &Val, b: &Val) -> bool {
    match (a, b) {
        (Val::Int(a), Val::Float(b)) => {
            let a = match a.to_f64() {
                Some(a) => a,
                None => return false,
            };
            a == *b
        }
        (Val::Float(a), Val::Int(b)) => {
            let b = match b.to_f64() {
                Some(b) => b,
                None => return false,
            };
            *a == b
        }
        _ => a == b,
    }
}

fn lt(a: &Val, b: &Val) -> bool {
    match (a, b) {
        (Val::Int(a), Val::Int(b)) => a < b,
        (Val::Float(a), Val::Float(b)) => a < b,
        (Val::Int(a), Val::Float(b)) => {
            let a = match a.to_f64() {
                Some(a) => a,
                None => return false,
            };
            a < *b
        }
        (Val::Float(a), Val::Int(b)) => {
            let b = match b.to_f64() {
                Some(b) => b,
                None => return false,
            };
            *a < b
        }
        _ => {
            let a = a.to_string();
            let b = b.to_string();
            a < b
        }
    }
}

fn le(a: &Val, b: &Val) -> bool {
    match (a, b) {
        (Val::Int(a), Val::Int(b)) => a <= b,
        (Val::Float(a), Val::Float(b)) => a <= b,
        (Val::Int(a), Val::Float(b)) => {
            let a = match a.to_f64() {
                Some(a) => a,
                None => return false,
            };
            a <= *b
        }
        (Val::Float(a), Val::Int(b)) => {
            let b = match b.to_f64() {
                Some(b) => b,
                None => return false,
            };
            *a <= b
        }
        _ => {
            let a = a.to_string();
            let b = b.to_string();
            a <= b
        }
    }
}

fn as_int(b: bool) -> Val {
    Val::Int(if b { BigInt::one() } else { BigInt::zero() })
}

impl fmt::Display for Val {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Val::Int(a) => write!(f, "{}", a),
            Val::Float(a) => write!(f, "{}", a),
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
    BitAnd,
    BitOr,
    BitXor,
    Add,
    Shl,
    Shr,
    Sub,
    Mul,
    FDiv,
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
                        (Val::Int(a), Val::Int(b)) => Val::Int(a + b),
                        (Val::Float(a), Val::Float(b)) => Val::Float(a + b),
                        (Val::Int(a), Val::Float(b)) => {
                            let a = match a.to_f64() {
                                Some(a) => a,
                                None => return Err("Expected numbers".to_string()),
                            };
                            Val::Float(a + *b)
                        }
                        (Val::Float(a), Val::Int(b)) => {
                            let b = match b.to_f64() {
                                Some(b) => b,
                                None => return Err("Expected numbers".to_string()),
                            };
                            Val::Float(*a + b)
                        }
                        _ => {
                            let a = a.to_string();
                            let b = b.to_string();
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
                    let r = as_int(eq(&a, &b));
                    self.push(r);
                }
                Inst::Ne => {
                    let b = self.pop();
                    let a = self.pop();
                    let r = as_int(!eq(&a, &b));
                    self.push(r);
                }
                Inst::Lt => {
                    let b = self.pop();
                    let a = self.pop();
                    let r = as_int(lt(&a, &b));
                    self.push(r);
                }
                Inst::Gt => {
                    let b = self.pop();
                    let a = self.pop();
                    let r = as_int(lt(&b, &a));
                    self.push(r);
                }
                Inst::Le => {
                    let b = self.pop();
                    let a = self.pop();
                    let r = as_int(le(&a, &b));
                    self.push(r);
                }
                Inst::Ge => {
                    let b = self.pop();
                    let a = self.pop();
                    let r = as_int(le(&b, &a));
                    self.push(r);
                }
                Inst::Sub => {
                    let b = self.pop();
                    let a = self.pop();
                    let r = match (&a, &b) {
                        (Val::Int(a), Val::Int(b)) => Val::Int(a - b),
                        (Val::Float(a), Val::Float(b)) => Val::Float(a - b),
                        (Val::Int(a), Val::Float(b)) => {
                            let a = match a.to_f64() {
                                Some(a) => a,
                                None => return Err("Expected numbers".to_string()),
                            };
                            Val::Float(a - *b)
                        }
                        (Val::Float(a), Val::Int(b)) => {
                            let b = match b.to_f64() {
                                Some(b) => b,
                                None => return Err("Expected numbers".to_string()),
                            };
                            Val::Float(*a - b)
                        }
                        _ => {
                            return Err("-: expected numbers".to_string());
                        }
                    };
                    self.push(r);
                }
                Inst::Neg => {
                    let a = self.pop();
                    let r = match &a {
                        Val::Int(a) => Val::Int(-a),
                        Val::Float(a) => Val::Float(-a),
                        _ => {
                            return Err("Expected number".to_string());
                        }
                    };
                    self.push(r);
                }
                Inst::FDiv => {
                    let b = self.pop();
                    let a = self.pop();
                    let a = match a.as_f64() {
                        Some(a) => a,
                        None => return Err("Expected numbers".to_string()),
                    };
                    let b = match b.as_f64() {
                        Some(b) => b,
                        None => return Err("Expected numbers".to_string()),
                    };
                    let r = Val::Float(a / b);
                    self.push(r);
                }
                Inst::Pow => {
                    let b = self.pop();
                    let a = self.pop();
                    let r = match (&a, &b) {
                        (Val::Int(a), Val::Int(b)) => match b.to_u32() {
                            Some(b) => Val::Int(a.pow(b)),
                            None => {
                                return Err("Exponent out of range".to_string());
                            }
                        },
                        (Val::Float(a), Val::Float(b)) => Val::Float(a.powf(*b)),
                        (Val::Int(a), Val::Float(b)) => {
                            let a = match a.to_f64() {
                                Some(a) => a,
                                None => return Err("Expected numbers".to_string()),
                            };
                            Val::Float(a.powf(*b))
                        }
                        (Val::Float(a), Val::Int(b)) => {
                            let b = match b.to_f64() {
                                Some(b) => b,
                                None => return Err("Expected numbers".to_string()),
                            };
                            Val::Float(a.powf(b))
                        }
                        _ => {
                            return Err("Expected numbers".to_string());
                        }
                    };
                    self.push(r);
                }
                Inst::BitAnd => {
                    let b = self.pop();
                    let a = self.pop();
                    let r = match (&a, &b) {
                        (Val::Int(a), Val::Int(b)) => Val::Int(a & b),
                        _ => {
                            return Err("Expected numbers".to_string());
                        }
                    };
                    self.push(r);
                }
                Inst::BitOr => {
                    let b = self.pop();
                    let a = self.pop();
                    let r = match (&a, &b) {
                        (Val::Int(a), Val::Int(b)) => Val::Int(a | b),
                        _ => {
                            return Err("Expected numbers".to_string());
                        }
                    };
                    self.push(r);
                }
                Inst::BitXor => {
                    let b = self.pop();
                    let a = self.pop();
                    let r = match (&a, &b) {
                        (Val::Int(a), Val::Int(b)) => Val::Int(a ^ b),
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
                        (Val::Int(a), Val::Int(b)) => match b.to_u32() {
                            Some(b) => Val::Int(a << b),
                            None => return Err("Shift amount out of range".to_string()),
                        },
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
                        (Val::Int(a), Val::Int(b)) => match b.to_u32() {
                            Some(b) => Val::Int(a >> b),
                            None => return Err("Shift amount out of range".to_string()),
                        },
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
                        (Val::Int(a), Val::Int(b)) => Val::Int(a / b),
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
                        (Val::Int(a), Val::Int(b)) => Val::Int(a % b),
                        _ => {
                            return Err("Expected numbers".to_string());
                        }
                    };
                    self.push(r);
                }
                Inst::BitNot => {
                    let a = self.pop();
                    let r = match &a {
                        Val::Int(a) => Val::Int(!a),
                        _ => {
                            return Err("Expected number".to_string());
                        }
                    };
                    self.push(r);
                }
                Inst::Sqrt => {
                    let a = self.pop();
                    let r = match &a {
                        Val::Float(a) => Val::Float(a.sqrt()),
                        Val::Int(a) => Val::Int(a.sqrt()),
                        _ => {
                            return Err("Expected number".to_string());
                        }
                    };
                    self.push(r);
                }
                Inst::Floor => {
                    let a = self.pop();
                    let r = match &a {
                        Val::Float(a) => Val::Float(a.floor()),
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
                    let r = as_int(!a.truth());
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
                        (Val::Int(a), Val::Int(b)) => Val::Int(a.clone() * b.clone()),
                        (Val::Int(a), Val::Float(b)) => match a.to_f64() {
                            Some(a) => Val::Float(a * b),
                            None => return Err("Integer too large to convert to float".to_string()),
                        },
                        (Val::Float(a), Val::Int(b)) => match b.to_f64() {
                            Some(b) => Val::Float(a * b),
                            None => return Err("Integer too large to convert to float".to_string()),
                        },
                        (Val::Float(a), Val::Float(b)) => Val::Float(*a * *b),
                        (Val::Int(a), Val::Str(b)) => {
                            let a = match usize::try_from(a.clone()) {
                                Ok(a) => a,
                                Err(_) => {
                                    return Err("Repeat count out of range".to_string());
                                }
                            };
                            Val::string(b.repeat(a))
                        }
                        (Val::Str(a), Val::Int(b)) => {
                            let b = match usize::try_from(b.clone()) {
                                Ok(b) => b,
                                Err(_) => {
                                    return Err("Repeat count out of range".to_string());
                                }
                            };
                            Val::string(a.repeat(b))
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
