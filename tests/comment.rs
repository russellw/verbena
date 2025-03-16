use verbena::*;

#[test]
fn test_rem() {
    let text = prep("REM No code here");
    let r = parse(&text);
    assert!(r.is_ok());
}

#[test]
fn test_rem_title_case() {
    let text = prep("Rem Or here");
    let r = parse(&text);
    assert!(r.is_ok());
}

#[test]
fn test_single_quote() {
    let text = prep("' No code here");
    let r = parse(&text);
    assert!(r.is_ok());
}

#[test]
fn test_shebang() {
    let text = prep("#! No code here");
    let r = parse(&text);
    assert!(r.is_ok());
}

#[test]
fn test_mixed_case_rem() {
    let text = prep("ReM This is a mixed case REM comment");
    let r = parse(&text);
    assert!(r.is_ok());
}

#[test]
fn test_rem_after_newline() {
    let text = prep("\nREM Comment after newline");
    let r = parse(&text);
    assert!(r.is_ok());
}

#[test]
fn test_rem_with_keywords() {
    let text = prep("REM This contains keywords like print, let, if, true, false");
    let r = parse(&text);
    assert!(r.is_ok());
}

#[test]
fn test_single_quote_after_whitespace() {
    let text = prep("  ' Comment with leading whitespace");
    let r = parse(&text);
    assert!(r.is_ok());
}

#[test]
fn test_multiple_comments() {
    let text = prep("REM First comment\n' Second comment\nREM Third comment");
    let r = parse(&text);
    assert!(r.is_ok());
}

#[test]
fn test_empty_comment_lines() {
    let text = prep("REM\n'\nREM");
    let r = parse(&text);
    assert!(r.is_ok());
}

#[test]
fn test_comment_with_special_chars() {
    let text = prep("' Comment with special chars: !@#$%^&*()_+{}|:<>?");
    let r = parse(&text);
    assert!(r.is_ok());
}

#[test]
fn test_comment_with_quotes() {
    let text = prep("REM Comment with 'single' and \"double\" quotes");
    let r = parse(&text);
    assert!(r.is_ok());
}

#[test]
fn test_shebang_followed_by_code() {
    let text = prep("#! Shebang line\nprint \"Hello\"");
    let r = parse(&text);
    assert!(r.is_ok());
}

#[test]
fn test_shebang_with_rem() {
    let text = prep("#! Shebang line\nREM Comment after shebang");
    let r = parse(&text);
    assert!(r.is_ok());
}

#[test]
fn test_rem_followed_by_code() {
    let text = prep("REM Comment\nprint \"Hello\"");
    let r = parse(&text);
    assert!(r.is_ok());
}

#[test]
fn test_consecutive_comment_styles() {
    let text = prep("REM First style\n' Second style\n#! Not a valid shebang position\n");
    let r = parse(&text);
    // This should fail because shebang is only valid at the start
    assert!(r.is_err());
}

#[test]
fn test_comment_with_escape_sequences() {
    let text = prep("' Comment with escape \\n \\t \\\"");
    let r = parse(&text);
    assert!(r.is_ok());
}
