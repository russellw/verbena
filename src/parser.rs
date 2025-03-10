mod vm;
use vm::*;

struct Parser<'a> {
    text: &'a str,
    pos: usize,
    code: Vec<Inst>,
}

impl<'a> Parser<'a> {
    fn new(text: &'a str) -> Self {
        Parser {
            text,
            pos: 0,
            code: Vec::<Inst>::new(),
        }
    }

    fn expr(&mut self) -> Result<(), String> {}

    fn parse(&mut self) -> Result<Vec<Inst>, String> {
        expr();
    }
}

pub fn parse(text: &str) -> Result<Vec<Inst>, String> {
    let mut parser = Parser::new(text);
    parser.parse()
}
