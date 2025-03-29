use verbena::*;

const FILE: &str = "test";

fn test_str(text: &str) -> Result<Program, CompileError> {
    let ast = parse_str(FILE, &text)?;
    compile(&ast)
}

#[test]
fn test_valid_decimal() {
    let text = "print 123";
    let r = test_str(&text);
    match r {
        Ok(_) => {}
        Err(_) => panic!("Should succeed on valid decimal"),
    }
}

#[test]
fn test_valid_decimal_with_underscores() {
    let text = "print 1_234_567";
    let r = test_str(&text);
    match r {
        Ok(_) => {}
        Err(_) => panic!("Should succeed with underscores in decimal"),
    }
}

#[test]
fn test_valid_decimal_with_fractional() {
    let text = "print 123.456";
    let r = test_str(&text);
    match r {
        Ok(_) => {}
        Err(_) => panic!("Should succeed on valid decimal with fraction"),
    }
}

#[test]
fn test_valid_decimal_with_exponent() {
    let text = "print 1.23e2";
    let r = test_str(&text);
    match r {
        Ok(_) => {}
        Err(_) => panic!("Should succeed on valid decimal with exponent"),
    }
}

#[test]
fn test_valid_hex() {
    let text = "print 0x1A";
    let r = test_str(&text);
    match r {
        Ok(_) => {}
        Err(_) => panic!("Should succeed on valid hex"),
    }
}

#[test]
fn test_valid_binary() {
    let text = "print 0b1010";
    let r = test_str(&text);
    match r {
        Ok(_) => {}
        Err(_) => panic!("Should succeed on valid binary"),
    }
}

#[test]
fn test_valid_octal() {
    let text = "print 0o17";
    let r = test_str(&text);
    match r {
        Ok(_) => {}
        Err(_) => panic!("Should succeed on valid octal"),
    }
}

#[test]
fn test_invalid_hex_digit() {
    let text = "print 0xG1";
    let r = test_str(&text);
    match r {
        Ok(_) => panic!("Should fail on invalid hex digit"),
        Err(_) => {}
    }
}

#[test]
fn test_invalid_binary_digit() {
    let text = "print 0b102";
    let r = test_str(&text);
    match r {
        Ok(_) => panic!("Should fail on invalid binary digit"),
        Err(_) => {}
    }
}

#[test]
fn test_invalid_octal_digit() {
    let text = "print 0o18";
    let r = test_str(&text);
    match r {
        Ok(_) => panic!("Should fail on invalid octal digit"),
        Err(_) => {}
    }
}

#[test]
fn test_empty_hex() {
    let text = "print 0x";
    let r = test_str(&text);
    match r {
        Ok(_) => panic!("Should fail on empty hex literal"),
        Err(_) => {}
    }
}

#[test]
fn test_empty_binary() {
    let text = "print 0b";
    let r = test_str(&text);
    match r {
        Ok(_) => panic!("Should fail on empty binary literal"),
        Err(_) => {}
    }
}

#[test]
fn test_empty_octal() {
    let text = "print 0o";
    let r = test_str(&text);
    match r {
        Ok(_) => panic!("Should fail on empty octal literal"),
        Err(_) => {}
    }
}

#[test]
fn test_malformed_exponent_no_digits() {
    let text = "print 1.5e";
    let r = test_str(&text);
    match r {
        Ok(_) => panic!("Should fail on malformed exponent with no digits"),
        Err(_) => {}
    }
}

#[test]
fn test_just_decimal_point() {
    let text = "print .";
    let r = test_str(&text);
    match r {
        Ok(_) => panic!("Should fail on just decimal point"),
        Err(_) => {}
    }
}

#[test]
fn test_negative_exponent() {
    let text = "print 1.5e-2";
    let r = test_str(&text);
    match r {
        Ok(_) => {}
        Err(_) => panic!("Should succeed on negative exponent"),
    }
}

#[test]
fn test_leading_zeros_hex() {
    let text = "print 0x0000F";
    let r = test_str(&text);
    match r {
        Ok(_) => {}
        Err(_) => panic!("Should succeed on hex with leading zeros"),
    }
}

#[test]
fn test_hex_with_underscores() {
    let text = "print 0xA_B_C_D";
    let r = test_str(&text);
    match r {
        Ok(_) => {}
        Err(_) => panic!("Should succeed on hex with underscores"),
    }
}
