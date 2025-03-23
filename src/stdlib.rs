use crate::val::*;
use crate::vm::*;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::Signed;
use num_traits::ToPrimitive;
use num_traits::Zero;
use rand::Rng;
use std::cell::RefCell;
use std::io;
use std::io::Write;
use std::rc::Rc;

fn input(_vm: &mut VM) -> Result<Val, String> {
    let mut s = String::new();
    // TODO
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut s).expect("Failed to read line");

    // Remove the trailing newline character
    let s = s.trim();

    Ok(Val::string(s))
}

fn dim(_vm: &mut VM, n: Val) -> Result<Val, String> {
    let n = match n.to_usize() {
        Some(n) => n,
        None => return Err("Expected integer length".to_string()),
    };
    let r = List::new(n + 1);
    let r = Val::List(Rc::new(RefCell::new(r)));
    Ok(r)
}

fn store_subscript(_vm: &mut VM, a: Val, i: Val, x: Val) -> Result<Val, String> {
    let i = match i.to_usize() {
        Some(i) => i,
        None => return Err("Invalid index".to_string()),
    };
    match a {
        Val::List(a) => {
            a.borrow_mut().v[i] = x;
        }
        _ => return Err("Invalid list".to_string()),
    };
    Ok(Val::Int(BigInt::zero()))
}

fn sqrt(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let r = match &a {
        Val::Float(a) => Val::Float(a.sqrt()),
        Val::Int(a) => Val::Int(a.sqrt()),
        _ => {
            return Err("Expected number".to_string());
        }
    };
    Ok(r)
}

fn to_float(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = match a.to_f64() {
        Some(a) => a,
        None => return Err("Unable to convert value".to_string()),
    };
    let r = Val::Float(a);
    Ok(r)
}

fn to_int(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = match a.to_bigint() {
        Some(a) => a,
        None => return Err("Unable to convert value".to_string()),
    };
    let r = Val::Int(a);
    Ok(r)
}

fn to_str(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let r = Val::Str(a.to_string().into());
    Ok(r)
}

fn _print(_vm: &mut VM, a: Val) -> Result<Val, String> {
    print!("{}", a);
    Ok(Val::Int(BigInt::from(0))) // Return 0 as a success indicator
}

fn typeof_val(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let r = match a {
        Val::Int(_) => "int",
        Val::Float(_) => "float",
        Val::Str(_) => "str",
        Val::List(_) => "list",
        _ => "fn",
    };
    Ok(Val::string(r))
}

fn _add(_vm: &mut VM, a: Val, b: Val) -> Result<Val, String> {
    let r = match (&a, &b) {
        (Val::Int(a), Val::Int(b)) => Val::Int(a + b),
        (Val::Float(a), Val::Float(b)) => Val::Float(a + b),
        (Val::Int(a), Val::Float(b)) => {
            let a = match a.to_f64() {
                Some(a) => a,
                None => return Err("Expected number".to_string()),
            };
            Val::Float(a + *b)
        }
        (Val::Float(a), Val::Int(b)) => {
            let b = match b.to_f64() {
                Some(b) => b,
                None => return Err("Expected number".to_string()),
            };
            Val::Float(*a + b)
        }
        _ => {
            let a = a.to_string();
            let b = b.to_string();
            let mut r = String::with_capacity(a.len() + b.len());
            r.push_str(&a);
            r.push_str(&b);
            Val::string(r)
        }
    };
    Ok(r)
}

fn _eq(_vm: &mut VM, a: Val, b: Val) -> Result<Val, String> {
    let r = Val::boolean(loose_eq(&a, &b));
    Ok(r)
}

fn _ne(_vm: &mut VM, a: Val, b: Val) -> Result<Val, String> {
    let r = Val::boolean(!loose_eq(&a, &b));
    Ok(r)
}

fn _lt(_vm: &mut VM, a: Val, b: Val) -> Result<Val, String> {
    let r = Val::boolean(loose_lt(&a, &b));
    Ok(r)
}

fn _gt(_vm: &mut VM, a: Val, b: Val) -> Result<Val, String> {
    let r = Val::boolean(loose_lt(&b, &a));
    Ok(r)
}

fn _le(_vm: &mut VM, a: Val, b: Val) -> Result<Val, String> {
    let r = Val::boolean(loose_le(&a, &b));
    Ok(r)
}

fn _ge(_vm: &mut VM, a: Val, b: Val) -> Result<Val, String> {
    let r = Val::boolean(loose_le(&b, &a));
    Ok(r)
}

