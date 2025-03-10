use fastnum::decimal::Context;
use fastnum::D256;
use std::fmt;
use std::ops::{Add, Div, Mul, Sub};
use std::rc::Rc;

#[derive(Clone, Debug)]
pub enum Val {
    Num(D256),
    String(Rc<String>),
}

pub type EvalResult = Result<Val, String>;

// fastnum provides Inf, so we might as well use it
pub const NO_TRAPS: Context = Context::default().without_traps();

impl Val {
    pub fn number(a: D256) -> Self {
        Val::Num(a)
    }

    pub fn string<S: Into<String>>(s: S) -> Self {
        Val::String(Rc::new(s.into()))
    }

    pub fn as_string(&self) -> String {
        match self {
            Val::String(s) => s.to_string(),
            Val::Num(a) => a.to_string(),
        }
    }

    pub fn truth(&self) -> bool {
        match self {
            Val::String(s) => s.len()!=0,
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
                Ok(Val::String(Rc::new(result)))
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
            (Val::Num(a), Val::String(b)) => {
                let count = match usize::try_from(*a) {
                    Ok(n) => n,
                    Err(_) => return Err("*: cannot convert number to repeat count".to_string()),
                };
                Ok(Val::String(Rc::new(b.repeat(count))))
            }
            (Val::String(a), Val::Num(b)) => {
                let count = match usize::try_from(*b) {
                    Ok(n) => n,
                    Err(_) => return Err("*: cannot convert number to repeat count".to_string()),
                };
                Ok(Val::String(Rc::new(a.repeat(count))))
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
            Val::String(s) => write!(f, "{}", s),
        }
    }
}

impl PartialEq for Val {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Val::Num(a), Val::Num(b)) => a == b,
            (Val::String(a), Val::String(b)) => a == b,
            _ => false,
        }
    }
}
