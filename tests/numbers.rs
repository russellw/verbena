use fastnum::D256;
use num_traits::FromPrimitive;
use verbena::*;

#[test]
fn test_valid_decimal() {
    let text = "print 123";
    let r = parse(text);
    match r {
        Ok(code) => {
            assert!(code.len() > 0);
            match &code[0] {
                Inst::Const(Val::Num(n)) => {
                    assert_eq!(*n, D256::from(123));
                }
                _ => panic!("Expected Num constant"),
            }
        }
        Err(_) => panic!("Should succeed on valid decimal"),
    }
}

#[test]
fn test_valid_decimal_with_underscores() {
    let text = "print 1_234_567";
    let r = parse(text);
    match r {
        Ok(code) => match &code[0] {
            Inst::Const(Val::Num(n)) => {
                assert_eq!(*n, D256::from(1234567));
            }
            _ => panic!("Expected Num constant"),
        },
        Err(_) => panic!("Should succeed with underscores in decimal"),
    }
}

#[test]
fn test_valid_decimal_with_fractional() {
    let text = "print 123.456";
    let r = parse(text);
    match r {
        Ok(code) => {
            match &code[0] {
                Inst::Const(Val::Num(n)) => {
                    // Using a string comparison since floating point equality can be tricky
                    assert_eq!(n.to_string(), "123.456");
                }
                _ => panic!("Expected Num constant"),
            }
        }
        Err(_) => panic!("Should succeed on valid decimal with fraction"),
    }
}

#[test]
fn test_valid_decimal_with_exponent() {
    let text = "print 1.23e2";
    let r = parse(text);
    match r {
        Ok(code) => match &code[0] {
            Inst::Const(Val::Num(n)) => {
                assert_eq!(n.to_string(), "123");
            }
            _ => panic!("Expected Num constant"),
        },
        Err(_) => panic!("Should succeed on valid decimal with exponent"),
    }
}

#[test]
fn test_valid_hex() {
    let text = "print 0x1A";
    let r = parse(text);
    match r {
        Ok(code) => match &code[0] {
            Inst::Const(Val::Num(n)) => {
                assert_eq!(*n, D256::from(26));
            }
            _ => panic!("Expected Num constant"),
        },
        Err(_) => panic!("Should succeed on valid hex"),
    }
}

#[test]
fn test_valid_binary() {
    let text = "print 0b1010";
    let r = parse(text);
    match r {
        Ok(code) => match &code[0] {
            Inst::Const(Val::Num(n)) => {
                assert_eq!(*n, D256::from(10));
            }
            _ => panic!("Expected Num constant"),
        },
        Err(_) => panic!("Should succeed on valid binary"),
    }
}

#[test]
fn test_valid_octal() {
    let text = "print 0o17";
    let r = parse(text);
    match r {
        Ok(code) => match &code[0] {
            Inst::Const(Val::Num(n)) => {
                assert_eq!(*n, D256::from(15));
            }
            _ => panic!("Expected Num constant"),
        },
        Err(_) => panic!("Should succeed on valid octal"),
    }
}

// Error cases

#[test]
fn test_hex_too_large_for_u128() {
    // This hex value is greater than u128::MAX (which is 2^128 - 1)
    let text = "print 0x100000000000000000000000000000000";
    let r = parse(text);
    match r {
        Ok(_) => panic!("Should fail on hex number too large for u128"),
        Err(e) => {
            assert_eq!(e.line, 1);
        }
    }
}

#[test]
fn test_binary_too_large_for_u128() {
    // 129 '1' bits, exceeding u128 range
    let mut binary_string = String::from("0b1");
    binary_string.extend(std::iter::repeat('0').take(128));

    let text = format!("print {}", binary_string);
    let r = parse(&text);
    match r {
        Ok(_) => panic!("Should fail on binary number too large for u128"),
        Err(e) => {
            assert_eq!(e.line, 1);
        }
    }
}

#[test]
fn test_octal_too_large_for_u128() {
    // This octal value is greater than u128::MAX
    let text = "print 0o4000000000000000000000000000000000000000000";
    let r = parse(text);
    match r {
        Ok(_) => panic!("Should fail on octal number too large for u128"),
        Err(e) => {
            assert_eq!(e.line, 1);
        }
    }
}

#[test]
fn test_invalid_hex_digit() {
    let text = "print 0xG1";
    let r = parse(text);
    match r {
        Ok(_) => panic!("Should fail on invalid hex digit"),
        Err(e) => {
            assert_eq!(e.line, 1);
            // The error should indicate there's an invalid digit
            assert!(
                e.msg.contains("invalid digit") || e.msg.contains("parse"),
                "Error message should indicate invalid digit: {}",
                e.msg
            );
        }
    }
}

#[test]
fn test_invalid_binary_digit() {
    let text = "print 0b102";
    let r = parse(text);
    match r {
        Ok(_) => panic!("Should fail on invalid binary digit"),
        Err(e) => {
            assert_eq!(e.line, 1);
        }
    }
}

#[test]
fn test_invalid_octal_digit() {
    let text = "print 0o18";
    let r = parse(text);
    match r {
        Ok(_) => panic!("Should fail on invalid octal digit"),
        Err(e) => {
            assert_eq!(e.line, 1);
        }
    }
}