fn _sub(_vm: &mut VM, a: Val, b: Val) -> Result<Val, String> {
    let r = match (&a, &b) {
        (Val::Int(a), Val::Int(b)) => Val::Int(a - b),
        _ => {
            let a = match a.to_f64() {
                Some(a) => a,
                None => return Err("Expected number".to_string()),
            };
            let b = match b.to_f64() {
                Some(b) => b,
                None => return Err("Expected number".to_string()),
            };
            Val::Float(a - b)
        }
    };
    Ok(r)
}

fn _neg(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let r = match &a {
        Val::Int(a) => Val::Int(-a),
        Val::Float(a) => Val::Float(-a),
        _ => {
            return Err("Expected number".to_string());
        }
    };
    Ok(r)
}

fn _fdiv(_vm: &mut VM, a: Val, b: Val) -> Result<Val, String> {
    let a = match a.to_f64() {
        Some(a) => a,
        None => return Err("Expected number".to_string()),
    };
    let b = match b.to_f64() {
        Some(b) => b,
        None => return Err("Expected number".to_string()),
    };
    let r = Val::Float(a / b);
    Ok(r)
}

fn total_cmp(_vm: &mut VM, a: Val, b: Val) -> Result<Val, String> {
    let a = match a.to_f64() {
        Some(a) => a,
        None => return Err("Expected number".to_string()),
    };
    let b = match b.to_f64() {
        Some(b) => b,
        None => return Err("Expected number".to_string()),
    };
    let cmp_result = match a.total_cmp(&b) {
        std::cmp::Ordering::Less => BigInt::from(-1),
        std::cmp::Ordering::Equal => BigInt::from(0),
        std::cmp::Ordering::Greater => BigInt::from(1),
    };
    let r = Val::Int(cmp_result);
    Ok(r)
}

fn copy_sign(_vm: &mut VM, a: Val, sign: Val) -> Result<Val, String> {
    let a = match a.to_f64() {
        Some(a) => a,
        None => return Err("Expected number".to_string()),
    };
    let sign = match sign.to_f64() {
        Some(sign) => sign,
        None => return Err("Expected number".to_string()),
    };
    let r = Val::Float(a.copysign(sign));
    Ok(r)
}

fn midpoint(_vm: &mut VM, a: Val, b: Val) -> Result<Val, String> {
    let a = match a.to_f64() {
        Some(a) => a,
        None => return Err("Expected number".to_string()),
    };
    let b = match b.to_f64() {
        Some(b) => b,
        None => return Err("Expected number".to_string()),
    };
    let r = Val::Float(a.midpoint(b));
    Ok(r)
}

fn _pow(_vm: &mut VM, a: Val, b: Val) -> Result<Val, String> {
    let r = match (&a, &b) {
        (Val::Int(a), Val::Int(b)) => match b.to_u32() {
            Some(b) => Val::Int(a.pow(b)),
            None => {
                return Err("Exponent out of range".to_string());
            }
        },
        _ => {
            let a = match a.to_f64() {
                Some(a) => a,
                None => return Err("Expected number".to_string()),
            };
            let b = match b.to_f64() {
                Some(b) => b,
                None => return Err("Expected number".to_string()),
            };
            Val::Float(a.powf(b))
        }
    };
    Ok(r)
}

fn _bitand(_vm: &mut VM, a: Val, b: Val) -> Result<Val, String> {
    let a = match a.to_bigint() {
        Some(a) => a,
        None => return Err("Expected integers".to_string()),
    };
    let b = match b.to_bigint() {
        Some(b) => b,
        None => return Err("Expected integers".to_string()),
    };
    let r = Val::Int(a & b);
    Ok(r)
}

fn _bitor(_vm: &mut VM, a: Val, b: Val) -> Result<Val, String> {
    let a = match a.to_bigint() {
        Some(a) => a,
        None => return Err("Expected integers".to_string()),
    };
    let b = match b.to_bigint() {
        Some(b) => b,
        None => return Err("Expected integers".to_string()),
    };
    let r = Val::Int(a | b);
    Ok(r)
}

fn _bitxor(_vm: &mut VM, a: Val, b: Val) -> Result<Val, String> {
    let a = match a.to_bigint() {
        Some(a) => a,
        None => return Err("Expected integers".to_string()),
    };
    let b = match b.to_bigint() {
        Some(b) => b,
        None => return Err("Expected integers".to_string()),
    };
    let r = Val::Int(a ^ b);
    Ok(r)
}

