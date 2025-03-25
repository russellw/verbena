use crate::val::*;
use crate::vm::*;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::One;
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

fn sqrt(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.num()?;
    let r = match a {
        Val::Float(a) => Val::Float(a.sqrt()),
        Val::Int(a) => Val::Int(a.sqrt()),
        _ => panic!(),
    };
    Ok(r)
}

fn float(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let r = match a {
        Val::True => 1.0,
        Val::False => 0.0,
        Val::Int(a) => a.to_f64().unwrap(),
        Val::Float(a) => a,
        Val::Str(s) => match s.parse::<f64>() {
            Ok(a) => a,
            Err(e) => return Err(e.to_string()),
        },
        _ => return Err("Not a number".to_string()),
    };
    let r = Val::Float(r);
    Ok(r)
}

fn int(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let r = match a {
        Val::True => BigInt::one(),
        Val::False => BigInt::zero(),
        Val::Int(a) => a.clone(),
        Val::Float(a) => {
            if !a.is_finite() {
                return Err("Not a finite number".to_string());
            }
            BigInt::from(a as i128)
        }
        Val::Str(s) => match s.parse::<BigInt>() {
            Ok(a) => a,
            Err(e) => return Err(e.to_string()),
        },
        _ => return Err("Not a number".to_string()),
    };
    let r = Val::Int(r);
    Ok(r)
}

fn str(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let r = Val::Str(a.to_string().into());
    Ok(r)
}

fn _print(_vm: &mut VM, a: Val) -> Result<Val, String> {
    print!("{}", a);
    Ok(Val::Int(BigInt::from(0))) // TODO Return 0 as a success indicator
}

fn typeof_(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let r = match a {
        Val::Int(_) => "int",
        Val::Float(_) => "float",
        Val::Str(_) => "str",
        Val::List(_) => "list",
        Val::True | Val::False => "bool",
        Val::Null => "null",
        _ => "fn",
    };
    Ok(Val::string(r))
}

