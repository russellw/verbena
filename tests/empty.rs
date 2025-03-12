use verbena::*;

#[test]
fn test() {
    let text = "";
    let r = parse(text);
    assert!(r.is_ok());
}