fn gcd(_vm: &mut VM, a: Val, b: Val) -> Result<Val, String> {
    let a = match a.to_bigint() {
        Some(a) => a,
        None => return Err("Expected integers".to_string()),
    };
    let b = match b.to_bigint() {
        Some(b) => b,
        None => return Err("Expected integers".to_string()),
    };
    let r = Val::Int(a.gcd(&b));
    Ok(r)
}

fn lcm(_vm: &mut VM, a: Val, b: Val) -> Result<Val, String> {
    let a = match a.to_bigint() {
        Some(a) => a,
        None => return Err("Expected integers".to_string()),
    };
    let b = match b.to_bigint() {
        Some(b) => b,
        None => return Err("Expected integers".to_string()),
    };
    let r = Val::Int(a.lcm(&b));
    Ok(r)
}

fn _shl(_vm: &mut VM, a: Val, b: Val) -> Result<Val, String> {
    let a = match a.to_bigint() {
        Some(a) => a,
        None => return Err("Expected integer".to_string()),
    };
    let b = match b.to_u32() {
        Some(b) => b,
        None => return Err("Shift amount not valid".to_string()),
    };

    let r = Val::Int(a << b);
    Ok(r)
}

fn str_base(_vm: &mut VM, a: Val, base: Val) -> Result<Val, String> {
    let a = match a.to_bigint() {
        Some(a) => a,
        None => return Err("Expected integer".to_string()),
    };
    let base = match base.to_u32() {
        Some(base) => base,
        None => return Err("Base not valid".to_string()),
    };

    let r = Val::Str(a.to_str_radix(base).into());
    Ok(r)
}

fn val_base(_vm: &mut VM, s: Val, base: Val) -> Result<Val, String> {
    let s = s.to_string();
    let base = match base.to_u32() {
        Some(base) => base,
        None => return Err("Base not valid".to_string()),
    };

    let r = match BigInt::parse_bytes(s.as_bytes(), base) {
        Some(a) => Val::Int(a),
        None => return Err("Unable to parse string".to_string()),
    };
    Ok(r)
}

fn _shr(_vm: &mut VM, a: Val, b: Val) -> Result<Val, String> {
    let a = match a.to_bigint() {
        Some(a) => a,
        None => return Err("Expected integer".to_string()),
    };
    let b = match b.to_u32() {
        Some(b) => b,
        None => return Err("Shift amount not valid".to_string()),
    };

    let r = Val::Int(a >> b);
    Ok(r)
}

fn _idiv(_vm: &mut VM, a: Val, b: Val) -> Result<Val, String> {
    let a = match a.to_bigint() {
        Some(a) => a,
        None => return Err("Expected integers".to_string()),
    };
    let b = match b.to_bigint() {
        Some(b) => b,
        None => return Err("Expected integers".to_string()),
    };
    let r = Val::Int(a / b);
    Ok(r)
}

fn mod_op(_vm: &mut VM, a: Val, b: Val) -> Result<Val, String> {
    let r = match (&a, &b) {
        (Val::Int(a), Val::Int(b)) => Val::Int(a % b),
        _ => {
            let a = match a.to_f64() {
                Some(a) => a,
                None => return Err("Expected number".to_string()),
            };
            let b = match b.to_f64() {
                Some(b) => b,
                None => return Err("Expected number".to_string()),
            };
            Val::Float(a % b)
        }
    };
    Ok(r)
}

fn _bitnot(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = match a.to_bigint() {
        Some(a) => a,
        None => return Err("Expected integers".to_string()),
    };
    let r = Val::Int(!a);
    Ok(r)
}

fn signum(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let r = match &a {
        Val::Float(a) => Val::Float(a.signum()),
        Val::Int(a) => Val::Int(a.signum()),
        _ => {
            return Err("Expected number".to_string());
        }
    };
    Ok(r)
}

fn abs(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let r = match &a {
        Val::Float(a) => Val::Float(a.abs()),
        Val::Int(a) => Val::Int(a.abs()),
        _ => {
            return Err("Expected number".to_string());
        }
    };
    Ok(r)
}

fn cbrt(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let r = match &a {
        Val::Float(a) => Val::Float(a.cbrt()),
        Val::Int(a) => Val::Int(a.cbrt()),
        _ => {
            return Err("Expected number".to_string());
        }
    };
    Ok(r)
}

fn _not(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let r = Val::boolean(!a.truth());
    Ok(r)
}

fn make_list(_vm: &mut VM, items: Vec<Val>) -> Result<Val, String> {
    let r = List::from(items);
    let r = Val::List(Rc::new(RefCell::new(r)));
    Ok(r)
}

