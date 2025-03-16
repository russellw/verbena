use verbena::*;

#[test]
fn empty() {
    let text = prep("");
    let r = parse(&text);
    assert!(r.is_ok());
}

#[test]
fn space() {
    let text = prep(" ");
    let r = parse(&text);
    assert!(r.is_ok());
}

#[test]
fn tab() {
    let text = prep("\t");
    let r = parse(&text);
    assert!(r.is_ok());
}

#[test]
fn blank_line() {
    let text = prep("\n");
    let r = parse(&text);
    assert!(r.is_ok());
}

#[test]
fn cr() {
    let text = prep("\r");
    let r = parse(&text);
    assert!(r.is_ok());
}
