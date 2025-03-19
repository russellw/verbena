use num_bigint::BigInt;
use num_traits::One;
use num_traits::ToPrimitive;
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

/// A runtime value in the virtual machine.
///
/// Values can be integers, floating-point numbers, strings, or lists.
#[derive(Clone, Debug, PartialEq)]
pub enum Val {
    /// Integer value with arbitrary precision
    Int(BigInt),
    /// Floating-point value
    Float(f64),
    /// String value
    Str(Rc<String>),
    /// List value
    List(Rc<RefCell<List>>),
}

/// A collection of values.
#[derive(Debug, PartialEq)]
pub struct List {
    pub v: Vec<Val>,
}

impl Val {
    /// Creates a new string value from any type that can be converted to a String.
    ///
    /// # Arguments
    ///
    /// * `s` - A value that can be converted into a String
    ///
    /// # Returns
    ///
    /// A Val::Str containing the string value
    pub fn string<S: Into<String>>(s: S) -> Self {
        Val::Str(Rc::new(s.into()))
    }

    /// Attempts to convert the value to a BigInt.
    ///
    /// # Returns
    ///
    /// * `Some(BigInt)` - If the value could be converted
    /// * `None` - If the value could not be converted
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

    /// Attempts to convert the value to a u32.
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

    /// Attempts to convert the value to an i32.
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

    /// Attempts to convert the value to a u64.
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

    /// Attempts to convert the value to a usize.
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

    /// Attempts to convert the value to an f64.
    pub fn to_f64(&self) -> Option<f64> {
        match self {
            Val::Int(a) => a.to_f64(),
            Val::Float(a) => Some(*a),
            Val::Str(s) => s.parse::<f64>().ok(),
            _ => None,
        }
    }

    /// Determines whether the value is "truthy".
    ///
    /// # Returns
    ///
    /// * `true` - For non-zero numbers, non-empty strings, and non-empty lists
    /// * `false` - For zero numbers, empty strings, and empty lists
    pub fn truth(&self) -> bool {
        match self {
            Val::Int(a) => !a.is_zero(),
            Val::Float(a) => *a != 0.0,
            Val::Str(s) => !s.is_empty(),
            Val::List(a) => !a.borrow().v.is_empty(),
        }
    }
}

pub fn eq(a: &Val, b: &Val) -> bool {
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

pub fn lt(a: &Val, b: &Val) -> bool {
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

pub fn le(a: &Val, b: &Val) -> bool {
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

pub fn to_int(b: bool) -> Val {
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
    pub fn new(n: usize) -> Self {
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