fn subscript(_vm: &mut VM, a: Val, i: Val) -> Result<Val, String> {
    let i = match i.to_usize() {
        Some(i) => i,
        None => return Err("Invalid index".to_string()),
    };
    let r = match a {
        Val::List(a) => a.borrow().v[i].clone(),
        Val::Str(a) => {
            let chars: Vec<char> = a.chars().collect();
            if i < chars.len() {
                Val::Str(chars[i].to_string().into())
            } else {
                Val::Str(String::new().into())
            }
        }
        _ => return Err("Invalid list".to_string()),
    };

    Ok(r)
}

fn _mul(_vm: &mut VM, a: Val, b: Val) -> Result<Val, String> {
    let r = match (&a, &b) {
        (Val::Int(a), Val::Int(b)) => Val::Int(a.clone() * b.clone()),
        (Val::Int(a), Val::Float(b)) => match a.to_f64() {
            Some(a) => Val::Float(a * b),
            None => return Err("Integer too large to convert to float".to_string()),
        },
        (Val::Float(a), Val::Int(b)) => match b.to_f64() {
            Some(b) => Val::Float(a * b),
            None => return Err("Integer too large to convert to float".to_string()),
        },
        (Val::Float(a), Val::Float(b)) => Val::Float(*a * *b),
        (Val::Int(a), Val::Str(b)) => {
            let a = match usize::try_from(a.clone()) {
                Ok(a) => a,
                Err(_) => {
                    return Err("Repeat count out of range".to_string());
                }
            };
            Val::string(b.repeat(a))
        }
        (Val::Str(a), Val::Int(b)) => {
            let b = match usize::try_from(b.clone()) {
                Ok(b) => b,
                Err(_) => {
                    return Err("Repeat count out of range".to_string());
                }
            };
            Val::string(a.repeat(b))
        }
        _ => {
            return Err("*: expected numbers".to_string());
        }
    };
    Ok(r)
}

fn exit(_vm: &mut VM, a: Val) -> Result<Val, String> {
    Ok(a)
}

fn rnd(vm: &mut VM, _a: Val) -> Result<Val, String> {
    let r: f64 = vm.rng.random();
    Ok(Val::Float(r))
}

fn floor(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = match a.to_f64() {
        Some(a) => a,
        None => return Err("Expected number".to_string()),
    };
    let r = Val::Float(a.floor());
    Ok(r)
}

fn ceil(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = match a.to_f64() {
        Some(a) => a,
        None => return Err("Expected number".to_string()),
    };
    let r = Val::Float(a.ceil());
    Ok(r)
}

fn round(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = match a.to_f64() {
        Some(a) => a,
        None => return Err("Expected number".to_string()),
    };
    let r = Val::Float(a.round());
    Ok(r)
}

fn round_ties_even(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = match a.to_f64() {
        Some(a) => a,
        None => return Err("Expected number".to_string()),
    };
    let r = Val::Float(a.round_ties_even());
    Ok(r)
}

fn trunc(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = match a.to_f64() {
        Some(a) => a,
        None => return Err("Expected number".to_string()),
    };
    let r = Val::Float(a.trunc());
    Ok(r)
}

fn fract(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = match a.to_f64() {
        Some(a) => a,
        None => return Err("Expected number".to_string()),
    };
    let r = Val::Float(a.fract());
    Ok(r)
}

fn mul_add(_vm: &mut VM, a: Val, b: Val, c: Val) -> Result<Val, String> {
    let a = match a.to_f64() {
        Some(a) => a,
        None => return Err("Expected number for first argument".to_string()),
    };
    let b = match b.to_f64() {
        Some(b) => b,
        None => return Err("Expected number for second argument".to_string()),
    };
    let c = match c.to_f64() {
        Some(c) => c,
        None => return Err("Expected number for third argument".to_string()),
    };
    let r = Val::Float(a.mul_add(b, c)); // a * b + c
    Ok(r)
}

fn div_euclid(_vm: &mut VM, a: Val, b: Val) -> Result<Val, String> {
    let a = match a.to_f64() {
        Some(a) => a,
        None => return Err("Expected number for dividend".to_string()),
    };
    let b = match b.to_f64() {
        Some(b) => b,
        None => return Err("Expected number for divisor".to_string()),
    };
    let r = Val::Float(a.div_euclid(b));
    Ok(r)
}

