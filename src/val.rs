use crate::VM;
use crate::list::*;
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

#[derive(Clone)]
pub enum Val {
    // Value semantics
    True,
    False,
    Null,
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

    pub fn as_string(&self) -> Result<String, String> {
        let r = match self {
            Val::Str(s) => s.to_string(),
            _ => return Err("Not a string".to_string()),
        };
        Ok(r)
    }

    pub fn num(&self) -> Result<Val, String> {
        let r = match self {
            Val::True => Val::Num(1.0),
            Val::False => Val::Num(0.0),
            Val::Num(_) => self.clone(),
            _ => return Err("Not a number".to_string()),
        };
        Ok(r)
    }

    // TODO Is this necessary?
    pub fn num_loose(&self) -> Val {
        match self {
            Val::True => Val::Num(1.0),
            Val::False => Val::Num(0.0),
            _ => self.clone(),
        }
    }

    pub fn to_u32(&self) -> Result<u32, String> {
        let r = match self.num()? {
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

    pub fn to_i64(&self) -> Result<i64, String> {
        let r = match self.num()? {
            Val::Num(a) => {
                if !a.is_finite() {
                    return Err("Not a finite number".to_string());
                }
                a as i64
            }
            _ => panic!(),
        };
        Ok(r)
    }

    pub fn to_u64(&self) -> Result<u64, String> {
        let r = match self.num()? {
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
        let r = match self {
            Val::True => 1.0,
            Val::False => 0.0,
            Val::Num(a) => *a,
            _ => return Err("Not a number".to_string()),
        };
        Ok(r)
    }

    pub fn truth(&self) -> bool {
        match self {
            Val::False | Val::Null => false,
            Val::Num(a) => *a != 0.0,
            Val::Str(s) => !s.is_empty(),
            _ => true,
        }
    }
}

pub fn lt_loose(a: &Val, b: &Val) -> bool {
    match (&a, &b) {
        (Val::Num(a), Val::Num(b)) => a < b,
        (Val::Str(a), Val::Str(b)) => a < b,
        (Val::False, Val::True) => true,
        _ => false,
    }
}

pub fn le_loose(a: &Val, b: &Val) -> bool {
    match (&a, &b) {
        (Val::Num(a), Val::Num(b)) => a <= b,
        (Val::Str(a), Val::Str(b)) => a <= b,
        (Val::False, Val::True) => true,
        _ => a == b,
    }
}

impl fmt::Display for Val {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Val::True => write!(f, "true"),
            Val::False => write!(f, "false"),
            Val::Null => write!(f, "null"),
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
