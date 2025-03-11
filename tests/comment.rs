use verbena::*;

#[test]
fn test() {
    let text = "REM No code here";
    let r = parse(text);
    assert!(r.is_ok());
}
