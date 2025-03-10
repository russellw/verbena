use fastnum::decimal::Context;
use fastnum::D256;
use std::fmt;
use std::ops::{Add, Div, Mul, Sub};
use std::rc::Rc;

#[derive(Clone, Debug)]
pub enum Value {
    Number(D256),
    String(Rc<String>),
}

pub type EvalResult = Result<Value, String>;

// fastnum provides Inf, so we might as well use it
pub const NO_TRAPS: Context = Context::default().without_traps();

impl Value {
    pub fn number(a: D256) -> Self {
        Value::Number(a)
    }

    pub fn string<S: Into<String>>(s: S) -> Self {
        Value::String(Rc::new(s.into()))
    }

    pub fn as_string(&self) -> String {
        match self {
            Value::String(s) => s.to_string(),
            Value::Number(a) => a.to_string(),
        }
    }

    pub fn truth(&self) -> bool {
        match self {
            Value::String(s) => s.len()!=0,
            Value::Number(a) => !a.is_zero(),
        }
    }
}

impl Add for Value {
    type Output = EvalResult;

    fn add(self, other: Value) -> EvalResult {
        match (&self, &other) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a.clone() + b.clone())),
            _ => {
                let mut result = self.as_string();
                result.push_str(&other.as_string());
                Ok(Value::String(Rc::new(result)))
            }
        }
    }
}

impl Div for Value {
    type Output = EvalResult;

    fn div(self, other: Value) -> EvalResult {
        match (&self, &other) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a.clone() / b.clone())),
            _ => Err("/: expected numbers".to_string()),
        }
    }
}

impl Mul for Value {
    type Output = EvalResult;

    fn mul(self, other: Value) -> EvalResult {
        match (&self, &other) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a.clone() * b.clone())),
            (Value::Number(a), Value::String(b)) => {
                let count = match usize::try_from(*a) {
                    Ok(n) => n,
                    Err(_) => return Err("*: cannot convert number to repeat count".to_string()),
                };
                Ok(Value::String(Rc::new(b.repeat(count))))
            }
            (Value::String(a), Value::Number(b)) => {
                let count = match usize::try_from(*b) {
                    Ok(n) => n,
                    Err(_) => return Err("*: cannot convert number to repeat count".to_string()),
                };
                Ok(Value::String(Rc::new(a.repeat(count))))
            }
            _ => Err("*: expected numbers".to_string()),
        }
    }
}

impl Sub for Value {
    type Output = EvalResult;

    fn sub(self, other: Value) -> EvalResult {
        match (&self, &other) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a.clone() - b.clone())),
            _ => Err("-: expected numbers".to_string()),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Number(a) => write!(f, "{}", a),
            Value::String(s) => write!(f, "{}", s),
        }
    }
}
