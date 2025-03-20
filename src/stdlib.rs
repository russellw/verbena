use crate::val::*;
use crate::vm::*;

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

fn print(_vm: &mut VM, a: Val) -> Result<Val, String> {
    print!("{}", a);
    Ok(Val::Int(BigInt::from(0))) // Return 0 as a success indicator
}

fn typeof_val(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let r = match a {
        Val::Int(_) => "int",
        Val::Float(_) => "float",
        Val::Str(_) => "str",
        Val::List(_) => "list",
        Val::Func(_) => "fn",
    };
    Ok(Val::string(r))
}

fn add(_vm: &mut VM, a: Val, b: Val) -> Result<Val, String> {
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

fn eq(_vm: &mut VM, a: Val, b: Val) -> Result<Val, String> {
    let r = to_int_val(eq_impl(&a, &b));
    Ok(r)
}

fn ne(_vm: &mut VM, a: Val, b: Val) -> Result<Val, String> {
    let r = to_int_val(!eq_impl(&a, &b));
    Ok(r)
}

fn lt(_vm: &mut VM, a: Val, b: Val) -> Result<Val, String> {
    let r = to_int_val(lt_impl(&a, &b));
    Ok(r)
}

fn gt(_vm: &mut VM, a: Val, b: Val) -> Result<Val, String> {
    let r = to_int_val(lt_impl(&b, &a));
    Ok(r)
}

fn le(_vm: &mut VM, a: Val, b: Val) -> Result<Val, String> {
    let r = to_int_val(le_impl(&a, &b));
    Ok(r)
}

fn ge(_vm: &mut VM, a: Val, b: Val) -> Result<Val, String> {
    let r = to_int_val(le_impl(&b, &a));
    Ok(r)
}

fn sub(_vm: &mut VM, a: Val, b: Val) -> Result<Val, String> {
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

fn neg(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let r = match &a {
        Val::Int(a) => Val::Int(-a),
        Val::Float(a) => Val::Float(-a),
        _ => {
            return Err("Expected number".to_string());
        }
    };
    Ok(r)
}

fn f_div(_vm: &mut VM, a: Val, b: Val) -> Result<Val, String> {
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

fn pow(_vm: &mut VM, a: Val, b: Val) -> Result<Val, String> {
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

fn bit_and(_vm: &mut VM, a: Val, b: Val) -> Result<Val, String> {
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

fn bit_or(_vm: &mut VM, a: Val, b: Val) -> Result<Val, String> {
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

fn bit_xor(_vm: &mut VM, a: Val, b: Val) -> Result<Val, String> {
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

fn shl(_vm: &mut VM, a: Val, b: Val) -> Result<Val, String> {
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

fn shr(_vm: &mut VM, a: Val, b: Val) -> Result<Val, String> {
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

fn i_div(_vm: &mut VM, a: Val, b: Val) -> Result<Val, String> {
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

fn bit_not(_vm: &mut VM, a: Val) -> Result<Val, String> {
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

fn not(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let r = to_int_val(!a.truth());
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

fn mul(_vm: &mut VM, a: Val, b: Val) -> Result<Val, String> {
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
    let r = to_int_val(a.is_nan());
    Ok(r)
}

fn is_finite(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = match a.to_f64() {
        Some(a) => a,
        None => return Err("Expected number".to_string()),
    };
    let r = to_int_val(a.is_finite());
    Ok(r)
}

fn is_infinite(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = match a.to_f64() {
        Some(a) => a,
        None => return Err("Expected number".to_string()),
    };
    let r = to_int_val(a.is_infinite());
    Ok(r)
}

fn is_subnormal(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = match a.to_f64() {
        Some(a) => a,
        None => return Err("Expected number".to_string()),
    };
    let r = to_int_val(a.is_subnormal());
    Ok(r)
}

fn is_normal(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = match a.to_f64() {
        Some(a) => a,
        None => return Err("Expected number".to_string()),
    };
    let r = to_int_val(a.is_normal());
    Ok(r)
}

fn is_sign_positive(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = match a.to_f64() {
        Some(a) => a,
        None => return Err("Expected number".to_string()),
    };
    let r = to_int_val(a.is_sign_positive());
    Ok(r)
}

fn is_sign_negative(_vm: &mut VM, a: Val) -> Result<Val, String> {
    let a = match a.to_f64() {
        Some(a) => a,
        None => return Err("Expected number".to_string()),
    };
    let r = to_int_val(a.is_sign_negative());
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
    let r = to_int_val(a.bit(b));
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
            if lt_impl(&b, &a) {
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
            if lt_impl(&a, &b) {
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

// Helper functions
fn to_int_val(b: bool) -> Val {
    Val::Int(BigInt::from(if b { 1 } else { 0 }))
}

fn eq_impl(a: &Val, b: &Val) -> bool {
    match (a, b) {
        (Val::Int(a), Val::Int(b)) => a == b,
        (Val::Float(a), Val::Float(b)) => a == b,
        (Val::Str(a), Val::Str(b)) => a == b,
        (Val::List(a), Val::List(b)) => {
            let a = a.borrow();
            let b = b.borrow();
            a.v == b.v
        }
        _ => false,
    }
}

fn lt_impl(a: &Val, b: &Val) -> bool {
    match (a, b) {
        (Val::Int(a), Val::Int(b)) => a < b,
        (Val::Float(a), Val::Float(b)) => a < b,
        (Val::Str(a), Val::Str(b)) => a < b,
        _ => false,
    }
}

fn le_impl(a: &Val, b: &Val) -> bool {
    match (a, b) {
        (Val::Int(a), Val::Int(b)) => a <= b,
        (Val::Float(a), Val::Float(b)) => a <= b,
        (Val::Str(a), Val::Str(b)) => a <= b,
        _ => false,
    }
}

// Register all functions to the VM
fn register(vm: &mut VM) {
    add1(vm, "sqrt", sqrt);
    add1(vm, "to_float", to_float);
    add1(vm, "to_int", to_int);
    add1(vm, "to_str", to_str);
    add1(vm, "print", print);
    add1(vm, "typeof", typeof_val);
    add2(vm, "add", add);
    add2(vm, "eq", eq);
    add2(vm, "ne", ne);
    add2(vm, "lt", lt);
    add2(vm, "gt", gt);
    add2(vm, "le", le);
    add2(vm, "ge", ge);
    add2(vm, "sub", sub);
    add1(vm, "neg", neg);
    add2(vm, "f_div", f_div);
    add2(vm, "total_cmp", total_cmp);
    add2(vm, "copy_sign", copy_sign);
    add2(vm, "midpoint", midpoint);
    add2(vm, "pow", pow);
    add2(vm, "bit_and", bit_and);
    add2(vm, "bit_or", bit_or);
    add2(vm, "bit_xor", bit_xor);
    add2(vm, "gcd", gcd);
    add2(vm, "lcm", lcm);
    add2(vm, "shl", shl);
    add2(vm, "str_base", str_base);
    add2(vm, "val_base", val_base);
    add2(vm, "shr", shr);
    add2(vm, "i_div", i_div);
    add2(vm, "mod", mod_op);
    add1(vm, "bit_not", bit_not);
    add1(vm, "signum", signum);
    add1(vm, "abs", abs);
    add1(vm, "cbrt", cbrt);
    add1(vm, "not", not);
    addv(vm, "list", make_list);
    add2(vm, "subscript", subscript);
    add2(vm, "mul", mul);
    add1(vm, "exit", exit);
    add1(vm, "rnd", rnd);
    add1(vm, "floor", floor);
    add1(vm, "ceil", ceil);
    add1(vm, "round", round);
    add1(vm, "round_ties_even", round_ties_even);
    add1(vm, "trunc", trunc);
    add1(vm, "fract", fract);
    add3(vm, "mul_add", mul_add);
    add2(vm, "div_euclid", div_euclid);
    add2(vm, "rem_euclid", rem_euclid);
    add2(vm, "pow_i", pow_i);
    add1(vm, "exp", exp);
    add1(vm, "exp2", exp2);
    add1(vm, "ln", ln);
    add2(vm, "log", log);
    add1(vm, "log2", log2);
    add1(vm, "log10", log10);
    add2(vm, "hypot", hypot);
    add1(vm, "sin", sin);
    add1(vm, "cos", cos);
    add1(vm, "tan", tan);
    add1(vm, "asin", asin);
    add1(vm, "acos", acos);
    add1(vm, "atan", atan);
    add2(vm, "atan2", atan2);
    add1(vm, "exp_m1", exp_m1);
    add1(vm, "ln_1p", ln_1p);
    add1(vm, "sinh", sinh);
    add1(vm, "cosh", cosh);
    add1(vm, "tanh", tanh);
    add1(vm, "asinh", asinh);
    add1(vm, "acosh", acosh);
    add1(vm, "atanh", atanh);
    add1(vm, "is_nan", is_nan);
    add1(vm, "is_finite", is_finite);
    add1(vm, "is_infinite", is_infinite);
    add1(vm, "is_subnormal", is_subnormal);
    add1(vm, "is_normal", is_normal);
    add1(vm, "is_sign_positive", is_sign_positive);
    add1(vm, "is_sign_negative", is_sign_negative);
    add1(vm, "recip", recip);
    add1(vm, "to_degrees", to_degrees);
    add1(vm, "to_radians", to_radians);
    add2(vm, "nth_root", nth_root);
    add1(vm, "trailing_zeros", trailing_zeros);
    add2(vm, "bit", bit);
    add3(vm, "set_bit", set_bit);
    add2(vm, "max", max);
    add2(vm, "min", min);
    add3(vm, "clamp", clamp);
    add1(vm, "len", len);
    add2(vm, "left", left);
    add2(vm, "right", right);
    add3(vm, "mid", mid);
    add1(vm, "asc", asc);
    add1(vm, "chr", chr);
    add2(vm, "instr", instr);
    add1(vm, "ucase", ucase);
    add1(vm, "lcase", lcase);
}

// Helper functions for registering functions with different number of arguments
fn add1(vm: &mut VM, name: &str, f: fn(&mut VM, Val) -> Result<Val, String>) {
    vm.vars.insert(name.to_string(), Val::func(f));
}

fn add2(vm: &mut VM, name: &str, f: fn(&mut VM, Val, Val) -> Result<Val, String>) {
    vm.vars.insert(name.to_string(), Val::func2(f));
}

fn add3(vm: &mut VM, name: &str, f: fn(&mut VM, Val, Val, Val) -> Result<Val, String>) {
    vm.vars.insert(name.to_string(), Val::func3(f));
}

fn addv(vm: &mut VM, name: &str, f: fn(&mut VM, Vec<Val>) -> Result<Val, String>) {
    vm.vars.insert(name.to_string(), Val::funcv(f));
}
