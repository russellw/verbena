use verbena::*;

#[test]
fn expected_name() {
    let text = "LET 1=2";
    let r = parse(text);
    match r {
        Ok(_) => panic!(),
        Err(e) => {
            assert_eq!(e.line, 1);
            assert_eq!(e.text, text);
        }
    }
}
