use crate::list::*;
use crate::val::*;
use crate::vm::*;
use rand::Rng;
use std::cell::RefCell;
use std::io;
use std::io::Write;
use std::rc::Rc;

fn input(_vm: &mut VM) -> Result<Val, String> {
    // Flush in case there is a prompt pending
    if let Err(e) = io::stdout().flush() {
        return Err(format!("Failed to flush stdout: {}", e));
    }

    // Handle potential read_line error
    let mut s = String::new();
    match io::stdin().read_line(&mut s) {
        Ok(_) => {
            // Remove the trailing newline character
            let s = s.trim();
            Ok(Val::Str(s.to_string()))
        }
        Err(e) => Err(format!("Failed to read line: {}", e)),
    }
}

fn eq(_vm: &mut VM, a: Val, b: Val) -> Result<Val, String> {
    fn f(a: &Val, b: &Val) -> bool {
        if a == b {
            return true;
        }
        match (&a, &b) {
            (Val::List(a), Val::List(b)) => {
                let a = &a.borrow();
                let b = &b.borrow();
                let n = a.len();
                if n != b.len() {
                    return false;
                }
                for i in 0..n {
                    if !f(&a[i], &b[i]) {
                        return false;
                    }
                }
                true
            }
            _ => false,
        }
    }

    Ok(Val::from_bool(f(&a, &b)))
}

fn sqrt(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.get_f64()?;
    let r = Val::Num(a.sqrt());
    Ok(r)
}

fn num(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let r = match a {
        Val::True => 1.0,
        Val::False => 0.0,
        Val::Num(a) => a,
        Val::Str(s) => match s.to_string().parse::<f64>() {
            Ok(a) => a,
            Err(e) => return Err(e.to_string()),
        },
        _ => return Err("Not a number".to_string()),
    };
    let r = Val::Num(r);
    Ok(r)
}

fn str_(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let r = Val::Str(a.to_string());
    Ok(r)
}

fn typeof_(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let r = match a {
        Val::Num(_) => "num",
        Val::Str(_) => "str",
        Val::List(_) => "list",
        Val::Object(_) => "object",
        Val::True | Val::False => "bool",
        Val::Null => "null",
        _ => "fn",
    };
    let r = r.to_string();
    let r = Val::Str(r);
    Ok(r)
}

fn copysign(_vm: &mut VM, a: Val, sign: Val) -> Result<Val, String> {
    let a = a.get_f64()?;
    let sign = sign.get_f64()?;
    let r = Val::Num(a.copysign(sign));
    Ok(r)
}

// TODO: name?
fn numbase(_vm: &mut VM, s: Val, base: Val) -> Result<Val, String> {
    let s = s.get_string()?;
    let base = base.get_u32()?;
    let r = match i64::from_str_radix(&s, base) {
        Ok(a) => a,
        Err(e) => return Err(e.to_string()),
    };
    let r = r as f64;
    let r = Val::Num(r);
    Ok(r)
}

fn abs(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.get_f64()?;
    let r = Val::Num(a.abs());
    Ok(r)
}

fn cbrt(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.get_f64()?;
    let r = Val::Num(a.cbrt());
    Ok(r)
}

fn rnd(vm: &mut VM) -> Result<Val, String> {
    let r: f64 = vm.rng.random();
    Ok(Val::Num(r))
}

fn floor(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.get_f64()?;
    let r = Val::Num(a.floor());
    Ok(r)
}

fn ceil(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.get_f64()?;
    let r = Val::Num(a.ceil());
    Ok(r)
}

fn round(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.get_f64()?;
    let r = Val::Num(a.round());
    Ok(r)
}

fn roundeven(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.get_f64()?;
    let r = Val::Num(a.round_ties_even());
    Ok(r)
}

fn trunc(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.get_f64()?;
    let r = Val::Num(a.trunc());
    Ok(r)
}

