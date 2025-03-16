use verbena::*;

#[test]
fn expected_name() {
    let text = prep("LET 1=2");
    let r = parse(&text);
    match r {
        Ok(_) => panic!(),
        Err(_) => {}
    }
}

#[test]
fn no_end() {
    let text = prep("LET a=1 LET b=2");
    let r = parse(&text);
    match r {
        Ok(_) => panic!(),
        Err(_) => {}
    }
}
