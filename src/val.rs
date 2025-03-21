use crate::VM;
use num_bigint::BigInt;
use num_traits::One;
use num_traits::ToPrimitive;
use num_traits::Zero;
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

/// A runtime value in the virtual machine.
///
/// Values can be integers, floating-point numbers, strings, lists, or functions.
#[derive(Clone)]
pub enum Val {
    /// Integer value with arbitrary precision
    Int(BigInt),
    /// Floating-point value
    Float(f64),
    /// String value
    Str(Rc<String>),
    /// List value
    List(Rc<RefCell<List>>),

    Func(Rc<dyn Fn(&mut VM) -> Result<Val, String>>),
    Func1(Rc<dyn Fn(&mut VM, Val) -> Result<Val, String>>),
    Func2(Rc<dyn Fn(&mut VM, Val, Val) -> Result<Val, String>>),
    Func3(Rc<dyn Fn(&mut VM, Val, Val, Val) -> Result<Val, String>>),
    FuncV(Rc<dyn Fn(&mut VM, Vec<Val>) -> Result<Val, String>>),
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

    pub fn func<F>(f: F) -> Self
    where
        F: Fn(&mut VM) -> Result<Val, String> + 'static,
    {
        Val::Func(Rc::new(f))
    }

    pub fn func1<F>(f: F) -> Self
    where
        F: Fn(&mut VM, Val) -> Result<Val, String> + 'static,
    {
        Val::Func1(Rc::new(f))
    }

    pub fn func2<F>(f: F) -> Self
    where
        F: Fn(&mut VM, Val, Val) -> Result<Val, String> + 'static,
    {
        Val::Func2(Rc::new(f))
    }

    pub fn func3<F>(f: F) -> Self
    where
        F: Fn(&mut VM, Val, Val, Val) -> Result<Val, String> + 'static,
    {
        Val::Func3(Rc::new(f))
    }

    pub fn funcv<F>(f: F) -> Self
    where
        F: Fn(&mut VM, Vec<Val>) -> Result<Val, String> + 'static,
    {
        Val::FuncV(Rc::new(f))
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

    pub fn apply(&self, vm: &mut VM, args: Vec<Val>) -> Result<Val, String> {
        match self {
            Val::Func(f) => {
                if args.len() != 0 {
                    return Err(format!("Expected 0 args, received {}", args.len()));
                }
                f(vm)
            }
            _ => Err("Not a function".to_string()),
        }
    }

    /// Determines whether the value is "truthy".
    ///
    /// # Returns
    ///
    /// * `true` - For non-zero numbers, non-empty strings, non-empty lists, and functions
    /// * `false` - For zero numbers, empty strings, and empty lists
    pub fn truth(&self) -> bool {
        match self {
            Val::Int(a) => !a.is_zero(),
            Val::Float(a) => *a != 0.0,
            Val::Str(s) => !s.is_empty(),
            _ => true,
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
        // TODO: is this needed?
        (Val::Func(a), Val::Func(b)) => Rc::ptr_eq(a, b),
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
            _ => write!(f, "<fn>"),
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

impl std::fmt::Debug for Val {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Val::Int(a) => write!(f, "Int({:?})", a),
            Val::Float(a) => write!(f, "Float({:?})", a),
            Val::Str(s) => write!(f, "Str({:?})", s),
            Val::List(l) => write!(f, "List({:?})", l),
            Val::Func(_) => write!(f, "Func(...)"),
            Val::Func1(_) => write!(f, "Func1(...)"),
            Val::Func2(_) => write!(f, "Func2(...)"),
            Val::Func3(_) => write!(f, "Func3(...)"),
            Val::FuncV(_) => write!(f, "FuncV(...)"),
        }
    }
}

impl PartialEq for Val {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Val::Int(a), Val::Int(b)) => a == b,
            (Val::Float(a), Val::Float(b)) => a == b,
            (Val::Str(a), Val::Str(b)) => a == b,
            (Val::List(a), Val::List(b)) => a == b,
            // Functions are compared by reference equality
            (Val::Func(a), Val::Func(b)) => Rc::ptr_eq(a, b),
            (Val::Func1(a), Val::Func1(b)) => Rc::ptr_eq(a, b),
            (Val::Func2(a), Val::Func2(b)) => Rc::ptr_eq(a, b),
            (Val::Func3(a), Val::Func3(b)) => Rc::ptr_eq(a, b),
            (Val::FuncV(a), Val::FuncV(b)) => Rc::ptr_eq(a, b),
            // Different variant types are not equal
            _ => false,
        }
    }
}