fn fma(_vm: &mut VM, a: Val, b: Val, c: Val) -> Result<Val, String> {
    let a = a.get_f64()?;
    let b = b.get_f64()?;
    let c = c.get_f64()?;
    let r = Val::Num(a.mul_add(b, c));
    Ok(r)
}

fn exp(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.get_f64()?;
    let r = Val::Num(a.exp());
    Ok(r)
}

fn exp2(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.get_f64()?;
    let r = Val::Num(a.exp2());
    Ok(r)
}

fn log(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.get_f64()?;
    let r = Val::Num(a.ln());
    Ok(r)
}

fn log2(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.get_f64()?;
    let r = Val::Num(a.log2());
    Ok(r)
}

fn log10(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.get_f64()?;
    let r = Val::Num(a.log10());
    Ok(r)
}

fn hypot(_vm: &mut VM, a: Val, b: Val) -> Result<Val, String> {
    let a = a.get_f64()?;
    let b = b.get_f64()?;
    let r = Val::Num(a.hypot(b));
    Ok(r)
}

fn sin(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.get_f64()?;
    let r = Val::Num(a.sin());
    Ok(r)
}

fn cos(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.get_f64()?;
    let r = Val::Num(a.cos());
    Ok(r)
}

fn tan(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.get_f64()?;
    let r = Val::Num(a.tan());
    Ok(r)
}

fn asin(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.get_f64()?;
    let r = Val::Num(a.asin());
    Ok(r)
}

fn acos(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.get_f64()?;
    let r = Val::Num(a.acos());
    Ok(r)
}

fn atan(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.get_f64()?;
    let r = Val::Num(a.atan());
    Ok(r)
}

fn atan2(_vm: &mut VM, a: Val, b: Val) -> Result<Val, String> {
    let a = a.get_f64()?;
    let b = b.get_f64()?;
    let r = Val::Num(a.atan2(b));
    Ok(r)
}

fn expm1(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.get_f64()?;
    let r = Val::Num(a.exp_m1());
    Ok(r)
}

fn log1p(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.get_f64()?;
    let r = Val::Num(a.ln_1p());
    Ok(r)
}

fn sinh(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.get_f64()?;
    let r = Val::Num(a.sinh());
    Ok(r)
}

fn cosh(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.get_f64()?;
    let r = Val::Num(a.cosh());
    Ok(r)
}

fn tanh(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.get_f64()?;
    let r = Val::Num(a.tanh());
    Ok(r)
}

fn asinh(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.get_f64()?;
    let r = Val::Num(a.asinh());
    Ok(r)
}
fn acosh(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.get_f64()?;
    let r = Val::Num(a.acosh());
    Ok(r)
}

fn atanh(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.get_f64()?;
    let r = Val::Num(a.atanh());
    Ok(r)
}

fn is_nan(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.get_f64()?;
    let r = a.is_nan();
    let r = Val::from_bool(r);
    Ok(r)
}

fn is_finite(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.get_f64()?;
    let r = a.is_finite();
    let r = Val::from_bool(r);
    Ok(r)
}

fn is_inf(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.get_f64()?;
    let r = a.is_infinite();
    let r = Val::from_bool(r);
    Ok(r)
}

fn is_subnormal(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.get_f64()?;
    let r = a.is_subnormal();
    let r = Val::from_bool(r);
    Ok(r)
}

fn is_normal(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.get_f64()?;
    let r = a.is_normal();
    let r = Val::from_bool(r);
    Ok(r)
}

fn max(_vm: &mut VM, a: Val, b: Val) -> Result<Val, String> {
    let r = match (&a, &b) {
        (Val::Num(a), Val::Num(b)) => Val::Num(a.max(*b)),
        _ => {
            if lt(&b, &a) {
                a
            } else {
                b
            }
        }
    };
    Ok(r)
}