fn rem_euclid(_vm: &mut VM, a: Val, b: Val) -> Result<Val, String> {
    let a = match a.to_f64() {
        Some(a) => a,
        None => return Err("Expected number for dividend".to_string()),
    };
    let b = match b.to_f64() {
        Some(b) => b,
        None => return Err("Expected number for divisor".to_string()),
    };
    let r = Val::Float(a.rem_euclid(b));
    Ok(r)
}

fn pow_i(_vm: &mut VM, a: Val, b: Val) -> Result<Val, String> {
    let a = match a.to_f64() {
        Some(a) => a,
        None => return Err("Expected number for base".to_string()),
    };
    let b = match b.to_i32() {
        Some(b) => b,
        None => return Err("Expected integer for exponent".to_string()),
    };
    let r = Val::Float(a.powi(b));
    Ok(r)
}

fn exp(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = match a.to_f64() {
        Some(a) => a,
        None => return Err("Expected number".to_string()),
    };
    let r = Val::Float(a.exp());
    Ok(r)
}

fn exp2(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = match a.to_f64() {
        Some(a) => a,
        None => return Err("Expected number".to_string()),
    };
    let r = Val::Float(a.exp2());
    Ok(r)
}

fn ln(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = match a.to_f64() {
        Some(a) => a,
        None => return Err("Expected number".to_string()),
    };
    let r = Val::Float(a.ln());
    Ok(r)
}

fn log(_vm: &mut VM, a: Val, b: Val) -> Result<Val, String> {
    let a = match a.to_f64() {
        Some(a) => a,
        None => return Err("Expected number for value".to_string()),
    };
    let b = match b.to_f64() {
        Some(b) => b,
        None => return Err("Expected number for base".to_string()),
    };
    let r = Val::Float(a.log(b));
    Ok(r)
}

fn log2(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = match a.to_f64() {
        Some(a) => a,
        None => return Err("Expected number".to_string()),
    };
    let r = Val::Float(a.log2());
    Ok(r)
}

fn log10(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = match a.to_f64() {
        Some(a) => a,
        None => return Err("Expected number".to_string()),
    };
    let r = Val::Float(a.log10());
    Ok(r)
}

fn hypot(_vm: &mut VM, a: Val, b: Val) -> Result<Val, String> {
    let a = match a.to_f64() {
        Some(a) => a,
        None => return Err("Expected number for first argument".to_string()),
    };
    let b = match b.to_f64() {
        Some(b) => b,
        None => return Err("Expected number for second argument".to_string()),
    };
    let r = Val::Float(a.hypot(b));
    Ok(r)
}

fn sin(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = match a.to_f64() {
        Some(a) => a,
        None => return Err("Expected number".to_string()),
    };
    let r = Val::Float(a.sin());
    Ok(r)
}

fn cos(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = match a.to_f64() {
        Some(a) => a,
        None => return Err("Expected number".to_string()),
    };
    let r = Val::Float(a.cos());
    Ok(r)
}

fn tan(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = match a.to_f64() {
        Some(a) => a,
        None => return Err("Expected number".to_string()),
    };
    let r = Val::Float(a.tan());
    Ok(r)
}

fn asin(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = match a.to_f64() {
        Some(a) => a,
        None => return Err("Expected number".to_string()),
    };
    let r = Val::Float(a.asin());
    Ok(r)
}

fn acos(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = match a.to_f64() {
        Some(a) => a,
        None => return Err("Expected number".to_string()),
    };
    let r = Val::Float(a.acos());
    Ok(r)
}

fn atan(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = match a.to_f64() {
        Some(a) => a,
        None => return Err("Expected number".to_string()),
    };
    let r = Val::Float(a.atan());
    Ok(r)
}

fn atan2(_vm: &mut VM, a: Val, b: Val) -> Result<Val, String> {
    let a = match a.to_f64() {
        Some(a) => a,
        None => return Err("Expected number for first argument".to_string()),
    };
    let b = match b.to_f64() {
        Some(b) => b,
        None => return Err("Expected number for second argument".to_string()),
    };
    let r = Val::Float(a.atan2(b));
    Ok(r)
}

fn exp_m1(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = match a.to_f64() {
        Some(a) => a,
        None => return Err("Expected number".to_string()),
    };
    let r = Val::Float(a.exp_m1());
    Ok(r)
}

fn ln_1p(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = match a.to_f64() {
        Some(a) => a,
        None => return Err("Expected number".to_string()),
    };
    let r = Val::Float(a.ln_1p());
    Ok(r)
}