fn _add(_vm: &mut VM, a: Val, b: Val) -> Result<Val, String> {
    let (a, b) = num2_loose(&a, &b);
    let r = match (&a, &b) {
        (Val::Int(a), Val::Int(b)) => Val::Int(a + b),
        (Val::Float(a), Val::Float(b)) => Val::Float(a + b),
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
    let r = Val::boolean(eq_loose(&a, &b));
    Ok(r)
}

fn _ne(_vm: &mut VM, a: Val, b: Val) -> Result<Val, String> {
    let r = Val::boolean(!eq_loose(&a, &b));
    Ok(r)
}

fn _lt(_vm: &mut VM, a: Val, b: Val) -> Result<Val, String> {
    let r = Val::boolean(lt_loose(&a, &b));
    Ok(r)
}

fn _gt(_vm: &mut VM, a: Val, b: Val) -> Result<Val, String> {
    let r = Val::boolean(lt_loose(&b, &a));
    Ok(r)
}

fn _le(_vm: &mut VM, a: Val, b: Val) -> Result<Val, String> {
    let r = Val::boolean(le_loose(&a, &b));
    Ok(r)
}

fn _ge(_vm: &mut VM, a: Val, b: Val) -> Result<Val, String> {
    let r = Val::boolean(le_loose(&b, &a));
    Ok(r)
}

fn _sub(_vm: &mut VM, a: Val, b: Val) -> Result<Val, String> {
    // TODO: refactor errors
    let (a, b) = num2(&a, &b)?;
    let r = match (&a, &b) {
        (Val::Int(a), Val::Int(b)) => Val::Int(a - b),
        (Val::Float(a), Val::Float(b)) => Val::Float(a - b),
        _ => panic!(),
    };
    Ok(r)
}

fn _neg(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.num()?;
    let r = match a {
        Val::Int(a) => Val::Int(-a),
        Val::Float(a) => Val::Float(-a),
        _ => panic!(),
    };
    Ok(r)
}

fn _fdiv(_vm: &mut VM, a: Val, b: Val) -> Result<Val, String> {
    let a = a.to_f64()?;
    let b = b.to_f64()?;
    let r = Val::Float(a / b);
    Ok(r)
}

fn cmp(_vm: &mut VM, a: Val, b: Val) -> Result<Val, String> {
    let (a, b) = num2(&a, &b)?;
    let r = match (&a, &b) {
        (Val::Int(a), Val::Int(b)) => a.cmp(&b),
        (Val::Float(a), Val::Float(b)) => match a.partial_cmp(&b) {
            Some(r) => r,
            None => return Err("Not ordered".to_string()),
        },
        _ => panic!(),
    };
    let r = match r {
        std::cmp::Ordering::Less => BigInt::from(-1),
        std::cmp::Ordering::Equal => BigInt::from(0),
        std::cmp::Ordering::Greater => BigInt::from(1),
    };
    let r = Val::Int(r);
    Ok(r)
}

fn total_cmp(_vm: &mut VM, a: Val, b: Val) -> Result<Val, String> {
    let a = a.to_f64()?;
    let b = b.to_f64()?;
    let r = a.total_cmp(&b);
    let r = match r {
        std::cmp::Ordering::Less => BigInt::from(-1),
        std::cmp::Ordering::Equal => BigInt::from(0),
        std::cmp::Ordering::Greater => BigInt::from(1),
    };
    let r = Val::Int(r);
    Ok(r)
}

fn copy_sign(_vm: &mut VM, a: Val, sign: Val) -> Result<Val, String> {
    let a = a.to_f64()?;
    let sign = sign.to_f64()?;
    let r = Val::Float(a.copysign(sign));
    Ok(r)
}

fn midpoint(_vm: &mut VM, a: Val, b: Val) -> Result<Val, String> {
    let a = a.to_f64()?;
    let b = b.to_f64()?;
    let r = Val::Float(a.midpoint(b));
    Ok(r)
}

fn _pow(_vm: &mut VM, a: Val, b: Val) -> Result<Val, String> {
    let (a, b) = num2(&a, &b)?;
    let r = match (&a, &b) {
        (Val::Int(a), Val::Int(_)) => {
            let b = b.to_u32()?;
            Val::Int(a.pow(b))
        }
        (Val::Float(a), Val::Float(b)) => Val::Float(a.powf(*b)),
        _ => panic!(),
    };
    Ok(r)
}

fn _bit_and(_vm: &mut VM, a: Val, b: Val) -> Result<Val, String> {
    let a = a.to_bigint()?;
    let b = b.to_bigint()?;
    let r = Val::Int(a & b);
    Ok(r)
}

fn _bit_or(_vm: &mut VM, a: Val, b: Val) -> Result<Val, String> {
    let a = a.to_bigint()?;
    let b = b.to_bigint()?;
    let r = Val::Int(a | b);
    Ok(r)
}

fn _bit_xor(_vm: &mut VM, a: Val, b: Val) -> Result<Val, String> {
    let a = a.to_bigint()?;
    let b = b.to_bigint()?;
    let r = Val::Int(a ^ b);
    Ok(r)
}

fn gcd(_vm: &mut VM, a: Val, b: Val) -> Result<Val, String> {
    let a = a.to_bigint()?;
    let b = b.to_bigint()?;
    let r = Val::Int(a.gcd(&b));
    Ok(r)
}

fn lcm(_vm: &mut VM, a: Val, b: Val) -> Result<Val, String> {
    let a = a.to_bigint()?;
    let b = b.to_bigint()?;
    let r = Val::Int(a.lcm(&b));
    Ok(r)
}

fn _shl(_vm: &mut VM, a: Val, b: Val) -> Result<Val, String> {
    let a = a.to_bigint()?;
    let b = b.to_u32()?;
    let r = Val::Int(a << b);
    Ok(r)
}

fn str_base(_vm: &mut VM, a: Val, base: Val) -> Result<Val, String> {
    let a = a.to_bigint()?;
    let base = base.to_u32()?;
    let r = Val::Str(a.to_str_radix(base).into());
    Ok(r)
}

fn int_base(_vm: &mut VM, s: Val, base: Val) -> Result<Val, String> {
    let s = s.to_string();
    let base = base.to_u32()?;
    let r = match BigInt::parse_bytes(s.as_bytes(), base) {
        Some(a) => Val::Int(a),
        None => return Err("Unable to parse integer".to_string()),
    };
    Ok(r)
}

fn _shr(_vm: &mut VM, a: Val, b: Val) -> Result<Val, String> {
    let a = a.to_bigint()?;
    let b = b.to_u32()?;
    let r = Val::Int(a >> b);
    Ok(r)
}

fn _idiv(_vm: &mut VM, a: Val, b: Val) -> Result<Val, String> {
    let a = a.to_bigint()?;
    let b = b.to_bigint()?;
    let r = Val::Int(a / b);
    Ok(r)
}

fn _mod(_vm: &mut VM, a: Val, b: Val) -> Result<Val, String> {
    let (a, b) = num2(&a, &b)?;
    let r = match (&a, &b) {
        (Val::Int(a), Val::Int(b)) => Val::Int(a % b),
        (Val::Float(a), Val::Float(b)) => Val::Float(a % b),
        _ => panic!(),
    };
    Ok(r)
}

fn _bit_not(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.to_bigint()?;
    let r = Val::Int(!a);
    Ok(r)
}

fn signum(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.num()?;
    let r = match &a {
        Val::Float(a) => Val::Float(a.signum()),
        Val::Int(a) => Val::Int(a.signum()),
        _ => panic!(),
    };
    Ok(r)
}

fn abs(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.num()?;
    let r = match &a {
        Val::Float(a) => Val::Float(a.abs()),
        Val::Int(a) => Val::Int(a.abs()),
        _ => panic!(),
    };
    Ok(r)
}

fn cbrt(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.num()?;
    let r = match &a {
        Val::Float(a) => Val::Float(a.cbrt()),
        Val::Int(a) => Val::Int(a.cbrt()),
        _ => panic!(),
    };
    Ok(r)
}

fn _not(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let r = Val::boolean(!a.truth());
    Ok(r)
}

fn _assert(_vm: &mut VM, a: Val) -> Result<Val, String> {
    if !a.truth() {
        return Err("Assert failed".to_string());
    }
    Ok(Val::Null)
}

fn _list(_vm: &mut VM, items: Vec<Val>) -> Result<Val, String> {
    let r = List::from(items);
    let r = Val::List(Rc::new(RefCell::new(r)));
    Ok(r)
}

fn _mul(_vm: &mut VM, a: Val, b: Val) -> Result<Val, String> {
    let (a, b) = num2_loose(&a, &b);
    let r = match (&a, &b) {
        (Val::Int(a), Val::Int(b)) => Val::Int(a.clone() * b.clone()),
        (Val::Float(a), Val::Float(b)) => Val::Float(*a * *b),
        (Val::Int(_), Val::Str(s)) => {
            let n = a.to_usize()?;
            Val::string(s.repeat(n))
        }
        (Val::Str(s), Val::Int(_)) => {
            let n = b.to_usize()?;
            Val::string(s.repeat(n))
        }
        (Val::Int(_), Val::List(v)) => {
            let n = a.to_usize()?;
            Val::List(Rc::new(RefCell::new(v.borrow().repeat(n))))
        }
        (Val::List(v), Val::Int(_)) => {
            let n = b.to_usize()?;
            Val::List(Rc::new(RefCell::new(v.borrow().repeat(n))))
        }
        _ => {
            return Err("Not numbers".to_string());
        }
    };
    Ok(r)
}

fn rnd(vm: &mut VM) -> Result<Val, String> {
    let r: f64 = vm.rng.random();
    Ok(Val::Float(r))
}

fn floor(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.to_f64()?;
    let r = Val::Float(a.floor());
    Ok(r)
}

fn ceil(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.to_f64()?;
    let r = Val::Float(a.ceil());
    Ok(r)
}

fn round(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.to_f64()?;
    let r = Val::Float(a.round());
    Ok(r)
}

fn round_ties_even(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.to_f64()?;
    let r = Val::Float(a.round_ties_even());
    Ok(r)
}

fn trunc(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.to_f64()?;
    let r = Val::Float(a.trunc());
    Ok(r)
}

fn fract(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.to_f64()?;
    let r = Val::Float(a.fract());
    Ok(r)
}

fn mul_add(_vm: &mut VM, a: Val, b: Val, c: Val) -> Result<Val, String> {
    let a = a.to_f64()?;
    let b = b.to_f64()?;
    let c = c.to_f64()?;
    let r = Val::Float(a.mul_add(b, c));
    Ok(r)
}

fn div_euclid(_vm: &mut VM, a: Val, b: Val) -> Result<Val, String> {
    let a = a.to_f64()?;
    let b = b.to_f64()?;
    let r = Val::Float(a.div_euclid(b));
    Ok(r)
}

fn rem_euclid(_vm: &mut VM, a: Val, b: Val) -> Result<Val, String> {
    let a = a.to_f64()?;
    let b = b.to_f64()?;
    let r = Val::Float(a.rem_euclid(b));
    Ok(r)
}

fn pow_i(_vm: &mut VM, a: Val, b: Val) -> Result<Val, String> {
    let a = a.to_f64()?;
    let b = b.to_i32()?;
    let r = Val::Float(a.powi(b));
    Ok(r)
}

fn exp(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.to_f64()?;
    let r = Val::Float(a.exp());
    Ok(r)
}

fn exp2(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.to_f64()?;
    let r = Val::Float(a.exp2());
    Ok(r)
}

fn ln(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.to_f64()?;
    let r = Val::Float(a.ln());
    Ok(r)
}

fn log(_vm: &mut VM, a: Val, b: Val) -> Result<Val, String> {
    let a = a.to_f64()?;
    let b = b.to_f64()?;
    let r = Val::Float(a.log(b));
    Ok(r)
}

fn log2(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.to_f64()?;
    let r = Val::Float(a.log2());
    Ok(r)
}

fn log10(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.to_f64()?;
    let r = Val::Float(a.log10());
    Ok(r)
}

fn hypot(_vm: &mut VM, a: Val, b: Val) -> Result<Val, String> {
    let a = a.to_f64()?;
    let b = b.to_f64()?;
    let r = Val::Float(a.hypot(b));
    Ok(r)
}

fn sin(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.to_f64()?;
    let r = Val::Float(a.sin());
    Ok(r)
}

fn cos(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.to_f64()?;
    let r = Val::Float(a.cos());
    Ok(r)
}

fn tan(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.to_f64()?;
    let r = Val::Float(a.tan());
    Ok(r)
}

fn asin(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.to_f64()?;
    let r = Val::Float(a.asin());
    Ok(r)
}

fn acos(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.to_f64()?;
    let r = Val::Float(a.acos());
    Ok(r)
}

fn atan(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.to_f64()?;
    let r = Val::Float(a.atan());
    Ok(r)
}

fn atan2(_vm: &mut VM, a: Val, b: Val) -> Result<Val, String> {
    let a = a.to_f64()?;
    let b = b.to_f64()?;
    let r = Val::Float(a.atan2(b));
    Ok(r)
}

fn exp_m1(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.to_f64()?;
    let r = Val::Float(a.exp_m1());
    Ok(r)
}

fn ln_1p(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.to_f64()?;
    let r = Val::Float(a.ln_1p());
    Ok(r)
}

fn sinh(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.to_f64()?;
    let r = Val::Float(a.sinh());
    Ok(r)
}

fn cosh(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.to_f64()?;
    let r = Val::Float(a.cosh());
    Ok(r)
}

fn tanh(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.to_f64()?;
    let r = Val::Float(a.tanh());
    Ok(r)
}

fn asinh(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.to_f64()?;
    let r = Val::Float(a.asinh());
    Ok(r)
}
fn acosh(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.to_f64()?;
    let r = Val::Float(a.acosh());
    Ok(r)
}

fn atanh(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.to_f64()?;
    let r = Val::Float(a.atanh());
    Ok(r)
}

fn is_nan(_vm: &mut VM, a: Val) -> Result<Val, String> {
    // TODO: nan?
    let r = match a {
        Val::Float(a) => a.is_nan(),
        _ => false,
    };
    let r = Val::boolean(r);
    Ok(r)
}

fn is_finite(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let r = match a {
        Val::Float(a) => a.is_finite(),
        _ => true,
    };
    let r = Val::boolean(r);
    Ok(r)
}

fn is_infinite(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let r = match a {
        Val::Float(a) => a.is_infinite(),
        _ => false,
    };
    let r = Val::boolean(r);
    Ok(r)
}

fn is_subnormal(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let r = match a {
        Val::Float(a) => a.is_subnormal(),
        _ => false,
    };
    let r = Val::boolean(r);
    Ok(r)
}

fn is_normal(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let r = match a {
        Val::Float(a) => a.is_normal(),
        _ => true,
    };
    let r = Val::boolean(r);
    Ok(r)
}

fn is_sign_positive(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.num()?;
    let r = match a {
        Val::Float(a) => a.is_sign_positive(),
        Val::Int(a) => a.is_positive(),
        _ => panic!(),
    };
    let r = Val::boolean(r);
    Ok(r)
}

fn is_sign_negative(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.num()?;
    let r = match a {
        Val::Float(a) => a.is_sign_negative(),
        Val::Int(a) => a.is_negative(),
        _ => panic!(),
    };
    let r = Val::boolean(r);
    Ok(r)
}

fn recip(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.to_f64()?;
    let r = Val::Float(a.recip());
    Ok(r)
}

fn to_degrees(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.to_f64()?;
    let r = Val::Float(a.to_degrees());
    Ok(r)
}

fn to_radians(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.to_f64()?;
    let r = Val::Float(a.to_radians());
    Ok(r)
}

fn nth_root(_vm: &mut VM, a: Val, b: Val) -> Result<Val, String> {
    let a = a.to_bigint()?;
    let b = b.to_u32()?;
    let r = Val::Int(a.nth_root(b));
    Ok(r)
}

fn trailing_zeros(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = a.to_bigint()?;
    let r = a.trailing_zeros().unwrap_or_default();
    let r = Val::Int(BigInt::from(r));
    Ok(r)
}

fn bit(_vm: &mut VM, a: Val, b: Val) -> Result<Val, String> {
    let a = a.to_bigint()?;
    let b = b.to_u64()?;
    let r = Val::boolean(a.bit(b));
    Ok(r)
}

fn set_bit(_vm: &mut VM, a: Val, bit: Val, value: Val) -> Result<Val, String> {
    let a = a.to_bigint()?;
    let bit = bit.to_u64()?;
    let value = value.truth();

    let mut r = a.clone();
    r.set_bit(bit, value);

    let r = Val::Int(r);
    Ok(r)
}

fn max(_vm: &mut VM, a: Val, b: Val) -> Result<Val, String> {
    let (a, b) = num2_loose(&a, &b);
    let r = match (&a, &b) {
        (Val::Float(a), Val::Float(b)) => Val::Float(a.max(*b)),
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
        (Val::Float(a), Val::Float(b)) => Val::Float(a.min(*b)),
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

fn clamp(_vm: &mut VM, a: Val, lo: Val, hi: Val) -> Result<Val, String> {
    let (a, lo, hi) = num3_loose(&a, &lo, &hi);
    let r = match (&a, &lo, &hi) {
        (Val::Float(a), Val::Float(lo), Val::Float(hi)) => Val::Float(a.clamp(*lo, *hi)),
        _ => {
            let r = if lt_loose(&a, &lo) {
                lo
            } else if lt_loose(&hi, &a) {
                hi
            } else {
                a
            };
            r.clone()
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
    let code_point = n.to_u32()?;
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
    vm.register1("_assert", _assert);
    vm.register2("_bit_and", _bit_and);
    vm.register1("_bit_not", _bit_not);
    vm.register2("_bit_or", _bit_or);
    vm.register2("_bit_xor", _bit_xor);
    vm.register2("_eq", _eq);
    vm.register2("_fdiv", _fdiv);
    vm.register2("_ge", _ge);
    vm.register2("_gt", _gt);
    vm.register2("_idiv", _idiv);
    vm.register2("_le", _le);
    vm.registerv("_list", _list);
    vm.register2("_lt", _lt);
    vm.register2("_mod", _mod);
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
    vm.register2("cmp", cmp);
    vm.register2("copy_sign", copy_sign);
    vm.register1("cos", cos);
    vm.register1("cosh", cosh);
    vm.register2("div_euclid", div_euclid);
    vm.register1("exp", exp);
    vm.register1("exp2", exp2);
    vm.register1("exp_m1", exp_m1);
    vm.register1("float", float);
    vm.register1("floor", floor);
    vm.register1("fract", fract);
    vm.register2("gcd", gcd);
    vm.register2("hypot", hypot);
    vm.register0("input", input);
    vm.register2("instr", instr);
    vm.register1("int", int);
    vm.register2("int_base", int_base);
    vm.register1("is_finite", is_finite);
    vm.register1("is_infinite", is_infinite);
    vm.register1("is_nan", is_nan);
    vm.register1("is_normal", is_normal);
    vm.register1("is_sign_negative", is_sign_negative);
    vm.register1("is_sign_positive", is_sign_positive);
    vm.register1("is_subnormal", is_subnormal);
    vm.register1("lcase", lcase);
    vm.register2("lcm", lcm);
    vm.register1("len", len);
    vm.register1("ln", ln);
    vm.register1("ln_1p", ln_1p);
    vm.register2("log", log);
    vm.register1("log10", log10);
    vm.register1("log2", log2);
    vm.register2("max", max);
    vm.register2("midpoint", midpoint);
    vm.register2("min", min);
    vm.register3("mul_add", mul_add);
    vm.register2("nth_root", nth_root);
    vm.register2("pow_i", pow_i);
    vm.register1("recip", recip);
    vm.register2("rem_euclid", rem_euclid);
    vm.register0("rnd", rnd);
    vm.register1("round", round);
    vm.register1("round_ties_even", round_ties_even);
    vm.register3("set_bit", set_bit);
    vm.register1("signum", signum);
    vm.register1("sin", sin);
    vm.register1("sinh", sinh);
    vm.register1("sqrt", sqrt);
    vm.register1("str", str);
    vm.register2("str_base", str_base);
    vm.register1("tan", tan);
    vm.register1("tanh", tanh);
    vm.register1("to_degrees", to_degrees);
    vm.register1("to_radians", to_radians);
    vm.register2("total_cmp", total_cmp);
    vm.register1("trailing_zeros", trailing_zeros);
    vm.register1("trunc", trunc);
    vm.register1("typeof_val", typeof_val);
    vm.register1("ucase", ucase);
}