fn range(_vm: &mut VM, i: Val, j: Val) -> Result<Val, String> {
    let (i, j) = if j == Val::Null {
        (Val::Num(0.0), i)
    } else {
        (i, j)
    };

    let i = i.get_f64()?;
    let j = j.get_f64()?;
    let i = i.min(j);

    let n = (j - i) as usize;
    let mut v = Vec::with_capacity(n);
    let mut k = i;
    while k < j {
        v.push(Val::Num(k));
        k += 1.0;
    }
    let v = List::from(v);
    let v = Rc::new(RefCell::new(v));
    let v = Val::List(v);
    Ok(v)
}

fn min(_vm: &mut VM, a: Val, b: Val) -> Result<Val, String> {
    let r = match (&a, &b) {
        (Val::Num(a), Val::Num(b)) => Val::Num(a.min(*b)),
        _ => {
            if lt(&a, &b) {
                a
            } else {
                b
            }
        }
    };
    Ok(r)
}

fn len(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let len = match &a {
        Val::List(a) => a.borrow().len(),
        Val::Str(s) => s.len(),
        Val::Object(a) => a.borrow().len(),
        _ => return Err("Not a collection".to_string()),
    };
    Ok(Val::Num(len as f64))
}

fn ord(_vm: &mut VM, s: Val) -> Result<Val, String> {
    let s = s.get_string()?;
    if s.len() != 1 {
        return Err("Expected character".to_string());
    }
    let r = s.bytes().next().unwrap();
    let r = r as f64;
    let r = Val::Num(r);
    Ok(r)
}

fn chr(_vm: &mut VM, n: Val) -> Result<Val, String> {
    let code_point = n.get_u32()?;
    let c = match std::char::from_u32(code_point) {
        Some(c) => c,
        None => return Err("Invalid character code".to_string()),
    };
    Ok(Val::Str(c.to_string()))
}

fn upper(_vm: &mut VM, s: Val) -> Result<Val, String> {
    let s = s.to_string();
    Ok(Val::Str(s.to_uppercase()))
}

fn lower(_vm: &mut VM, s: Val) -> Result<Val, String> {
    let s = s.to_string();
    Ok(Val::Str(s.to_lowercase()))
}

// Register all functions to the VM
pub fn register_all(vm: &mut VM) {
    vm.register1("abs", abs);
    vm.register1("acos", acos);
    vm.register1("acosh", acosh);
    vm.register1("asin", asin);
    vm.register1("asinh", asinh);
    vm.register1("atan", atan);
    vm.register2("atan2", atan2);
    vm.register1("atanh", atanh);
    vm.register1("cbrt", cbrt);
    vm.register1("ceil", ceil);
    vm.register1("chr", chr);
    vm.register2("copysign", copysign);
    vm.register1("cos", cos);
    vm.register1("cosh", cosh);
    vm.register2("eq", eq);
    vm.register1("exp", exp);
    vm.register1("exp2", exp2);
    vm.register1("expm1", expm1);
    vm.register1("finite?", is_finite);
    vm.register1("floor", floor);
    vm.register3("fma", fma);
    vm.register2("hypot", hypot);
    vm.register1("inf?", is_inf);
    vm.register0("input", input);
    vm.register1("len", len);
    vm.register1("log", log);
    vm.register1("log10", log10);
    vm.register1("log1p", log1p);
    vm.register1("log2", log2);
    vm.register1("lower", lower);
    vm.register2("max", max);
    vm.register2("min", min);
    vm.register1("nan?", is_nan);
    vm.register1("normal?", is_normal);
    vm.register1("num", num);
    vm.register2("numbase", numbase);
    vm.register1("ord", ord);
    vm.register2("range", range);
    vm.register0("rnd", rnd);
    vm.register1("round", round);
    vm.register1("roundeven", roundeven);
    vm.register1("sin", sin);
    vm.register1("sinh", sinh);
    vm.register1("sqrt", sqrt);
    vm.register1("str", str_);
    vm.register1("subnormal?", is_subnormal);
    vm.register1("tan", tan);
    vm.register1("tanh", tanh);
    vm.register1("trunc", trunc);
    vm.register1("typeof", typeof_);
    vm.register1("upper", upper);
}