fn sinh(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = match a.to_f64() {
        Some(a) => a,
        None => return Err("Expected number".to_string()),
    };
    let r = Val::Float(a.sinh());
    Ok(r)
}

fn cosh(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = match a.to_f64() {
        Some(a) => a,
        None => return Err("Expected number".to_string()),
    };
    let r = Val::Float(a.cosh());
    Ok(r)
}

fn tanh(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = match a.to_f64() {
        Some(a) => a,
        None => return Err("Expected number".to_string()),
    };
    let r = Val::Float(a.tanh());
    Ok(r)
}

fn asinh(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = match a.to_f64() {
        Some(a) => a,
        None => return Err("Expected number".to_string()),
    };
    let r = Val::Float(a.asinh());
    Ok(r)
}
fn acosh(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = match a.to_f64() {
        Some(a) => a,
        None => return Err("Expected number".to_string()),
    };
    let r = Val::Float(a.acosh());
    Ok(r)
}

fn atanh(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = match a.to_f64() {
        Some(a) => a,
        None => return Err("Expected number".to_string()),
    };
    let r = Val::Float(a.atanh());
    Ok(r)
}

fn is_nan(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = match a.to_f64() {
        Some(a) => a,
        None => return Err("Expected number".to_string()),
    };
    let r = Val::boolean(a.is_nan());
    Ok(r)
}

fn is_finite(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = match a.to_f64() {
        Some(a) => a,
        None => return Err("Expected number".to_string()),
    };
    let r = Val::boolean(a.is_finite());
    Ok(r)
}

fn is_infinite(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = match a.to_f64() {
        Some(a) => a,
        None => return Err("Expected number".to_string()),
    };
    let r = Val::boolean(a.is_infinite());
    Ok(r)
}

fn is_subnormal(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = match a.to_f64() {
        Some(a) => a,
        None => return Err("Expected number".to_string()),
    };
    let r = Val::boolean(a.is_subnormal());
    Ok(r)
}

fn is_normal(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = match a.to_f64() {
        Some(a) => a,
        None => return Err("Expected number".to_string()),
    };
    let r = Val::boolean(a.is_normal());
    Ok(r)
}

fn is_sign_positive(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = match a.to_f64() {
        Some(a) => a,
        None => return Err("Expected number".to_string()),
    };
    let r = Val::boolean(a.is_sign_positive());
    Ok(r)
}

fn is_sign_negative(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = match a.to_f64() {
        Some(a) => a,
        None => return Err("Expected number".to_string()),
    };
    let r = Val::boolean(a.is_sign_negative());
    Ok(r)
}

fn recip(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = match a.to_f64() {
        Some(a) => a,
        None => return Err("Expected number".to_string()),
    };
    let r = Val::Float(a.recip());
    Ok(r)
}

fn to_degrees(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = match a.to_f64() {
        Some(a) => a,
        None => return Err("Expected number".to_string()),
    };
    let r = Val::Float(a.to_degrees());
    Ok(r)
}

fn to_radians(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = match a.to_f64() {
        Some(a) => a,
        None => return Err("Expected number".to_string()),
    };
    let r = Val::Float(a.to_radians());
    Ok(r)
}

fn nth_root(_vm: &mut VM, a: Val, b: Val) -> Result<Val, String> {
    let a = match a.to_bigint() {
        Some(a) => a,
        None => return Err("Expected integer".to_string()),
    };
    let b = match b.to_u32() {
        Some(b) => b,
        None => return Err("N out of range".to_string()),
    };
    let r = Val::Int(a.nth_root(b));
    Ok(r)
}

fn trailing_zeros(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = match a.to_bigint() {
        Some(a) => a,
        None => return Err("Expected integer".to_string()),
    };
    let r = a.trailing_zeros().unwrap_or_default();
    let r = Val::Int(BigInt::from(r));
    Ok(r)
}

fn bit(_vm: &mut VM, a: Val, b: Val) -> Result<Val, String> {
    let a = match a.to_bigint() {
        Some(a) => a,
        None => return Err("Expected integer".to_string()),
    };
    let b = match b.to_u64() {
        Some(b) => b,
        None => return Err("Bit out of range".to_string()),
    };
    let r = Val::boolean(a.bit(b));
    Ok(r)
}

fn set_bit(_vm: &mut VM, a: Val, bit: Val, value: Val) -> Result<Val, String> {
    let a = match a.to_bigint() {
        Some(a) => a,
        None => return Err("Expected integer".to_string()),
    };
    let bit = match bit.to_u64() {
        Some(bit) => bit,
        None => return Err("Bit out of range".to_string()),
    };
    let value = value.truth();

    let mut r = a.clone();
    r.set_bit(bit, value);

    let r = Val::Int(r);
    Ok(r)
}

