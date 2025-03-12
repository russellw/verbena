use verbena::*;

#[test]
fn ats() {
    let text = "@@@@@";
    let r = parse(text);
    match r {
        Ok(_) => panic!(),
        Err(e) => {
            assert_eq!(e.line, 1);
            assert_eq!(e.text, text);
        }
    }
}
