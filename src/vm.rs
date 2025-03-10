use fastnum::decimal::Context;
use fastnum::{dec256, D256};
use std::fmt;
use std::ops::{Add, Div, Mul, Sub};
use std::rc::Rc;

#[derive(Clone, Debug)]
pub enum Val {
    Num(D256),
    Str(Rc<String>),
}

pub type EvalResult = Result<Val, String>;

// fastnum provides Inf, so we might as well use it
pub const NO_TRAPS: Context = Context::default().without_traps();

impl Val {
    pub fn number(a: D256) -> Self {
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
    type Output = EvalResult;

    fn add(self, other: Val) -> EvalResult {
        match (&self, &other) {
            (Val::Num(a), Val::Num(b)) => Ok(Val::Num(a.clone() + b.clone())),
            _ => {
                let mut result = self.as_string();
                result.push_str(&other.as_string());
                Ok(Val::Str(Rc::new(result)))
            }
        }
    }
}

impl Div for Val {
    type Output = EvalResult;

    fn div(self, other: Val) -> EvalResult {
        match (&self, &other) {
            (Val::Num(a), Val::Num(b)) => Ok(Val::Num(a.clone() / b.clone())),
            _ => Err("/: expected numbers".to_string()),
        }
    }
}

impl Mul for Val {
    type Output = EvalResult;

    fn mul(self, other: Val) -> EvalResult {
        match (&self, &other) {
            (Val::Num(a), Val::Num(b)) => Ok(Val::Num(a.clone() * b.clone())),
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
    type Output = EvalResult;

    fn sub(self, other: Val) -> EvalResult {
        match (&self, &other) {
            (Val::Num(a), Val::Num(b)) => Ok(Val::Num(a.clone() - b.clone())),
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

pub struct VM {
    stack: Vec<Val>,
    // other VM state here
}

impl VM {
    pub fn new() -> Self {
        VM {
            stack: Vec::new(),
            // initialize other fields
        }
    }

    fn push(&mut self, val: Val) {
        self.stack.push(val);
    }

    fn pop(&mut self) -> Option<Val> {
        self.stack.pop()
    }

    pub fn run(&mut self) {
        self.push(Val::Num(dec256!(42)));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}
