use fastnum::decimal::Context;
use fastnum::{dec256, D256};
use std::fmt;
use std::ops::{Add, Div, Mul, Sub};
use std::rc::Rc;

// fastnum provides Inf, so we might as well use it
pub const NO_TRAPS: Context = Context::default().without_traps();

#[derive(Clone, Debug)]
pub enum Val {
    Num(D256),
    Str(Rc<String>),
}

pub type ValResult = Result<Val, String>;

impl Val {
    pub fn num(a: D256) -> Self {
        Val::Num(a)
    }

    pub fn string<S: Into<String>>(s: S) -> Self {
        Val::Str(Rc::new(s.into()))
    }

    pub fn as_string(&self) -> String {
        match self {
            Val::Str(s) => s.to_string(),
            Val::Num(a) => a.to_string(),
        }
    }

    pub fn truth(&self) -> bool {
        match self {
            Val::Str(s) => s.len() != 0,
            Val::Num(a) => !a.is_zero(),
        }
    }
}

impl Add for Val {
    type Output = ValResult;

    fn add(self, other: Val) -> ValResult {
        match (&self, &other) {
            (Val::Num(a), Val::Num(b)) => Ok(Val::Num(*a + *b)),
            _ => {
                let mut s = self.as_string();
                s.push_str(&other.as_string());
                Ok(Val::Str(Rc::new(s)))
            }
        }
    }
}

impl Div for Val {
    type Output = ValResult;

    fn div(self, other: Val) -> ValResult {
        match (&self, &other) {
            (Val::Num(a), Val::Num(b)) => Ok(Val::Num(*a / *b)),
            _ => Err("/: expected numbers".to_string()),
        }
    }
}

impl Mul for Val {
    type Output = ValResult;

    fn mul(self, other: Val) -> ValResult {
        match (&self, &other) {
            (Val::Num(a), Val::Num(b)) => Ok(Val::Num(*a * *b)),
            (Val::Num(a), Val::Str(b)) => {
                let count = match usize::try_from(*a) {
                    Ok(n) => n,
                    Err(_) => return Err("*: cannot convert number to repeat count".to_string()),
                };
                Ok(Val::Str(Rc::new(b.repeat(count))))
            }
            (Val::Str(a), Val::Num(b)) => {
                let count = match usize::try_from(*b) {
                    Ok(n) => n,
                    Err(_) => return Err("*: cannot convert number to repeat count".to_string()),
                };
                Ok(Val::Str(Rc::new(a.repeat(count))))
            }
            _ => Err("*: expected numbers".to_string()),
        }
    }
}

impl Sub for Val {
    type Output = ValResult;

    fn sub(self, other: Val) -> ValResult {
        match (&self, &other) {
            (Val::Num(a), Val::Num(b)) => Ok(Val::Num(*a - *b)),
            _ => Err("-: expected numbers".to_string()),
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

impl PartialEq for Val {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Val::Num(a), Val::Num(b)) => a == b,
            (Val::Str(a), Val::Str(b)) => a == b,
            _ => false,
        }
    }
}

#[derive(Debug)]
pub enum Inst {
    Add,
    Sub,
    Mul,
    Div,
    Goto(usize),
    End,
    Const(Val),
}

#[derive(Debug)]
pub struct VM {
    code: Vec<Inst>,
    pc: usize,
    stack: Vec<Val>,
}

impl VM {
    pub fn new(code: Vec<Inst>) -> Self {
        VM {
            code,
            pc: 0,
            stack: Vec::new(),
        }
    }

    fn push(&mut self, val: Val) {
        self.stack.push(val);
    }

    fn pop(&mut self) -> Val {
        self.stack.pop().expect("stack underflow")
    }

    pub fn run(&mut self) -> Result<(), String> {
        while self.pc < self.code.len() {
            let i = self.pc;
            self.pc += 1;
            match &self.code[i] {
                Inst::Const(a) => {
                    self.push(a.clone());
                }
                Inst::Add => {
                    let b = self.pop();
                    let a = self.pop();
                    match a.add(b) {
                        Ok(result) => self.push(result),
                        Err(e) => return Err(e),
                    }
                }
                Inst::Goto(target) => {
                    self.pc = *target;
                }
                Inst::End => {
                    return Ok(());
                }
                _ => {
                    panic!("")
                }
            }
        }
        return Ok(());
    }
}