fn max(_vm: &mut VM, a: Val, b: Val) -> Result<Val, String> {
    let r = match (&a, &b) {
        (Val::Float(a), Val::Float(b)) => Val::Float(a.max(*b)),
        _ => {
            if loose_lt(&b, &a) {
                a
            } else {
                b
            }
        }
    };
    Ok(r)
}

fn min(_vm: &mut VM, a: Val, b: Val) -> Result<Val, String> {
    let r = match (&a, &b) {
        (Val::Float(a), Val::Float(b)) => Val::Float(a.min(*b)),
        _ => {
            if loose_lt(&a, &b) {
                a
            } else {
                b
            }
        }
    };
    Ok(r)
}

fn clamp(_vm: &mut VM, a: Val, min: Val, max: Val) -> Result<Val, String> {
    let r = match (&a, &min, &max) {
        (Val::Int(a), Val::Int(min), Val::Int(max)) => {
            let r = if a < min {
                min
            } else if max < a {
                max
            } else {
                a
            };
            Val::Int(r.clone())
        }
        _ => {
            let a = match a.to_f64() {
                Some(a) => a,
                None => return Err("Expected number".to_string()),
            };
            let min = match min.to_f64() {
                Some(min) => min,
                None => return Err("Expected number".to_string()),
            };
            let max = match max.to_f64() {
                Some(max) => max,
                None => return Err("Expected number".to_string()),
            };
            Val::Float(a.clamp(min, max))
        }
    };
    Ok(r)
}

fn len(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let len = match &a {
        Val::List(a) => a.borrow().v.len(),
        _ => a.to_string().len(),
    };
    Ok(Val::Int(BigInt::from(len)))
}

fn left(_vm: &mut VM, s: Val, n: Val) -> Result<Val, String> {
    let s = s.to_string();
    let n = match n.to_u64() {
        Some(n) => n as usize,
        None => return Err("Expected non-negative integer for length".to_string()),
    };

    let result = if n >= s.len() {
        s
    } else {
        // Safe to index since we verified n < s.len()
        s[..n].to_string()
    };

    Ok(Val::string(result))
}

fn right(_vm: &mut VM, s: Val, n: Val) -> Result<Val, String> {
    let s = s.to_string();
    let n = match n.to_u64() {
        Some(n) => n as usize,
        None => return Err("Expected non-negative integer for length".to_string()),
    };

    let result = if n >= s.len() {
        s
    } else {
        // Safe to index since we verified n < s.len()
        s[s.len() - n..].to_string()
    };

    Ok(Val::string(result))
}

fn mid(_vm: &mut VM, s: Val, start: Val, len: Val) -> Result<Val, String> {
    let s = s.to_string();

    // In BASIC, string indices are typically 1-based
    let start = match start.to_u64() {
        Some(start) => start as usize,
        None => {
            return Err("Expected non-negative integer for start position".to_string());
        }
    };

    // Adjust to 0-based indexing
    let start = if start > 0 { start - 1 } else { 0 };

    // Handle out of bounds start
    if start >= s.len() {
        return Ok(Val::string(""));
    }

    let len = match len.to_u64() {
        Some(len) => len as usize,
        None => {
            return Err("Expected non-negative integer for length".to_string());
        }
    };

    // Calculate end position, ensuring we don't go past the end of the string
    let end = std::cmp::min(start + len, s.len());

    // Extract the substring
    let result = s[start..end].to_string();
    Ok(Val::string(result))
}

fn asc(_vm: &mut VM, s: Val) -> Result<Val, String> {
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
    let code_point = match n.to_u32() {
        Some(n) => n,
        None => {
            return Err("Expected non-negative integer for character code".to_string());
        }
    };

    let c = match std::char::from_u32(code_point) {
        Some(c) => c,
        None => return Err("Invalid character code".to_string()),
    };

    Ok(Val::string(c.to_string()))
}

fn instr(_vm: &mut VM, s: Val, find: Val) -> Result<Val, String> {
    let s = s.to_string();
    let find = find.to_string();

    // In BASIC, InStr returns 0 if not found, position otherwise (1-based)
    let position = match s.find(&find) {
        Some(pos) => pos + 1, // Converting to 1-based indexing
        None => 0,
    };

    Ok(Val::Int(BigInt::from(position)))
}

