use verbena::*;

#[test]
fn test_rem() {
    let text = "REM No code here";
    let r = parse(text);
    assert!(r.is_ok());
}

#[test]
fn test_rem_title_case() {
    let text = "Rem Or here";
    let r = parse(text);
    assert!(r.is_ok());
}

#[test]
fn test_single_quote() {
    let text = "' No code here";
    let r = parse(text);
    assert!(r.is_ok());
}

#[test]
fn test_shebang() {
    let text = "#! No code here";
    let r = parse(text);
    assert!(r.is_ok());
}
