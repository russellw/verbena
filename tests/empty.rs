use verbena::*;

const FILE: &str = "test";

#[test]
fn empty() {
    let text = "";
    let r = parse(FILE, &text);
    assert!(r.is_ok());
}

#[test]
fn space() {
    let text = " ";
    let r = parse(FILE, &text);
    assert!(r.is_ok());
}

#[test]
fn tab() {
    let text = "\t";
    let r = parse(FILE, &text);
    assert!(r.is_ok());
}

#[test]
fn blank_line() {
    let text = "\n";
    let r = parse(FILE, &text);
    assert!(r.is_ok());
}

#[test]
fn cr() {
    let text = "\r";
    let r = parse(FILE, &text);
    assert!(r.is_ok());
}