#[test]
fn test_empty_hex() {
    let text = "print 0x";
    let r = parse(text);
    match r {
        Ok(_) => panic!("Should fail on empty hex literal"),
        Err(e) => {
            assert_eq!(e.line, 1);
            // The parser would likely treat this as invalid digit or empty sequence
            assert!(
                e.msg.contains("invalid") || e.msg.contains("empty") || e.msg.contains("Expected"),
                "Error message should indicate invalid or empty sequence: {}",
                e.msg
            );
        }
    }
}

#[test]
fn test_empty_binary() {
    let text = "print 0b";
    let r = parse(text);
    match r {
        Ok(_) => panic!("Should fail on empty binary literal"),
        Err(e) => {
            assert_eq!(e.line, 1);
            assert!(
                e.msg.contains("invalid") || e.msg.contains("empty") || e.msg.contains("Expected"),
                "Error message should indicate invalid or empty sequence: {}",
                e.msg
            );
        }
    }
}

#[test]
fn test_empty_octal() {
    let text = "print 0o";
    let r = parse(text);
    match r {
        Ok(_) => panic!("Should fail on empty octal literal"),
        Err(e) => {
            assert_eq!(e.line, 1);
            assert!(
                e.msg.contains("invalid") || e.msg.contains("empty") || e.msg.contains("Expected"),
                "Error message should indicate invalid or empty sequence: {}",
                e.msg
            );
        }
    }
}

#[test]
fn test_decimal_large() {
    let text = "print 1e1000"; // Very large exponent
    let r = parse(text);
    match r {
        Ok(_) => {}
        Err(_) => {
            panic!();
        }
    }
}

#[test]
fn test_malformed_exponent_no_digits() {
    let text = "print 1.5e";
    let r = parse(text);
    match r {
        Ok(_) => panic!("Should fail on malformed exponent with no digits"),
        Err(e) => {
            assert_eq!(e.line, 1);
        }
    }
}

#[test]
fn test_just_decimal_point() {
    let text = "print .";
    let r = parse(text);
    match r {
        Ok(_) => panic!("Should fail on just decimal point"),
        Err(e) => {
            assert_eq!(e.line, 1);
            // Should indicate invalid token or unexpected character
            assert!(
                e.msg.contains("Unknown")
                    || e.msg.contains("invalid")
                    || e.msg.contains("Expected"),
                "Error message should indicate invalid token: {}",
                e.msg
            );
        }
    }
}

#[test]
fn test_max_value_hex() {
    // Test the maximum value that u128 can hold
    let text = "print 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF";
    let r = parse(text);
    match r {
        Ok(code) => match &code[0] {
            Inst::Const(Val::Num(n)) => {
                assert_eq!(*n, D256::from_u128(u128::MAX).unwrap());
            }
            _ => panic!("Expected Num constant"),
        },
        Err(_) => panic!("Should succeed on max u128 value"),
    }
}

#[test]
fn test_just_above_max_value_hex() {
    // Test one more than the maximum value that u128 can hold
    let text = "print 0x100000000000000000000000000000000";
    let r = parse(text);
    match r {
        Ok(_) => panic!("Should fail on value just above max u128"),
        Err(e) => {
            assert_eq!(e.line, 1);
        }
    }
}

#[test]
fn test_max_value_binary() {
    // Create a string of 128 '1's, which is the maximum binary value for u128
    let mut binary_string = String::from("0b");
    binary_string.extend(std::iter::repeat('1').take(128));

    let text = format!("print {}", binary_string);
    let r = parse(&text);
    match r {
        Ok(code) => match &code[0] {
            Inst::Const(Val::Num(n)) => {
                assert_eq!(*n, D256::from_u128(u128::MAX).unwrap());
            }
            _ => panic!("Expected Num constant"),
        },
        Err(_) => panic!("Should succeed on max u128 value in binary"),
    }
}

#[test]
fn test_just_above_max_value_binary() {
    // Create a string of 129 '1's, which exceeds maximum binary value for u128
    let mut binary_string = String::from("0b");
    binary_string.extend(std::iter::repeat('1').take(129));

    let text = format!("print {}", binary_string);
    let r = parse(&text);
    match r {
        Ok(_) => panic!("Should fail on value just above max u128 in binary"),
        Err(e) => {
            assert_eq!(e.line, 1);
        }
    }
}

#[test]
fn test_negative_exponent() {
    let text = "print 1.5e-2";
    let r = parse(text);
    match r {
        Ok(code) => match &code[0] {
            Inst::Const(Val::Num(n)) => {
                assert_eq!(n.to_string(), "0.015");
            }
            _ => panic!("Expected Num constant"),
        },
        Err(_) => panic!("Should succeed on negative exponent"),
    }
}

#[test]
fn test_leading_zeros_hex() {
    let text = "print 0x0000F";
    let r = parse(text);
    match r {
        Ok(code) => match &code[0] {
            Inst::Const(Val::Num(n)) => {
                assert_eq!(*n, D256::from(15));
            }
            _ => panic!("Expected Num constant"),
        },
        Err(_) => panic!("Should succeed on hex with leading zeros"),
    }
}

#[test]
fn test_hex_with_underscores() {
    let text = "print 0xA_B_C_D";
    let r = parse(text);
    match r {
        Ok(code) => match &code[0] {
            Inst::Const(Val::Num(n)) => {
                assert_eq!(*n, D256::from(0xABCD));
            }
            _ => panic!("Expected Num constant"),
        },
        Err(_) => panic!("Should succeed on hex with underscores"),
    }
}
