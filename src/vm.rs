use crate::error::*;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::One;
use num_traits::Signed;
use num_traits::ToPrimitive;
use num_traits::Zero;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub enum Val {
    Int(BigInt),
    Float(f64),
    Str(Rc<String>),
    List(Rc<RefCell<List>>),
}

#[derive(Debug, PartialEq)]
pub struct List {
    v: Vec<Val>,
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
            _ => None,
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
            _ => None,
        }
    }

    pub fn to_i32(&self) -> Option<i32> {
        match self {
            Val::Int(a) => a.to_i32(),
            Val::Float(a) => {
                if a.is_finite() {
                    Some(*a as i32)
                } else {
                    None
                }
            }
            Val::Str(s) => s.parse::<i32>().ok(),
            _ => None,
        }
    }

    pub fn to_u64(&self) -> Option<u64> {
        match self {
            Val::Int(a) => a.to_u64(),
            Val::Float(a) => {
                if a.is_finite() {
                    Some(*a as u64)
                } else {
                    None
                }
            }
            Val::Str(s) => s.parse::<u64>().ok(),
            _ => None,
        }
    }

    pub fn to_usize(&self) -> Option<usize> {
        match self {
            Val::Int(a) => a.to_usize(),
            Val::Float(a) => {
                if a.is_finite() {
                    Some(*a as usize)
                } else {
                    None
                }
            }
            Val::Str(s) => s.parse::<usize>().ok(),
            _ => None,
        }
    }

    pub fn to_f64(&self) -> Option<f64> {
        match self {
            Val::Int(a) => a.to_f64(),
            Val::Float(a) => Some(*a),
            Val::Str(s) => s.parse::<f64>().ok(),
            _ => None,
        }
    }

    pub fn truth(&self) -> bool {
        match self {
            Val::Int(a) => !a.is_zero(),
            Val::Float(a) => *a != 0.0,
            Val::Str(s) => !s.is_empty(),
            Val::List(a) => !a.borrow().v.is_empty(),
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
            Val::List(a) => write!(f, "{}", a.borrow()),
        }
    }
}

impl List {
    fn new(n: usize) -> Self {
        let default_val = Val::Int(BigInt::zero());
        List {
            v: vec![default_val; n],
        }
    }
}

impl From<Vec<Val>> for List {
    fn from(v: Vec<Val>) -> Self {
        List { v }
    }
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for (i, a) in self.v.iter().enumerate() {
            if 0 < i {
                write!(f, ", ")?;
            }
            write!(f, "{}", a)?;
        }
        write!(f, "]")
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
    Assert,

    // I/O Operations
    Print, // Output value

    // Type Conversion
    ToInt,   // Convert to integer
    ToFloat, // Convert to float
    ToStr,   // Convert to string
    StrBase,
    ValBase,

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
    NthRoot,
    TrailingZeros,
    Bit,
    SetBit,
    Gcd,
    Lcm,

    // Arithmetic Operations (Float-specific)
    FDiv,  // Float division
    Floor, // Round down to nearest integer
    Ceil,
    Round,
    RoundTiesEven,
    Trunc,
    Fract,
    MulAdd,
    DivEuclid,
    RemEuclid,
    PowI,
    Exp,
    Exp2,
    Ln,
    Log,
    Log2,
    Log10,
    Hypot,
    Sin,
    Cos,
    Tan,
    ASin,
    ACos,
    ATan,
    ATan2,
    ExpM1,
    Ln1P,
    SinH,
    CosH,
    TanH,
    ASinH,
    ACosH,
    ATanH,
    IsNan,
    IsFinite,
    IsInfinite,
    IsSubnormal,
    IsNormal,
    IsSignPositive,
    IsSignNegative,
    Recip,
    ToDegrees,
    ToRadians,

    // Arithmetic Operations (Polymorphic)
    Add,  // Addition
    Sub,  // Subtraction
    Mul,  // Multiplication
    Mod,  // Modulo
    Neg,  // Negation
    Pow,  // Power
    Sqrt, // Square root
    Cbrt,
    Max,
    Min,
    Midpoint,
    TotalCmp,
    Clamp,
    Abs,
    Signum,
    CopySign,

    // String Operations
    Len,
    Left,
    Right,
    Mid,
    Asc,
    Chr,
    Instr,
    UCase,
    LCase,

    // List Operations
    Dim(String),
    List(usize),
    Subscript,
}

pub struct Program {
    // These two vectors parallel each other
    // carets[i] is the error location in the input text
    // if an error occurs while executing code[i]
    carets: Vec<usize>,
    code: Vec<Inst>,
}

