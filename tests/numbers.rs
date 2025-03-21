use verbena::*;

const FILE: &str = "test";

#[test]
fn test_valid_decimal() {
    let text = "print 123";
    let r = parse_str(FILE, &text);
    match r {
        Ok(_) => {}
        Err(_) => panic!("Should succeed on valid decimal"),
    }
}

#[test]
fn test_valid_decimal_with_underscores() {
    let text = "print 1_234_567";
    let r = parse_str(FILE, &text);
    match r {
        Ok(_) => {}
        Err(_) => panic!("Should succeed with underscores in decimal"),
    }
}

#[test]
fn test_valid_decimal_with_fractional() {
    let text = "print 123.456";
    let r = parse_str(FILE, &text);
    match r {
        Ok(_) => {}
        Err(_) => panic!("Should succeed on valid decimal with fraction"),
    }
}

#[test]
fn test_valid_decimal_with_exponent() {
    let text = "print 1.23e2";
    let r = parse_str(FILE, &text);
    match r {
        Ok(_) => {}
        Err(_) => panic!("Should succeed on valid decimal with exponent"),
    }
}

#[test]
fn test_valid_hex() {
    let text = "print 0x1A";
    let r = parse_str(FILE, &text);
    match r {
        Ok(_) => {}
        Err(_) => panic!("Should succeed on valid hex"),
    }
}

#[test]
fn test_valid_binary() {
    let text = "print 0b1010";
    let r = parse_str(FILE, &text);
    match r {
        Ok(_) => {}
        Err(_) => panic!("Should succeed on valid binary"),
    }
}

#[test]
fn test_valid_octal() {
    let text = "print 0o17";
    let r = parse_str(FILE, &text);
    match r {
        Ok(_) => {}
        Err(_) => panic!("Should succeed on valid octal"),
    }
}

#[test]
fn test_hex_too_large_for_u128() {
    // This hex value is greater than u128::MAX (which is 2^128 - 1)
    let text = "print 0x100000000000000000000000000000000";
    let r = parse_str(FILE, &text);
    match r {
        Ok(_) => {}
        Err(_) => {
            panic!("Should not fail on hex number too large for u128");
        }
    }
}

#[test]
fn test_binary_too_large_for_u128() {
    // 129 '1' bits, exceeding u128 range
    let mut binary_string = String::from("0b1");
    binary_string.extend(std::iter::repeat('0').take(128));

    let text = format!("print {}", binary_string);
    let r = parse_str(FILE, &text);
    match r {
        Ok(_) => {}
        Err(_) => {
            panic!("Should not fail on binary number too large for u128");
        }
    }
}

#[test]
fn test_octal_too_large_for_u128() {
    // This octal value is greater than u128::MAX
    let text = "print 0o4000000000000000000000000000000000000000000";
    let r = parse_str(FILE, &text);
    match r {
        Ok(_) => {}
        Err(_) => {
            panic!("Should not fail on octal number too large for u128");
        }
    }
}

#[test]
fn test_invalid_hex_digit() {
    let text = "print 0xG1";
    let r = parse_str(FILE, &text);
    match r {
        Ok(_) => panic!("Should fail on invalid hex digit"),
        Err(_) => {}
    }
}

#[test]
fn test_invalid_binary_digit() {
    let text = "print 0b102";
    let r = parse_str(FILE, &text);
    match r {
        Ok(_) => panic!("Should fail on invalid binary digit"),
        Err(_) => {}
    }
}

#[test]
fn test_invalid_octal_digit() {
    let text = "print 0o18";
    let r = parse_str(FILE, &text);
    match r {
        Ok(_) => panic!("Should fail on invalid octal digit"),
        Err(_) => {}
    }
}

#[test]
fn test_empty_hex() {
    let text = "print 0x";
    let r = parse_str(FILE, &text);
    match r {
        Ok(_) => panic!("Should fail on empty hex literal"),
        Err(_) => {}
    }
}

#[test]
fn test_empty_binary() {
    let text = "print 0b";
    let r = parse_str(FILE, &text);
    match r {
        Ok(_) => panic!("Should fail on empty binary literal"),
        Err(_) => {}
    }
}

#[test]
fn test_empty_octal() {
    let text = "print 0o";
    let r = parse_str(FILE, &text);
    match r {
        Ok(_) => panic!("Should fail on empty octal literal"),
        Err(_) => {}
    }
}

#[test]
fn test_decimal_large() {
    let text = "print 1e1000"; // Very large exponent
    let r = parse_str(FILE, &text);
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
    let r = parse_str(FILE, &text);
    match r {
        Ok(_) => panic!("Should fail on malformed exponent with no digits"),
        Err(_) => {}
    }
}

#[test]
fn test_just_decimal_point() {
    let text = "print .";
    let r = parse_str(FILE, &text);
    match r {
        Ok(_) => panic!("Should fail on just decimal point"),
        Err(_) => {}
    }
}

#[test]
fn test_max_value_hex() {
    // Test the maximum value that u128 can hold
    let text = "print 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF";
    let r = parse_str(FILE, &text);
    match r {
        Ok(_) => {}
        Err(_) => panic!("Should succeed on max u128 value"),
    }
}

#[test]
fn test_max_value_binary() {
    // Create a string of 128 '1's, which is the maximum binary value for u128
    let mut binary_string = String::from("0b");
    binary_string.extend(std::iter::repeat('1').take(128));

    let text = format!("print {}", binary_string);
    let r = parse_str(FILE, &text);
    match r {
        Ok(_) => {}
        Err(_) => panic!("Should succeed on max u128 value in binary"),
    }
}

#[test]
fn test_negative_exponent() {
    let text = "print 1.5e-2";
    let r = parse_str(FILE, &text);
    match r {
        Ok(_) => {}
        Err(_) => panic!("Should succeed on negative exponent"),
    }
}

#[test]
fn test_leading_zeros_hex() {
    let text = "print 0x0000F";
    let r = parse_str(FILE, &text);
    match r {
        Ok(_) => {}
        Err(_) => panic!("Should succeed on hex with leading zeros"),
    }
}

#[test]
fn test_hex_with_underscores() {
    let text = "print 0xA_B_C_D";
    let r = parse_str(FILE, &text);
    match r {
        Ok(_) => {}
        Err(_) => panic!("Should succeed on hex with underscores"),
    }
}
