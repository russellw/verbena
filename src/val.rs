use crate::VM;
use crate::list::*;
use num_bigint::BigInt;
use num_traits::One;
use num_traits::ToPrimitive;
use num_traits::Zero;
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

#[derive(Clone)]
pub enum Val {
    // Value semantics
    True,
    False,
    Null,
    Int(BigInt),
    Num(f64),
    Str(String),

    // Reference semantics
    List(Rc<RefCell<List>>),

    // Functions of various arities
    Func0(Rc<dyn Fn(&mut VM) -> Result<Val, String>>),
    Func1(Rc<dyn Fn(&mut VM, Val) -> Result<Val, String>>),
    Func2(Rc<dyn Fn(&mut VM, Val, Val) -> Result<Val, String>>),
    Func3(Rc<dyn Fn(&mut VM, Val, Val, Val) -> Result<Val, String>>),
    FuncV(Rc<dyn Fn(&mut VM, Vec<Val>) -> Result<Val, String>>),
}

impl Val {
    pub fn from_bool(b: bool) -> Self {
        if b { Val::True } else { Val::False }
    }

    pub fn func0<F>(f: F) -> Self
    where
        F: Fn(&mut VM) -> Result<Val, String> + 'static,
    {
        Val::Func0(Rc::new(f))
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

    pub fn num(&self) -> Result<Val, String> {
        let r = match self {
            Val::True => Val::Int(BigInt::one()),
            Val::False => Val::Int(BigInt::zero()),
            Val::Int(_) | Val::Num(_) => self.clone(),
            _ => return Err("Not a number".to_string()),
        };
        Ok(r)
    }

    pub fn num_loose(&self) -> Val {
        match self {
            Val::True => Val::Int(BigInt::one()),
            Val::False => Val::Int(BigInt::zero()),
            _ => self.clone(),
        }
    }

    pub fn to_bigint(&self) -> Result<BigInt, String> {
        let r = match self.num()? {
            Val::Int(a) => a.clone(),
            Val::Num(a) => {
                if !a.is_finite() {
                    return Err("Not a finite number".to_string());
                }
                BigInt::from(a as i128)
            }
            _ => panic!(),
        };
        Ok(r)
    }

    pub fn to_u32(&self) -> Result<u32, String> {
        let r = match self.num()? {
            Val::Int(a) => match a.to_u32() {
                Some(a) => a,
                None => return Err("Integer out of range".to_string()),
            },
            Val::Num(a) => {
                if !a.is_finite() {
                    return Err("Not a finite number".to_string());
                }
                a as u32
            }
            _ => panic!(),
        };
        Ok(r)
    }

    pub fn to_i32(&self) -> Result<i32, String> {
        let r = match self.num()? {
            Val::Int(a) => match a.to_i32() {
                Some(a) => a,
                None => return Err("Integer out of range".to_string()),
            },
            Val::Num(a) => {
                if !a.is_finite() {
                    return Err("Not a finite number".to_string());
                }
                a as i32
            }
            _ => panic!(),
        };
        Ok(r)
    }

    pub fn to_u64(&self) -> Result<u64, String> {
        let r = match self.num()? {
            Val::Int(a) => match a.to_u64() {
                Some(a) => a,
                None => return Err("Integer out of range".to_string()),
            },
            Val::Num(a) => {
                if !a.is_finite() {
                    return Err("Not a finite number".to_string());
                }
                a as u64
            }
            _ => panic!(),
        };
        Ok(r)
    }

    pub fn to_usize(&self) -> Result<usize, String> {
        let r = match self.num()? {
            Val::Int(a) => match a.to_usize() {
                Some(a) => a,
                None => return Err("Integer out of range".to_string()),
            },
            Val::Num(a) => {
                if !a.is_finite() {
                    return Err("Not a finite number".to_string());
                }
                a as usize
            }
            _ => panic!(),
        };
        Ok(r)
    }

    pub fn to_isize(&self) -> Result<isize, String> {
        let r = match self.num()? {
            Val::Int(a) => match a.to_isize() {
                Some(a) => a,
                None => return Err("Integer out of range".to_string()),
            },
            Val::Num(a) => {
                if !a.is_finite() {
                    return Err("Not a finite number".to_string());
                }
                a as isize
            }
            _ => panic!(),
        };
        Ok(r)
    }

    pub fn to_f64(&self) -> Result<f64, String> {
        let r = match self.num()? {
            Val::Int(a) => a.to_f64().unwrap(),
            Val::Num(a) => a,
            _ => panic!(),
        };
        Ok(r)
    }

    pub fn truth(&self) -> bool {
        match self {
            Val::False | Val::Null => false,
            Val::Int(a) => !a.is_zero(),
            Val::Num(a) => *a != 0.0,
            Val::Str(s) => !s.is_empty(),
            _ => true,
        }
    }
}

pub fn num2(a: &Val, b: &Val) -> Result<(Val, Val), String> {
    let a = a.num()?;
    let b = b.num()?;
    let r = match (&a, &b) {
        (Val::Int(a), Val::Num(_)) => {
            let a = Val::Num(a.to_f64().unwrap());
            (a, b)
        }
        (Val::Num(_), Val::Int(b)) => {
            let b = Val::Num(b.to_f64().unwrap());
            (a, b)
        }
        _ => (a, b),
    };
    Ok(r)
}

pub fn num2_loose(a: &Val, b: &Val) -> (Val, Val) {
    let a = a.num_loose();
    let b = b.num_loose();
    match (&a, &b) {
        (Val::Int(a), Val::Num(_)) => {
            let a = Val::Num(a.to_f64().unwrap());
            (a, b)
        }
        (Val::Num(_), Val::Int(b)) => {
            let b = Val::Num(b.to_f64().unwrap());
            (a, b)
        }
        _ => (a, b),
    }
}

pub fn eq_loose(a: &Val, b: &Val) -> bool {
    let (a, b) = num2_loose(a, b);
    match (&a, &b) {
        // TODO: is this needed?
        (Val::Func0(a), Val::Func0(b)) => Rc::ptr_eq(a, b),
        _ => a == b,
    }
}

pub fn lt_loose(a: &Val, b: &Val) -> bool {
    let (a, b) = num2_loose(a, b);
    match (&a, &b) {
        (Val::Int(a), Val::Int(b)) => a < b,
        (Val::Num(a), Val::Num(b)) => a < b,
        _ => {
            let a = a.to_string();
            let b = b.to_string();
            a < b
        }
    }
}

pub fn le_loose(a: &Val, b: &Val) -> bool {
    let (a, b) = num2_loose(a, b);
    match (&a, &b) {
        (Val::Int(a), Val::Int(b)) => a <= b,
        (Val::Num(a), Val::Num(b)) => a <= b,
        _ => {
            let a = a.to_string();
            let b = b.to_string();
            a <= b
        }
    }
}

impl fmt::Display for Val {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Val::True => write!(f, "true"),
            Val::False => write!(f, "false"),
            Val::Null => write!(f, "null"),
            Val::Int(a) => write!(f, "{}", a),
            Val::Num(a) => write!(f, "{}", a),
            Val::Str(s) => write!(f, "{}", s),
            Val::List(a) => write!(f, "{}", a.borrow()),
            // TODO
            _ => write!(f, "<fn>"),
        }
    }
}