fn ucase(_vm: &mut VM, s: Val) -> Result<Val, String> {
    let s = s.to_string();
    Ok(Val::string(s.to_uppercase()))
}

fn lcase(_vm: &mut VM, s: Val) -> Result<Val, String> {
    let s = s.to_string();
    Ok(Val::string(s.to_lowercase()))
}

// Register all functions to the VM
pub fn register_all(vm: &mut VM) {
    vm.register2("_add", _add);
    vm.register2("_bitand", _bitand);
    vm.register1("_bitnot", _bitnot);
    vm.register2("_bitor", _bitor);
    vm.register2("_bitxor", _bitxor);
    vm.register2("_eq", _eq);
    vm.register2("_fdiv", _fdiv);
    vm.register2("_ge", _ge);
    vm.register2("_gt", _gt);
    vm.register2("_idiv", _idiv);
    vm.register2("_le", _le);
    vm.register2("_lt", _lt);
    vm.register2("_mul", _mul);
    vm.register2("_ne", _ne);
    vm.register1("_neg", _neg);
    vm.register1("_not", _not);
    vm.register2("_pow", _pow);
    vm.register1("_print", _print);
    vm.register2("_shl", _shl);
    vm.register2("_shr", _shr);
    vm.register2("_sub", _sub);
    vm.register1("abs", abs);
    vm.register1("acos", acos);
    vm.register1("acosh", acosh);
    vm.register1("asc", asc);
    vm.register1("asin", asin);
    vm.register1("asinh", asinh);
    vm.register1("atan", atan);
    vm.register2("atan2", atan2);
    vm.register1("atanh", atanh);
    vm.register2("bit", bit);
    vm.register1("cbrt", cbrt);
    vm.register1("ceil", ceil);
    vm.register1("chr", chr);
    vm.register3("clamp", clamp);
    vm.register2("copy_sign", copy_sign);
    vm.register1("cos", cos);
    vm.register1("cosh", cosh);
    vm.register1("dim", dim);
    vm.register2("div_euclid", div_euclid);
    vm.register1("exit", exit);
    vm.register1("exp", exp);
    vm.register1("exp2", exp2);
    vm.register1("exp_m1", exp_m1);
    vm.register1("floor", floor);
    vm.register1("fract", fract);
    vm.register2("gcd", gcd);
    vm.register2("hypot", hypot);
    vm.register0("input", input);
    vm.register2("instr", instr);
    vm.register1("is_finite", is_finite);
    vm.register1("is_infinite", is_infinite);
    vm.register1("is_nan", is_nan);
    vm.register1("is_normal", is_normal);
    vm.register1("is_sign_negative", is_sign_negative);
    vm.register1("is_sign_positive", is_sign_positive);
    vm.register1("is_subnormal", is_subnormal);
    vm.register1("lcase", lcase);
    vm.register2("lcm", lcm);
    vm.register2("left", left);
    vm.register1("len", len);
    vm.register1("ln", ln);
    vm.register1("ln_1p", ln_1p);
    vm.register2("log", log);
    vm.register1("log10", log10);
    vm.register1("log2", log2);
    vm.registerv("make_list", make_list);
    vm.register2("max", max);
    vm.register3("mid", mid);
    vm.register2("midpoint", midpoint);
    vm.register2("min", min);
    vm.register2("mod_op", mod_op);
    vm.register3("mul_add", mul_add);
    vm.register2("nth_root", nth_root);
    vm.register2("pow_i", pow_i);
    vm.register1("recip", recip);
    vm.register2("rem_euclid", rem_euclid);
    vm.register2("right", right);
    vm.register1("round", round);
    vm.register1("round_ties_even", round_ties_even);
    vm.register3("set_bit", set_bit);
    vm.register1("signum", signum);
    vm.register1("sin", sin);
    vm.register1("sinh", sinh);
    vm.register1("sqrt", sqrt);
    vm.register3("store_subscript", store_subscript);
    vm.register2("str_base", str_base);
    vm.register2("subscript", subscript);
    vm.register1("tan", tan);
    vm.register1("tanh", tanh);
    vm.register1("to_degrees", to_degrees);
    vm.register1("to_float", to_float);
    vm.register1("to_int", to_int);
    vm.register1("to_radians", to_radians);
    vm.register1("to_str", to_str);
    vm.register2("total_cmp", total_cmp);
    vm.register1("trailing_zeros", trailing_zeros);
    vm.register1("trunc", trunc);
    vm.register1("typeof_val", typeof_val);
    vm.register1("ucase", ucase);
    vm.register2("val_base", val_base);
}
