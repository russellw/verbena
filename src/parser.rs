use crate::vm::*;
use std::mem;

enum Tok {
    Colon,
    Newline,
    LParen,
    RParen,
    LSquare,
    RSquare,
    Semi,
    Comma,
    Star,
    Plus,
    Minus,
    Slash,
    Lt,
    Le,
    Gt,
    Ge,
    Eq,
    Print,
    Rem,
}

struct Parser {
    text: Vec<char>,
    pos: usize,
    line: usize,
    tok: Tok,
    code: Vec<Inst>,
}

impl Parser {
    fn new(text: Vec<char>) -> Self {
        Parser {
            text,
            pos: 0,
            line: 1,
            tok: Tok::Newline,
            code: Vec::<Inst>::new(),
        }
    }

    fn err<S: AsRef<str>>(&self, msg: S) -> Result<(), String> {
        Err(format!("{}: {}", self.line, msg.as_ref()).to_string())
    }

    fn lex(&mut self) -> Result<(), String> {
        loop {
            let c = self.text[self.pos];
            match c {
                ':' => {
                    self.pos += 1;
                    self.tok = Tok::Colon;
                    return Ok(());
                }
                _ => {
                    return self.err(format!("'{}': unknown character", c));
                }
            }
        }
    }

    fn expr(&mut self) -> Result<(), String> {
        Ok(())
    }

    fn parse(&mut self) -> Result<Vec<Inst>, String> {
        self.lex()?;
        self.expr()?;
        Ok(mem::take(&mut self.code))
    }
}

pub fn parse(text: &str) -> Result<Vec<Inst>, String> {
    let mut chars: Vec<char> = text.chars().collect();
    if !text.ends_with('\n') {
        chars.push('\n');
    }
    let mut parser = Parser::new(chars);
    parser.parse()
}