impl std::fmt::Debug for Val {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Val::True => f.debug_tuple("True").finish(),
            Val::False => f.debug_tuple("False").finish(),
            Val::Null => f.debug_tuple("Null").finish(),
            Val::Int(a) => f.debug_tuple("Int").field(a).finish(),
            Val::Num(a) => f.debug_tuple("Num").field(a).finish(),
            Val::Str(s) => f.debug_tuple("Str").field(s).finish(),
            Val::List(a) => f.debug_tuple("List").field(&a.borrow()).finish(),
            Val::Func0(_) => f.debug_tuple("Func0").field(&"...").finish(),
            Val::Func1(_) => f.debug_tuple("Func1").field(&"...").finish(),
            Val::Func2(_) => f.debug_tuple("Func2").field(&"...").finish(),
            Val::Func3(_) => f.debug_tuple("Func3").field(&"...").finish(),
            Val::FuncV(_) => f.debug_tuple("FuncV").field(&"...").finish(),
        }
    }
}

impl PartialEq for Val {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Val::True, Val::True) | (Val::False, Val::False) | (Val::Null, Val::Null) => true,
            (Val::Int(a), Val::Int(b)) => a == b,
            (Val::Num(a), Val::Num(b)) => a == b,
            (Val::Str(a), Val::Str(b)) => a == b,
            (Val::List(a), Val::List(b)) => a == b,
            // Functions are compared by reference equality
            (Val::Func0(a), Val::Func0(b)) => Rc::ptr_eq(a, b),
            (Val::Func1(a), Val::Func1(b)) => Rc::ptr_eq(a, b),
            (Val::Func2(a), Val::Func2(b)) => Rc::ptr_eq(a, b),
            (Val::Func3(a), Val::Func3(b)) => Rc::ptr_eq(a, b),
            (Val::FuncV(a), Val::FuncV(b)) => Rc::ptr_eq(a, b),
            // Different variant types are not equal
            _ => false,
        }
    }
}

impl std::hash::Hash for Val {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // Hash the discriminant first
        std::mem::discriminant(self).hash(state);

        match self {
            Val::True => {}
            Val::False => {}
            Val::Null => {}
            Val::Int(i) => i.hash(state),
            Val::Num(f) => {
                // Handle float hashing by converting to bits
                f.to_bits().hash(state)
            }
            Val::Str(s) => s.hash(state),
            _ => panic!(),
        }
    }
}

impl Eq for Val {}
