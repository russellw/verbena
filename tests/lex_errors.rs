use verbena::*;

#[test]
fn ats() {
    let text = prep("@@@@@");
    let r = parse(&text);
    match r {
        Ok(_) => panic!(),
        Err(e) => {
            assert_eq!(e.text, text);
        }
    }
}

#[test]
fn test_invalid_escape_sequence() {
    let text = prep("print \"Hello\\z World\"");
    let r = parse(&text);
    match r {
        Ok(_) => panic!("Should fail on invalid escape sequence"),
        Err(e) => {

            // Don't test exact error message
        }
    }
}

#[test]
fn test_invalid_hex_escape() {
    let text = prep("print \"Hello\\xZG World\"");
    let r = parse(&text);
    match r {
        Ok(_) => panic!("Should fail on invalid hex escape"),
        Err(e) => {}
    }
}

#[test]
fn test_invalid_unicode_escape_no_brace() {
    let text = prep("print \"Hello\\u1234 World\"");
    let r = parse(&text);
    match r {
        Ok(_) => panic!("Should fail on invalid unicode escape without braces"),
        Err(e) => {}
    }
}

#[test]
fn test_invalid_unicode_escape_non_hex() {
    let text = prep("print \"Hello\\u{XYZW} World\"");
    let r = parse(&text);
    match r {
        Ok(_) => panic!("Should fail on non-hex characters in unicode escape"),
        Err(e) => {}
    }
}

#[test]
fn test_unterminated_string() {
    let text = prep("print \"Hello World");
    let r = parse(&text);
    match r {
        Ok(_) => panic!("Should fail on unterminated string"),
        Err(e) => {}
    }
}

#[test]
fn test_unclosed_unicode_brace() {
    let text = prep("print \"Hello\\u{1234 World\"");
    let r = parse(&text);
    match r {
        Ok(_) => panic!("Should fail on unclosed unicode escape brace"),
        Err(e) => {}
    }
}

#[test]
fn test_invalid_bang() {
    let text = prep("print !true");
    let r = parse(&text);
    match r {
        Ok(_) => panic!("Should fail on invalid bang usage"),
        Err(e) => {}
    }
}

#[test]
fn test_multiline_error() {
    let text = prep("print \"Hello\"\n@@@");
    let r = parse(&text);
    match r {
        Ok(_) => panic!("Should fail on invalid character"),
        Err(e) => {
            assert_eq!(e.line, 2);
        }
    }
}

#[test]
fn test_invalid_unicode_codepoint() {
    let text = prep("print \"Hello\\u{D800} World\""); // D800 is an invalid codepoint (surrogate)
    let r = parse(&text);
    match r {
        Ok(_) => panic!("Should fail on invalid Unicode codepoint"),
        Err(e) => {}
    }
}

#[test]
fn test_empty_unicode_escape() {
    let text = prep("print \"Hello\\u{} World\"");
    let r = parse(&text);
    match r {
        Ok(_) => panic!("Should fail on empty Unicode escape"),
        Err(e) => {}
    }
}

#[test]
fn test_unicode_escape_too_large() {
    let text = prep("print \"Hello\\u{110000} World\""); // Larger than U+10FFFF
    let r = parse(&text);
    match r {
        Ok(_) => panic!("Should fail on Unicode codepoint too large"),
        Err(e) => {}
    }
}

#[test]
fn test_incomplete_hex_escape() {
    let text = prep("print \"Hello\\x4\""); // Needs two hex digits
    let r = parse(&text);
    match r {
        Ok(_) => panic!("Should fail on incomplete hex escape"),
        Err(e) => {}
    }
}

#[test]
fn test_invalid_token_sequence() {
    let text = prep("print + \"Hello\"");
    let r = parse(&text);
    match r {
        Ok(_) => panic!("Should fail on invalid token sequence"),
        Err(e) => {}
    }
}
