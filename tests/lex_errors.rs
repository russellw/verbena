use verbena::*;

const FILE: &str = "test";

#[test]
fn ats() {
    let text = "@@@@@";
    let r = parse_str(FILE, &text);
    match r {
        Ok(_) => panic!(),
        Err(_) => {}
    }
}

#[test]
fn test_invalid_escape_sequence() {
    let text = "print \"Hello\\z World\"";
    let r = parse_str(FILE, &text);
    match r {
        Ok(_) => panic!("Should fail on invalid escape sequence"),
        Err(_) => {}
    }
}

#[test]
fn test_invalid_hex_escape() {
    let text = "print \"Hello\\xZG World\"";
    let r = parse_str(FILE, &text);
    match r {
        Ok(_) => panic!("Should fail on invalid hex escape"),
        Err(_) => {}
    }
}

#[test]
fn test_invalid_unicode_escape_no_brace() {
    let text = "print \"Hello\\u1234 World\"";
    let r = parse_str(FILE, &text);
    match r {
        Ok(_) => panic!("Should fail on invalid unicode escape without braces"),
        Err(_) => {}
    }
}

#[test]
fn test_invalid_unicode_escape_non_hex() {
    let text = "print \"Hello\\u{XYZW} World\"";
    let r = parse_str(FILE, &text);
    match r {
        Ok(_) => panic!("Should fail on non-hex characters in unicode escape"),
        Err(_) => {}
    }
}

#[test]
fn test_unterminated_string() {
    let text = "print \"Hello World";
    let r = parse_str(FILE, &text);
    match r {
        Ok(_) => panic!("Should fail on unterminated string"),
        Err(_) => {}
    }
}

#[test]
fn test_unclosed_unicode_brace() {
    let text = "print \"Hello\\u{1234 World\"";
    let r = parse_str(FILE, &text);
    match r {
        Ok(_) => panic!("Should fail on unclosed unicode escape brace"),
        Err(_) => {}
    }
}

#[test]
fn test_multiline_error() {
    let text = "print \"Hello\"\n@@@";
    let r = parse_str(FILE, &text);
    match r {
        Ok(_) => panic!("Should fail on invalid character"),
        Err(_) => {}
    }
}

#[test]
fn test_invalid_unicode_codepoint() {
    let text = "print \"Hello\\u{D800} World\""; // D800 is an invalid codepoint (surrogate)
    let r = parse_str(FILE, &text);
    match r {
        Ok(_) => panic!("Should fail on invalid Unicode codepoint"),
        Err(_) => {}
    }
}

#[test]
fn test_empty_unicode_escape() {
    let text = "print \"Hello\\u{} World\"";
    let r = parse_str(FILE, &text);
    match r {
        Ok(_) => panic!("Should fail on empty Unicode escape"),
        Err(_) => {}
    }
}

#[test]
fn test_unicode_escape_too_large() {
    let text = "print \"Hello\\u{110000} World\""; // Larger than U+10FFFF
    let r = parse_str(FILE, &text);
    match r {
        Ok(_) => panic!("Should fail on Unicode codepoint too large"),
        Err(_) => {}
    }
}

#[test]
fn test_incomplete_hex_escape() {
    let text = "print \"Hello\\x4\""; // Needs two hex digits
    let r = parse_str(FILE, &text);
    match r {
        Ok(_) => panic!("Should fail on incomplete hex escape"),
        Err(_) => {}
    }
}

#[test]
fn test_invalid_token_sequence() {
    let text = "print + \"Hello\"";
    let r = parse_str(FILE, &text);
    match r {
        Ok(_) => panic!("Should fail on invalid token sequence"),
        Err(_) => {}
    }
}
