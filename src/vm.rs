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

    pub fn to_bigint(&self) -> Option<BigInt> {
        match self {
            Val::Int(a) => Some(a.clone()),
            Val::Float(a) => {
                if a.is_finite() {
                    Some(BigInt::from(*a as i128))
                } else {
                    None
                }
            }
            Val::Str(s) => s.parse::<BigInt>().ok(),
        }
    }

    pub fn to_u32(&self) -> Option<u32> {
        match self {
            Val::Int(a) => a.to_u32(),
            Val::Float(a) => {
                if a.is_finite() {
                    Some(*a as u32)
                } else {
                    None
                }
            }
            Val::Str(s) => s.parse::<u32>().ok(),
        }
    }

    pub fn to_f64(&self) -> Option<f64> {
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

fn to_int(b: bool) -> Val {
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
    // Stack & Memory Operations
    Const(Val),    // Push constant onto stack
    Pop,           // Remove top value from stack
    Load(String),  // Load variable onto stack
    Store(String), // Store stack value to variable

    // Control Flow
    Br(usize),         // Unconditional branch
    BrFalse(usize),    // Branch if false
    DupBrTrue(usize),  // Duplicate and branch if true
    DupBrFalse(usize), // Duplicate and branch if false
    Exit,              // Terminate execution

    // I/O Operations
    Print, // Output value

    // Type Conversion
    ToInt,   // Convert to integer
    ToFloat, // Convert to float
    ToStr,   // Convert to string

    // Comparison Operations
    Eq, // Equal
    Ne, // Not equal
    Lt, // Less than
    Le, // Less than or equal
    Gt, // Greater than
    Ge, // Greater than or equal

    // Logical Operations
    Not, // Logical negation

    // Bitwise Operations
    BitNot, // Bitwise NOT
    BitAnd, // Bitwise AND
    BitOr,  // Bitwise OR
    BitXor, // Bitwise XOR
    Shl,    // Shift left
    Shr,    // Shift right

    // Arithmetic Operations (Integer-specific)
    IDiv, // Integer division

    // Arithmetic Operations (Float-specific)
    FDiv,  // Float division
    Floor, // Round down to nearest integer

    // Arithmetic Operations (Polymorphic)
    Add,  // Addition
    Sub,  // Subtraction
    Mul,  // Multiplication
    Mod,  // Modulo
    Neg,  // Negation
    Pow,  // Power
    Sqrt, // Square root
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

    fn top(&mut self) -> Val {
        self.stack.last().unwrap().clone()
    }

    pub fn run(&mut self) -> Result<(), String> {
        while self.pc < self.code.len() {
            let i = self.pc;
            self.pc += 1;
            let inst = self.code[i].clone();
            match inst {
                Inst::ToFloat => {
                    let a = self.pop();
                    let a = match a.to_f64() {
                        Some(a) => a,
                        None => return Err("Unable to convert value".to_string()),
                    };
                    let r = Val::Float(a);
                    self.push(r);
                }
                Inst::ToInt => {
                    let a = self.pop();
                    let a = match a.to_bigint() {
                        Some(a) => a,
                        None => return Err("Unable to convert value".to_string()),
                    };
                    let r = Val::Int(a);
                    self.push(r);
                }
                Inst::ToStr => {
                    let a = self.pop();
                    let r = Val::Str(a.to_string().into());
                    self.push(r);
                }
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
                    let r = to_int(eq(&a, &b));
                    self.push(r);
                }
                Inst::Ne => {
                    let b = self.pop();
                    let a = self.pop();
                    let r = to_int(!eq(&a, &b));
                    self.push(r);
                }
                Inst::Lt => {
                    let b = self.pop();
                    let a = self.pop();
                    let r = to_int(lt(&a, &b));
                    self.push(r);
                }
                Inst::Gt => {
                    let b = self.pop();
                    let a = self.pop();
                    let r = to_int(lt(&b, &a));
                    self.push(r);
                }
                Inst::Le => {
                    let b = self.pop();
                    let a = self.pop();
                    let r = to_int(le(&a, &b));
                    self.push(r);
                }
                Inst::Ge => {
                    let b = self.pop();
                    let a = self.pop();
                    let r = to_int(le(&b, &a));
                    self.push(r);
                }
                Inst::Sub => {
                    let b = self.pop();
                    let a = self.pop();
                    let r = match (&a, &b) {
                        (Val::Int(a), Val::Int(b)) => Val::Int(a - b),
                        _ => {
                            let a = match a.to_f64() {
                                Some(a) => a,
                                None => return Err("Expected numbers".to_string()),
                            };
                            let b = match b.to_f64() {
                                Some(b) => b,
                                None => return Err("Expected numbers".to_string()),
                            };
                            Val::Float(a - b)
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
                    let a = match a.to_f64() {
                        Some(a) => a,
                        None => return Err("Expected numbers".to_string()),
                    };
                    let b = match b.to_f64() {
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
                        _ => {
                            let a = match a.to_f64() {
                                Some(a) => a,
                                None => return Err("Expected numbers".to_string()),
                            };
                            let b = match b.to_f64() {
                                Some(b) => b,
                                None => return Err("Expected numbers".to_string()),
                            };
                            Val::Float(a.powf(b))
                        }
                    };
                    self.push(r);
                }
                Inst::BitAnd => {
                    let b = self.pop();
                    let a = self.pop();
                    let a = match a.to_bigint() {
                        Some(a) => a,
                        None => return Err("Expected integers".to_string()),
                    };
                    let b = match b.to_bigint() {
                        Some(b) => b,
                        None => return Err("Expected integers".to_string()),
                    };
                    let r = Val::Int(a & b);
                    self.push(r);
                }
                Inst::BitOr => {
                    let b = self.pop();
                    let a = self.pop();
                    let a = match a.to_bigint() {
                        Some(a) => a,
                        None => return Err("Expected integers".to_string()),
                    };
                    let b = match b.to_bigint() {
                        Some(b) => b,
                        None => return Err("Expected integers".to_string()),
                    };
                    let r = Val::Int(a | b);
                    self.push(r);
                }
                Inst::BitXor => {
                    let b = self.pop();
                    let a = self.pop();
                    let a = match a.to_bigint() {
                        Some(a) => a,
                        None => return Err("Expected integers".to_string()),
                    };
                    let b = match b.to_bigint() {
                        Some(b) => b,
                        None => return Err("Expected integers".to_string()),
                    };
                    let r = Val::Int(a ^ b);
                    self.push(r);
                }
                Inst::Shl => {
                    let b = self.pop();
                    let a = self.pop();
                    let a = match a.to_bigint() {
                        Some(a) => a,
                        None => return Err("Expected integer".to_string()),
                    };
                    let b = match b.to_u32() {
                        Some(b) => b,
                        None => return Err("Shift amount not valid".to_string()),
                    };
                    let r = Val::Int(a << b);
                    self.push(r);
                }
                Inst::Shr => {
                    let b = self.pop();
                    let a = self.pop();
                    let a = match a.to_bigint() {
                        Some(a) => a,
                        None => return Err("Expected integer".to_string()),
                    };
                    let b = match b.to_u32() {
                        Some(b) => b,
                        None => return Err("Shift amount not valid".to_string()),
                    };
                    let r = Val::Int(a >> b);
                    self.push(r);
                }
                Inst::IDiv => {
                    let b = self.pop();
                    let a = self.pop();
                    let a = match a.to_bigint() {
                        Some(a) => a,
                        None => return Err("Expected integers".to_string()),
                    };
                    let b = match b.to_bigint() {
                        Some(b) => b,
                        None => return Err("Expected integers".to_string()),
                    };
                    let r = Val::Int(a / b);
                    self.push(r);
                }
                Inst::Mod => {
                    let b = self.pop();
                    let a = self.pop();
                    let r = match (&a, &b) {
                        (Val::Int(a), Val::Int(b)) => Val::Int(a % b),
                        _ => {
                            let a = match a.to_f64() {
                                Some(a) => a,
                                None => return Err("Expected numbers".to_string()),
                            };
                            let b = match b.to_f64() {
                                Some(b) => b,
                                None => return Err("Expected numbers".to_string()),
                            };
                            Val::Float(a % b)
                        }
                    };
                    self.push(r);
                }
                Inst::BitNot => {
                    let a = self.pop();
                    let a = match a.to_bigint() {
                        Some(a) => a,
                        None => return Err("Expected integers".to_string()),
                    };
                    let r = Val::Int(!a);
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
                Inst::Pop => {
                    self.pop();
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
                Inst::DupBrFalse(target) => {
                    let a = self.top();
                    if !a.truth() {
                        self.pc = target;
                    }
                }
                Inst::DupBrTrue(target) => {
                    let a = self.top();
                    if a.truth() {
                        self.pc = target;
                    }
                }
                Inst::Not => {
                    let a = self.pop();
                    let r = to_int(!a.truth());
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
