use crate::list::*;
use crate::val::*;
use crate::vm::*;
use num_bigint::BigInt;
use num_traits::ToPrimitive;
use num_traits::Zero;
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

fn sqrt(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.num()?;
    let r = match a {
        Val::Num(a) => Val::Num(a.sqrt()),
        Val::Int(a) => Val::Int(a.sqrt()),
        _ => panic!(),
    };
    Ok(r)
}

fn num(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let r = match a {
        Val::True => 1.0,
        Val::False => 0.0,
        Val::Int(a) => a.to_f64().unwrap(),
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

// TODO
fn _print(_vm: &mut VM, a: Val) -> Result<Val, String> {
    print!("{}", a);
    Ok(Val::Null)
}

fn typeof_(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let r = match a {
        Val::Int(_) => "int",
        Val::Num(_) => "num",
        Val::Str(_) => "str",
        Val::List(_) => "list",
        Val::True | Val::False => "bool",
        Val::Null => "null",
        _ => "fn",
    };
    let r = r.to_string();
    let r = Val::Str(r);
    Ok(r)
}

fn copysign(_vm: &mut VM, a: Val, sign: Val) -> Result<Val, String> {
    let a = a.to_f64()?;
    let sign = sign.to_f64()?;
    let r = Val::Num(a.copysign(sign));
    Ok(r)
}

fn strbase(_vm: &mut VM, a: Val, base: Val) -> Result<Val, String> {
    let a = a.to_bigint()?;
    let base = base.to_u32()?;
    let r = Val::Str(a.to_str_radix(base));
    Ok(r)
}

fn numbase(_vm: &mut VM, s: Val, base: Val) -> Result<Val, String> {
    let s = s.to_string();
    let base = base.to_u32()?;
    let r = match BigInt::parse_bytes(s.as_bytes(), base) {
        Some(a) => Val::Int(a),
        None => return Err("Unable to parse integer".to_string()),
    };
    Ok(r)
}

fn abs(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.to_f64()?;
    let r = Val::Num(a.abs());
    Ok(r)
}

fn cbrt(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.to_f64()?;
    let r = Val::Num(a.cbrt());
    Ok(r)
}

// TODO
fn _list(_vm: &mut VM, items: Vec<Val>) -> Result<Val, String> {
    let r = List::from(items);
    let r = Val::List(Rc::new(RefCell::new(r)));
    Ok(r)
}

fn rnd(vm: &mut VM) -> Result<Val, String> {
    let r: f64 = vm.rng.random();
    Ok(Val::Num(r))
}

fn floor(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.to_f64()?;
    let r = Val::Num(a.floor());
    Ok(r)
}

fn ceil(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.to_f64()?;
    let r = Val::Num(a.ceil());
    Ok(r)
}

fn round(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.to_f64()?;
    let r = Val::Num(a.round());
    Ok(r)
}

fn roundeven(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.to_f64()?;
    let r = Val::Num(a.round_ties_even());
    Ok(r)
}

fn trunc(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.to_f64()?;
    let r = Val::Num(a.trunc());
    Ok(r)
}

fn fma(_vm: &mut VM, a: Val, b: Val, c: Val) -> Result<Val, String> {
    let a = a.to_f64()?;
    let b = b.to_f64()?;
    let c = c.to_f64()?;
    let r = Val::Num(a.mul_add(b, c));
    Ok(r)
}

fn exp(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.to_f64()?;
    let r = Val::Num(a.exp());
    Ok(r)
}

fn exp2(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.to_f64()?;
    let r = Val::Num(a.exp2());
    Ok(r)
}

fn log(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.to_f64()?;
    let r = Val::Num(a.ln());
    Ok(r)
}

fn log2(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.to_f64()?;
    let r = Val::Num(a.log2());
    Ok(r)
}

fn log10(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.to_f64()?;
    let r = Val::Num(a.log10());
    Ok(r)
}

fn hypot(_vm: &mut VM, a: Val, b: Val) -> Result<Val, String> {
    let a = a.to_f64()?;
    let b = b.to_f64()?;
    let r = Val::Num(a.hypot(b));
    Ok(r)
}

fn sin(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.to_f64()?;
    let r = Val::Num(a.sin());
    Ok(r)
}

fn cos(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.to_f64()?;
    let r = Val::Num(a.cos());
    Ok(r)
}

fn tan(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.to_f64()?;
    let r = Val::Num(a.tan());
    Ok(r)
}

fn asin(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.to_f64()?;
    let r = Val::Num(a.asin());
    Ok(r)
}

fn acos(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.to_f64()?;
    let r = Val::Num(a.acos());
    Ok(r)
}

fn atan(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.to_f64()?;
    let r = Val::Num(a.atan());
    Ok(r)
}

fn atan2(_vm: &mut VM, a: Val, b: Val) -> Result<Val, String> {
    let a = a.to_f64()?;
    let b = b.to_f64()?;
    let r = Val::Num(a.atan2(b));
    Ok(r)
}

fn expm1(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.to_f64()?;
    let r = Val::Num(a.exp_m1());
    Ok(r)
}

fn log1p(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.to_f64()?;
    let r = Val::Num(a.ln_1p());
    Ok(r)
}

fn sinh(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.to_f64()?;
    let r = Val::Num(a.sinh());
    Ok(r)
}

fn cosh(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.to_f64()?;
    let r = Val::Num(a.cosh());
    Ok(r)
}

fn tanh(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.to_f64()?;
    let r = Val::Num(a.tanh());
    Ok(r)
}

fn asinh(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.to_f64()?;
    let r = Val::Num(a.asinh());
    Ok(r)
}
fn acosh(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.to_f64()?;
    let r = Val::Num(a.acosh());
    Ok(r)
}

fn atanh(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.to_f64()?;
    let r = Val::Num(a.atanh());
    Ok(r)
}

fn is_nan(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.to_f64()?;
    let r = a.is_nan();
    let r = Val::from_bool(r);
    Ok(r)
}

fn is_finite(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.to_f64()?;
    let r = a.is_finite();
    let r = Val::from_bool(r);
    Ok(r)
}

fn is_inf(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.to_f64()?;
    let r = a.is_infinite();
    let r = Val::from_bool(r);
    Ok(r)
}

fn is_subnormal(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.to_f64()?;
    let r = a.is_subnormal();
    let r = Val::from_bool(r);
    Ok(r)
}

fn is_normal(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.to_f64()?;
    let r = a.is_normal();
    let r = Val::from_bool(r);
    Ok(r)
}

fn max(_vm: &mut VM, a: Val, b: Val) -> Result<Val, String> {
    let (a, b) = num2_loose(&a, &b);
    let r = match (&a, &b) {
        (Val::Num(a), Val::Num(b)) => Val::Num(a.max(*b)),
        _ => {
            if lt_loose(&b, &a) {
                a
            } else {
                b
            }
        }
    };
    Ok(r)
}

fn min(_vm: &mut VM, a: Val, b: Val) -> Result<Val, String> {
    let (a, b) = num2_loose(&a, &b);
    let r = match (&a, &b) {
        (Val::Num(a), Val::Num(b)) => Val::Num(a.min(*b)),
        _ => {
            if lt_loose(&a, &b) {
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
        Val::List(a) => a.borrow().v.len(),
        Val::Str(s) => s.len(),
        _ => return Err("Not a collection".to_string()),
    };
    Ok(Val::Num(len as f64))
}

fn ord(_vm: &mut VM, s: Val) -> Result<Val, String> {
    let s = s.to_string();
    if s.is_empty() {
        // Return 0 for empty string (consistent with some BASIC implementations)
        Ok(Val::Int(BigInt::zero()))
    } else {
        // Get the first character and convert to its Unicode code point
        let first_char = s.chars().next().unwrap();
        Ok(Val::Int(BigInt::from(first_char as u32)))
    }
}

fn chr(_vm: &mut VM, n: Val) -> Result<Val, String> {
    let code_point = n.to_u32()?;
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
    vm.registerv("_list", _list);
    vm.register1("_print", _print);
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
    vm.register0("rnd", rnd);
    vm.register1("round", round);
    vm.register1("roundeven", roundeven);
    vm.register1("sin", sin);
    vm.register1("sinh", sinh);
    vm.register1("sqrt", sqrt);
    vm.register1("str", str_);
    vm.register2("strbase", strbase);
    vm.register1("subnormal?", is_subnormal);
    vm.register1("tan", tan);
    vm.register1("tanh", tanh);
    vm.register1("trunc", trunc);
    vm.register1("typeof", typeof_);
    vm.register1("upper", upper);
}
