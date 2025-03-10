use crate::vm::*;
use std::mem;

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

    fn expr(&mut self) -> Result<(), String> {
        Ok(())
    }

    fn parse(&mut self) -> Result<Vec<Inst>, String> {
        self.expr()?;
        Ok(mem::take(&mut self.code))
    }
}

pub fn parse(text: &str) -> Result<Vec<Inst>, String> {
    let text = if text.ends_with('\n') {
        text.to_string()
    } else {
        let mut r = String::with_capacity(text.len() + 1);
        r.push_str(&text);
        r.push('\n');
        r
    };
    let mut parser = Parser::new(&text);
    parser.parse()
}