impl Program {
    pub fn new(carets: Vec<usize>, code: Vec<Inst>) -> Self {
        assert_eq!(carets.len(), code.len());
        Program { carets, code }
    }
}

pub struct Process {
    program: Program,
    pc: usize,
    stack: Vec<Val>,
    vars: HashMap<String, Val>,
}

impl Process {
    pub fn new(program: Program) -> Self {
        Process {
            program,
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

    fn err<S: AsRef<str>>(&self, msg: S) -> Error {
        Error {
            caret: self.program.carets[self.pc],
            msg: msg.as_ref().to_string(),
        }
    }

    pub fn run(&mut self) -> Result<(), Error> {
        while self.pc < self.program.code.len() {
            let inst = self.program.code[self.pc].clone();
            // TODO: blank lines
            // TODO: parameter names
            match inst {
                Inst::ToFloat => {
                    let a = self.pop();
                    let a = match a.to_f64() {
                        Some(a) => a,
                        // TODO: consistent messages
                        None => return Err(self.err("Unable to convert value")),
                    };
                    let r = Val::Float(a);
                    self.push(r);
                }
                Inst::ToInt => {
                    let a = self.pop();
                    let a = match a.to_bigint() {
                        Some(a) => a,
                        None => return Err(self.err("Unable to convert value")),
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
                                None => return Err(self.err("Expected number")),
                            };
                            Val::Float(a + *b)
                        }
                        (Val::Float(a), Val::Int(b)) => {
                            let b = match b.to_f64() {
                                Some(b) => b,
                                None => return Err(self.err("Expected number")),
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
                                None => return Err(self.err("Expected number")),
                            };
                            let b = match b.to_f64() {
                                Some(b) => b,
                                None => return Err(self.err("Expected number")),
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
                            return Err(self.err("Expected number"));
                        }
                    };
                    self.push(r);
                }
                Inst::FDiv => {
                    let b = self.pop();
                    let a = self.pop();
                    let a = match a.to_f64() {
                        Some(a) => a,
                        None => return Err(self.err("Expected number")),
                    };
                    let b = match b.to_f64() {
                        Some(b) => b,
                        None => return Err(self.err("Expected number")),
                    };
                    let r = Val::Float(a / b);
                    self.push(r);
                }
                Inst::TotalCmp => {
                    let b = self.pop();
                    let a = self.pop();
                    let a = match a.to_f64() {
                        Some(a) => a,
                        None => return Err(self.err("Expected number")),
                    };
                    let b = match b.to_f64() {
                        Some(b) => b,
                        None => return Err(self.err("Expected number")),
                    };
                    let cmp_result = match a.total_cmp(&b) {
                        std::cmp::Ordering::Less => BigInt::from(-1),
                        std::cmp::Ordering::Equal => BigInt::from(0),
                        std::cmp::Ordering::Greater => BigInt::from(1),
                    };
                    let r = Val::Int(cmp_result);
                    self.push(r);
                }
                Inst::CopySign => {
                    let sign = self.pop();
                    let a = self.pop();
                    let a = match a.to_f64() {
                        Some(a) => a,
                        None => return Err(self.err("Expected number")),
                    };
                    let sign = match sign.to_f64() {
                        Some(sign) => sign,
                        None => return Err(self.err("Expected number")),
                    };
                    let r = Val::Float(a.copysign(sign));
                    self.push(r);
                }
                Inst::Midpoint => {
                    let b = self.pop();
                    let a = self.pop();
                    let a = match a.to_f64() {
                        Some(a) => a,
                        None => return Err(self.err("Expected number")),
                    };
                    let b = match b.to_f64() {
                        Some(b) => b,
                        None => return Err(self.err("Expected number")),
                    };
                    let r = Val::Float(a.midpoint(b));
                    self.push(r);
                }
                Inst::Pow => {
                    let b = self.pop();
                    let a = self.pop();
                    let r = match (&a, &b) {
                        (Val::Int(a), Val::Int(b)) => match b.to_u32() {
                            Some(b) => Val::Int(a.pow(b)),
                            None => {
                                return Err(self.err("Exponent out of range"));
                            }
                        },
                        _ => {
                            let a = match a.to_f64() {
                                Some(a) => a,
                                None => return Err(self.err("Expected number")),
                            };
                            let b = match b.to_f64() {
                                Some(b) => b,
                                None => return Err(self.err("Expected number")),
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
                        None => return Err(self.err("Expected integers")),
                    };
                    let b = match b.to_bigint() {
                        Some(b) => b,
                        None => return Err(self.err("Expected integers")),
                    };
                    let r = Val::Int(a & b);
                    self.push(r);
                }
                Inst::BitOr => {
                    let b = self.pop();
                    let a = self.pop();
                    let a = match a.to_bigint() {
                        Some(a) => a,
                        None => return Err(self.err("Expected integers")),
                    };
                    let b = match b.to_bigint() {
                        Some(b) => b,
                        None => return Err(self.err("Expected integers")),
                    };
                    let r = Val::Int(a | b);
                    self.push(r);
                }
                Inst::BitXor => {
                    let b = self.pop();
                    let a = self.pop();
                    let a = match a.to_bigint() {
                        Some(a) => a,
                        None => return Err(self.err("Expected integers")),
                    };
                    let b = match b.to_bigint() {
                        Some(b) => b,
                        None => return Err(self.err("Expected integers")),
                    };
                    let r = Val::Int(a ^ b);
                    self.push(r);
                }
                Inst::Gcd => {
                    let b = self.pop();
                    let a = self.pop();
                    let a = match a.to_bigint() {
                        Some(a) => a,
                        None => return Err(self.err("Expected integers")),
                    };
                    let b = match b.to_bigint() {
                        Some(b) => b,
                        None => return Err(self.err("Expected integers")),
                    };
                    let r = Val::Int(a.gcd(&b));
                    self.push(r);
                }
                Inst::Lcm => {
                    let b = self.pop();
                    let a = self.pop();
                    let a = match a.to_bigint() {
                        Some(a) => a,
                        None => return Err(self.err("Expected integers")),
                    };
                    let b = match b.to_bigint() {
                        Some(b) => b,
                        None => return Err(self.err("Expected integers")),
                    };
                    let r = Val::Int(a.lcm(&b));
                    self.push(r);
                }
                Inst::Shl => {
                    let b = self.pop();
                    let a = self.pop();

                    let a = match a.to_bigint() {
                        Some(a) => a,
                        None => return Err(self.err("Expected integer")),
                    };
                    let b = match b.to_u32() {
                        Some(b) => b,
                        None => return Err(self.err("Shift amount not valid")),
                    };

                    let r = Val::Int(a << b);
                    self.push(r);
                }
                Inst::StrBase => {
                    let base = self.pop();
                    let a = self.pop();

                    let a = match a.to_bigint() {
                        Some(a) => a,
                        None => return Err(self.err("Expected integer")),
                    };
                    let base = match base.to_u32() {
                        Some(base) => base,
                        None => return Err(self.err("Shift amount not valid")),
                    };

                    let r = Val::Str(a.to_str_radix(base).into());
                    self.push(r);
                }
                Inst::ValBase => {
                    let base = self.pop();
                    let s = self.pop();

                    let s = s.to_string();
                    let base = match base.to_u32() {
                        Some(base) => base,
                        None => return Err(self.err("Shift amount not valid")),
                    };

                    let r = match BigInt::parse_bytes(s.as_bytes(), base) {
                        Some(a) => Val::Int(a),
                        None => return Err(self.err("Unable to parse string")),
                    };
                    self.push(r);
                }
                Inst::Shr => {
                    let b = self.pop();
                    let a = self.pop();

                    let a = match a.to_bigint() {
                        Some(a) => a,
                        None => return Err(self.err("Expected integer")),
                    };
                    let b = match b.to_u32() {
                        Some(b) => b,
                        None => return Err(self.err("Shift amount not valid")),
                    };

                    let r = Val::Int(a >> b);
                    self.push(r);
                }
                Inst::IDiv => {
                    let b = self.pop();
                    let a = self.pop();
                    let a = match a.to_bigint() {
                        Some(a) => a,
                        None => return Err(self.err("Expected integers")),
                    };
                    let b = match b.to_bigint() {
                        Some(b) => b,
                        None => return Err(self.err("Expected integers")),
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
                                None => return Err(self.err("Expected number")),
                            };
                            let b = match b.to_f64() {
                                Some(b) => b,
                                None => return Err(self.err("Expected number")),
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
                        None => return Err(self.err("Expected integers")),
                    };
                    let r = Val::Int(!a);
                    self.push(r);
                }
                Inst::Signum => {
                    let a = self.pop();
                    let r = match &a {
                        Val::Float(a) => Val::Float(a.signum()),
                        Val::Int(a) => Val::Int(a.signum()),
                        _ => {
                            return Err(self.err("Expected number"));
                        }
                    };
                    self.push(r);
                }
                Inst::Abs => {
                    let a = self.pop();
                    let r = match &a {
                        Val::Float(a) => Val::Float(a.abs()),
                        Val::Int(a) => Val::Int(a.abs()),
                        _ => {
                            return Err(self.err("Expected number"));
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
                            return Err(self.err("Expected number"));
                        }
                    };
                    self.push(r);
                }
                Inst::Cbrt => {
                    let a = self.pop();
                    let r = match &a {
                        Val::Float(a) => Val::Float(a.cbrt()),
                        Val::Int(a) => Val::Int(a.cbrt()),
                        _ => {
                            return Err(self.err("Expected number"));
                        }
                    };
                    self.push(r);
                }
                Inst::Pop => {
                    self.pop();
                }
                Inst::BrFalse(target) => {
                    let a = self.pop();
                    if !a.truth() {
                        self.pc = target;
                        continue;
                    }
                }
                Inst::DupBrFalse(target) => {
                    let a = self.top();
                    if !a.truth() {
                        self.pc = target;
                        continue;
                    }
                }
                Inst::DupBrTrue(target) => {
                    let a = self.top();
                    if a.truth() {
                        self.pc = target;
                        continue;
                    }
                }
                Inst::Assert => {
                    let a = self.pop();
                    if !a.truth() {
                        return Err(self.err("Assert failed"));
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
                            return Err(self.err(format!("'{}' is not defined", name)));
                        }
                    };
                    self.push(a.clone());
                }
                Inst::Store(name) => {
                    let a = self.pop();
                    self.vars.insert(name.clone(), a);
                }
                Inst::List(n) => {
                    let drained = self
                        .stack
                        .drain(self.stack.len() - n..)
                        .collect::<Vec<Val>>();
                    let r = List::from(drained);
                    let r = Val::List(Rc::new(RefCell::new(r)));
                    self.push(r);
                }
                Inst::Dim(name) => {
                    let n = self.pop();
                    let n = match n.to_usize() {
                        Some(n) => n,
                        None => return Err(self.err("Expected integer length")),
                    };
                    let r = List::new(n + 1);
                    let r = Val::List(Rc::new(RefCell::new(r)));
                    self.vars.insert(name.clone(), r);
                }
                Inst::Subscript => {
                    let i = self.pop();
                    let a = self.pop();

                    let i = match i.to_usize() {
                        Some(i) => i,
                        None => return Err(self.err("Invalid index")),
                    };
                    let r = match a {
                        Val::List(a) => a.borrow().v[i].clone(),
                        _ => return Err(self.err("Invalid list")),
                    };

                    self.push(r);
                }
                Inst::Mul => {
                    let b = self.pop();
                    let a = self.pop();
                    let r = match (&a, &b) {
                        (Val::Int(a), Val::Int(b)) => Val::Int(a.clone() * b.clone()),
                        (Val::Int(a), Val::Float(b)) => match a.to_f64() {
                            Some(a) => Val::Float(a * b),
                            None => return Err(self.err("Integer too large to convert to float")),
                        },
                        (Val::Float(a), Val::Int(b)) => match b.to_f64() {
                            Some(b) => Val::Float(a * b),
                            None => return Err(self.err("Integer too large to convert to float")),
                        },
                        (Val::Float(a), Val::Float(b)) => Val::Float(*a * *b),
                        (Val::Int(a), Val::Str(b)) => {
                            let a = match usize::try_from(a.clone()) {
                                Ok(a) => a,
                                Err(_) => {
                                    return Err(self.err("Repeat count out of range"));
                                }
                            };
                            Val::string(b.repeat(a))
                        }
                        (Val::Str(a), Val::Int(b)) => {
                            let b = match usize::try_from(b.clone()) {
                                Ok(b) => b,
                                Err(_) => {
                                    return Err(self.err("Repeat count out of range"));
                                }
                            };
                            Val::string(a.repeat(b))
                        }
                        _ => {
                            return Err(self.err("*: expected numbers"));
                        }
                    };
                    self.push(r);
                }
                Inst::Br(target) => {
                    self.pc = target;
                    continue;
                }
                Inst::Exit => {
                    return Ok(());
                }
                Inst::Floor => {
                    let a = self.pop();
                    let a = match a.to_f64() {
                        Some(a) => a,
                        None => return Err(self.err("Expected number")),
                    };
                    let r = Val::Float(a.floor());
                    self.push(r);
                }
                Inst::Ceil => {
                    let a = self.pop();
                    let a = match a.to_f64() {
                        Some(a) => a,
                        None => return Err(self.err("Expected number")),
                    };
                    let r = Val::Float(a.ceil());
                    self.push(r);
                }
                Inst::Round => {
                    let a = self.pop();
                    let a = match a.to_f64() {
                        Some(a) => a,
                        None => return Err(self.err("Expected number")),
                    };
                    let r = Val::Float(a.round());
                    self.push(r);
                }
                Inst::RoundTiesEven => {
                    let a = self.pop();
                    let a = match a.to_f64() {
                        Some(a) => a,
                        None => return Err(self.err("Expected number")),
                    };
                    let r = Val::Float(a.round_ties_even());
                    self.push(r);
                }
                Inst::Trunc => {
                    let a = self.pop();
                    let a = match a.to_f64() {
                        Some(a) => a,
                        None => return Err(self.err("Expected number")),
                    };
                    let r = Val::Float(a.trunc());
                    self.push(r);
                }
                Inst::Fract => {
                    let a = self.pop();
                    let a = match a.to_f64() {
                        Some(a) => a,
                        None => return Err(self.err("Expected number")),
                    };
                    let r = Val::Float(a.fract());
                    self.push(r);
                }
                Inst::MulAdd => {
                    let c = self.pop();
                    let b = self.pop();
                    let a = self.pop();
                    let a = match a.to_f64() {
                        Some(a) => a,
                        None => return Err(self.err("Expected number for first argument")),
                    };
                    let b = match b.to_f64() {
                        Some(b) => b,
                        None => return Err(self.err("Expected number for second argument")),
                    };
                    let c = match c.to_f64() {
                        Some(c) => c,
                        None => return Err(self.err("Expected number for third argument")),
                    };
                    let r = Val::Float(a.mul_add(b, c)); // a * b + c
                    self.push(r);
                }
                Inst::DivEuclid => {
                    let b = self.pop();
                    let a = self.pop();
                    let a = match a.to_f64() {
                        Some(a) => a,
                        None => return Err(self.err("Expected number for dividend")),
                    };
                    let b = match b.to_f64() {
                        Some(b) => b,
                        None => return Err(self.err("Expected number for divisor")),
                    };
                    let r = Val::Float(a.div_euclid(b));
                    self.push(r);
                }
                Inst::RemEuclid => {
                    let b = self.pop();
                    let a = self.pop();
                    let a = match a.to_f64() {
                        Some(a) => a,
                        None => return Err(self.err("Expected number for dividend")),
                    };
                    let b = match b.to_f64() {
                        Some(b) => b,
                        None => return Err(self.err("Expected number for divisor")),
                    };
                    let r = Val::Float(a.rem_euclid(b));
                    self.push(r);
                }
                Inst::PowI => {
                    let b = self.pop();
                    let a = self.pop();
                    let a = match a.to_f64() {
                        Some(a) => a,
                        None => return Err(self.err("Expected number for base")),
                    };
                    let b = match b.to_i32() {
                        Some(b) => b,
                        None => return Err(self.err("Expected integer for exponent")),
                    };
                    let r = Val::Float(a.powi(b));
                    self.push(r);
                }
                Inst::Exp => {
                    let a = self.pop();
                    let a = match a.to_f64() {
                        Some(a) => a,
                        None => return Err(self.err("Expected number")),
                    };
                    let r = Val::Float(a.exp());
                    self.push(r);
                }
                Inst::Exp2 => {
                    let a = self.pop();
                    let a = match a.to_f64() {
                        Some(a) => a,
                        None => return Err(self.err("Expected number")),
                    };
                    let r = Val::Float(a.exp2());
                    self.push(r);
                }
                Inst::Ln => {
                    let a = self.pop();
                    let a = match a.to_f64() {
                        Some(a) => a,
                        None => return Err(self.err("Expected number")),
                    };
                    let r = Val::Float(a.ln());
                    self.push(r);
                }
                Inst::Log => {
                    let b = self.pop();
                    let a = self.pop();
                    let a = match a.to_f64() {
                        Some(a) => a,
                        None => return Err(self.err("Expected number for value")),
                    };
                    let b = match b.to_f64() {
                        Some(b) => b,
                        None => return Err(self.err("Expected number for base")),
                    };
                    let r = Val::Float(a.log(b));
                    self.push(r);
                }
                Inst::Log2 => {
                    let a = self.pop();
                    let a = match a.to_f64() {
                        Some(a) => a,
                        None => return Err(self.err("Expected number")),
                    };
                    let r = Val::Float(a.log2());
                    self.push(r);
                }
                Inst::Log10 => {
                    let a = self.pop();
                    let a = match a.to_f64() {
                        Some(a) => a,
                        None => return Err(self.err("Expected number")),
                    };
                    let r = Val::Float(a.log10());
                    self.push(r);
                }
                Inst::Hypot => {
                    let b = self.pop();
                    let a = self.pop();
                    let a = match a.to_f64() {
                        Some(a) => a,
                        None => return Err(self.err("Expected number for first argument")),
                    };
                    let b = match b.to_f64() {
                        Some(b) => b,
                        None => return Err(self.err("Expected number for second argument")),
                    };
                    let r = Val::Float(a.hypot(b));
                    self.push(r);
                }
                Inst::Sin => {
                    let a = self.pop();
                    let a = match a.to_f64() {
                        Some(a) => a,
                        None => return Err(self.err("Expected number")),
                    };
                    let r = Val::Float(a.sin());
                    self.push(r);
                }
                Inst::Cos => {
                    let a = self.pop();
                    let a = match a.to_f64() {
                        Some(a) => a,
                        None => return Err(self.err("Expected number")),
                    };
                    let r = Val::Float(a.cos());
                    self.push(r);
                }
                Inst::Tan => {
                    let a = self.pop();
                    let a = match a.to_f64() {
                        Some(a) => a,
                        None => return Err(self.err("Expected number")),
                    };
                    let r = Val::Float(a.tan());
                    self.push(r);
                }
                Inst::ASin => {
                    let a = self.pop();
                    let a = match a.to_f64() {
                        Some(a) => a,
                        None => return Err(self.err("Expected number")),
                    };
                    let r = Val::Float(a.asin());
                    self.push(r);
                }
                Inst::ACos => {
                    let a = self.pop();
                    let a = match a.to_f64() {
                        Some(a) => a,
                        None => return Err(self.err("Expected number")),
                    };
                    let r = Val::Float(a.acos());
                    self.push(r);
                }
                Inst::ATan => {
                    let a = self.pop();
                    let a = match a.to_f64() {
                        Some(a) => a,
                        None => return Err(self.err("Expected number")),
                    };
                    let r = Val::Float(a.atan());
                    self.push(r);
                }
                Inst::ATan2 => {
                    let b = self.pop();
                    let a = self.pop();
                    let a = match a.to_f64() {
                        Some(a) => a,
                        None => return Err(self.err("Expected number for first argument")),
                    };
                    let b = match b.to_f64() {
                        Some(b) => b,
                        None => return Err(self.err("Expected number for second argument")),
                    };
                    let r = Val::Float(a.atan2(b));
                    self.push(r);
                }
                Inst::ExpM1 => {
                    let a = self.pop();
                    let a = match a.to_f64() {
                        Some(a) => a,
                        None => return Err(self.err("Expected number")),
                    };
                    let r = Val::Float(a.exp_m1());
                    self.push(r);
                }
                Inst::Ln1P => {
                    let a = self.pop();
                    let a = match a.to_f64() {
                        Some(a) => a,
                        None => return Err(self.err("Expected number")),
                    };
                    let r = Val::Float(a.ln_1p());
                    self.push(r);
                }
                Inst::SinH => {
                    let a = self.pop();
                    let a = match a.to_f64() {
                        Some(a) => a,
                        None => return Err(self.err("Expected number")),
                    };
                    let r = Val::Float(a.sinh());
                    self.push(r);
                }
                Inst::CosH => {
                    let a = self.pop();
                    let a = match a.to_f64() {
                        Some(a) => a,
                        None => return Err(self.err("Expected number")),
                    };
                    let r = Val::Float(a.cosh());
                    self.push(r);
                }
                Inst::TanH => {
                    let a = self.pop();
                    let a = match a.to_f64() {
                        Some(a) => a,
                        None => return Err(self.err("Expected number")),
                    };
                    let r = Val::Float(a.tanh());
                    self.push(r);
                }
                Inst::ASinH => {
                    let a = self.pop();
                    let a = match a.to_f64() {
                        Some(a) => a,
                        None => return Err(self.err("Expected number")),
                    };
                    let r = Val::Float(a.asinh());
                    self.push(r);
                }
                Inst::ACosH => {
                    let a = self.pop();
                    let a = match a.to_f64() {
                        Some(a) => a,
                        None => return Err(self.err("Expected number")),
                    };
                    let r = Val::Float(a.acosh());
                    self.push(r);
                }
                Inst::ATanH => {
                    let a = self.pop();
                    let a = match a.to_f64() {
                        Some(a) => a,
                        None => return Err(self.err("Expected number")),
                    };
                    let r = Val::Float(a.atanh());
                    self.push(r);
                }
                Inst::IsNan => {
                    let a = self.pop();
                    let a = match a.to_f64() {
                        Some(a) => a,
                        None => return Err(self.err("Expected number")),
                    };
                    let r = to_int(a.is_nan());
                    self.push(r);
                }
                Inst::IsFinite => {
                    let a = self.pop();
                    let a = match a.to_f64() {
                        Some(a) => a,
                        None => return Err(self.err("Expected number")),
                    };
                    let r = to_int(a.is_finite());
                    self.push(r);
                }
                Inst::IsInfinite => {
                    let a = self.pop();
                    let a = match a.to_f64() {
                        Some(a) => a,
                        None => return Err(self.err("Expected number")),
                    };
                    let r = to_int(a.is_infinite());
                    self.push(r);
                }
                Inst::IsSubnormal => {
                    let a = self.pop();
                    let a = match a.to_f64() {
                        Some(a) => a,
                        None => return Err(self.err("Expected number")),
                    };
                    let r = to_int(a.is_subnormal());
                    self.push(r);
                }
                Inst::IsNormal => {
                    let a = self.pop();
                    let a = match a.to_f64() {
                        Some(a) => a,
                        None => return Err(self.err("Expected number")),
                    };
                    let r = to_int(a.is_normal());
                    self.push(r);
                }
                Inst::IsSignPositive => {
                    let a = self.pop();
                    let a = match a.to_f64() {
                        Some(a) => a,
                        None => return Err(self.err("Expected number")),
                    };
                    let r = to_int(a.is_sign_positive());
                    self.push(r);
                }
                Inst::IsSignNegative => {
                    let a = self.pop();
                    let a = match a.to_f64() {
                        Some(a) => a,
                        None => return Err(self.err("Expected number")),
                    };
                    let r = to_int(a.is_sign_negative());
                    self.push(r);
                }
                Inst::Recip => {
                    let a = self.pop();
                    let a = match a.to_f64() {
                        Some(a) => a,
                        None => return Err(self.err("Expected number")),
                    };
                    let r = Val::Float(a.recip());
                    self.push(r);
                }
                Inst::ToDegrees => {
                    let a = self.pop();
                    let a = match a.to_f64() {
                        Some(a) => a,
                        None => return Err(self.err("Expected number")),
                    };
                    let r = Val::Float(a.to_degrees());
                    self.push(r);
                }
                Inst::ToRadians => {
                    let a = self.pop();
                    let a = match a.to_f64() {
                        Some(a) => a,
                        None => return Err(self.err("Expected number")),
                    };
                    let r = Val::Float(a.to_radians());
                    self.push(r);
                }

                Inst::NthRoot => {
                    let b = self.pop();
                    let a = self.pop();
                    let a = match a.to_bigint() {
                        Some(a) => a,
                        None => return Err(self.err("Expected integer")),
                    };
                    let b = match b.to_u32() {
                        Some(b) => b,
                        None => return Err(self.err("N out of range")),
                    };
                    let r = Val::Int(a.nth_root(b));
                    self.push(r);
                }
                Inst::TrailingZeros => {
                    let a = self.pop();
                    let a = match a.to_bigint() {
                        Some(a) => a,
                        None => return Err(self.err("Expected integer")),
                    };
                    let r = match a.trailing_zeros() {
                        Some(r) => r,
                        None => 0,
                    };
                    let r = Val::Int(BigInt::from(r));
                    self.push(r);
                }
                Inst::Bit => {
                    let b = self.pop();
                    let a = self.pop();
                    let a = match a.to_bigint() {
                        Some(a) => a,
                        None => return Err(self.err("Expected integer")),
                    };
                    let b = match b.to_u64() {
                        Some(b) => b,
                        None => return Err(self.err("Bit out of range")),
                    };
                    let r = to_int(a.bit(b));
                    self.push(r);
                }
                Inst::SetBit => {
                    let value = self.pop();
                    let bit = self.pop();
                    let a = self.pop();

                    let a = match a.to_bigint() {
                        Some(a) => a,
                        None => return Err(self.err("Expected integer")),
                    };
                    let bit = match bit.to_u64() {
                        Some(bit) => bit,
                        None => return Err(self.err("Bit out of range")),
                    };
                    let value = value.truth();

                    let mut r = a.clone();
                    r.set_bit(bit, value);

                    let r = Val::Int(r);
                    self.push(r);
                }

                Inst::Max => {
                    let b = self.pop();
                    let a = self.pop();
                    let r = match (&a, &b) {
                        (Val::Float(a), Val::Float(b)) => Val::Float(a.max(*b)),
                        _ => {
                            if lt(&b, &a) {
                                a
                            } else {
                                b
                            }
                        }
                    };
                    self.push(r);
                }
                Inst::Min => {
                    let b = self.pop();
                    let a = self.pop();
                    let r = match (&a, &b) {
                        (Val::Float(a), Val::Float(b)) => Val::Float(a.min(*b)),
                        _ => {
                            if lt(&a, &b) {
                                a
                            } else {
                                b
                            }
                        }
                    };
                    self.push(r);
                }
                Inst::Clamp => {
                    let max = self.pop();
                    let min = self.pop();
                    let a = self.pop();
                    let r = match (&a, &min, &max) {
                        (Val::Int(a), Val::Int(min), Val::Int(max)) => {
                            let r = if a < min {
                                min
                            } else {
                                if max < a { max } else { a }
                            };
                            Val::Int(r.clone())
                        }
                        _ => {
                            let a = match a.to_f64() {
                                Some(a) => a,
                                None => return Err(self.err("Expected number")),
                            };
                            let min = match min.to_f64() {
                                Some(min) => min,
                                None => return Err(self.err("Expected number")),
                            };
                            let max = match max.to_f64() {
                                Some(max) => max,
                                None => return Err(self.err("Expected number")),
                            };
                            Val::Float(a.clamp(min, max))
                        }
                    };
                    self.push(r);
                }
                Inst::Len => {
                    let a = self.pop();
                    let len = match &a {
                        Val::Str(s) => s.len(),
                        _ => a.to_string().len(),
                    };
                    self.push(Val::Int(BigInt::from(len)));
                }

                Inst::Left => {
                    let n = self.pop();
                    let s = self.pop();

                    let s = s.to_string();
                    let n = match n.to_u64() {
                        Some(n) => n as usize,
                        None => return Err(self.err("Expected non-negative integer for length")),
                    };

                    let result = if n >= s.len() {
                        s
                    } else {
                        // Safe to index since we verified n < s.len()
                        s[..n].to_string()
                    };

                    self.push(Val::string(result));
                }

                Inst::Right => {
                    let n = self.pop();
                    let s = self.pop();

                    let s = s.to_string();
                    let n = match n.to_u64() {
                        Some(n) => n as usize,
                        None => return Err(self.err("Expected non-negative integer for length")),
                    };

                    let result = if n >= s.len() {
                        s
                    } else {
                        // Safe to index since we verified n < s.len()
                        s[s.len() - n..].to_string()
                    };

                    self.push(Val::string(result));
                }

                Inst::Mid => {
                    let len = self.pop();
                    let start = self.pop();
                    let s = self.pop();

                    let s = s.to_string();

                    // In BASIC, string indices are typically 1-based
                    let start = match start.to_u64() {
                        Some(start) => start as usize,
                        None => {
                            return Err(
                                self.err("Expected non-negative integer for start position")
                            );
                        }
                    };

                    // Adjust to 0-based indexing
                    let start = if start > 0 { start - 1 } else { 0 };

                    // Handle out of bounds start
                    if start >= s.len() {
                        self.push(Val::string(""));
                    } else {
                        let len = match len.to_u64() {
                            Some(len) => len as usize,
                            None => {
                                return Err(self.err("Expected non-negative integer for length"));
                            }
                        };

                        // Calculate end position, ensuring we don't go past the end of the string
                        let end = std::cmp::min(start + len, s.len());

                        // Extract the substring
                        let result = s[start..end].to_string();
                        self.push(Val::string(result));
                    }
                }

                Inst::Asc => {
                    let s = self.pop();

                    let s = s.to_string();
                    if s.is_empty() {
                        // Return 0 for empty string (consistent with some BASIC implementations)
                        self.push(Val::Int(BigInt::zero()));
                    } else {
                        // Get the first character and convert to its Unicode code point
                        let first_char = s.chars().next().unwrap();
                        self.push(Val::Int(BigInt::from(first_char as u32)));
                    }
                }

                Inst::Chr => {
                    let n = self.pop();

                    let code_point = match n.to_u32() {
                        Some(n) => n,
                        None => {
                            return Err(
                                self.err("Expected non-negative integer for character code")
                            );
                        }
                    };

                    let c = match std::char::from_u32(code_point) {
                        Some(c) => c,
                        None => return Err(self.err("Invalid character code")),
                    };

                    self.push(Val::string(c.to_string()));
                }

                Inst::Instr => {
                    let find = self.pop();
                    let s = self.pop();

                    let s = s.to_string();
                    let find = find.to_string();

                    // In BASIC, InStr returns 0 if not found, position otherwise (1-based)
                    let position = match s.find(&find) {
                        Some(pos) => pos + 1, // Converting to 1-based indexing
                        None => 0,
                    };

                    self.push(Val::Int(BigInt::from(position)));
                }

                Inst::UCase => {
                    let s = self.pop();
                    let s = s.to_string();

                    self.push(Val::string(s.to_uppercase()));
                }

                Inst::LCase => {
                    let s = self.pop();
                    let s = s.to_string();

                    self.push(Val::string(s.to_lowercase()));
                }
            }
            self.pc += 1;
        }
        Ok(())
    }
}
