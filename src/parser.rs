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
    Ne,
    Shr,
    Shl,
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

fn report_char(c: char) -> String {
    if c.is_ascii_graphic() {
        c.to_string()
    } else {
        format!("U{:X}", c as u32)
    }
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
                '\'' => {
                    loop {
                        self.pos += 1;
                        if self.text[self.pos] == '\n' {
                            break;
                        }
                    }
                    continue;
                }
                ':' => {
                    self.pos += 1;
                    self.tok = Tok::Colon;
                    return Ok(());
                }
                ',' => {
                    self.pos += 1;
                    self.tok = Tok::Comma;
                    return Ok(());
                }
                '+' => {
                    self.pos += 1;
                    self.tok = Tok::Plus;
                    return Ok(());
                }
                '-' => {
                    self.pos += 1;
                    self.tok = Tok::Minus;
                    return Ok(());
                }
                '*' => {
                    self.pos += 1;
                    self.tok = Tok::Star;
                    return Ok(());
                }
                '/' => {
                    self.pos += 1;
                    self.tok = Tok::Slash;
                    return Ok(());
                }
                '(' => {
                    self.pos += 1;
                    self.tok = Tok::LParen;
                    return Ok(());
                }
                ')' => {
                    self.pos += 1;
                    self.tok = Tok::RParen;
                    return Ok(());
                }
                '[' => {
                    self.pos += 1;
                    self.tok = Tok::LSquare;
                    return Ok(());
                }
                ']' => {
                    self.pos += 1;
                    self.tok = Tok::RSquare;
                    return Ok(());
                }
                '=' => {
                    self.pos += 1;
                    self.tok = Tok::Eq;
                    return Ok(());
                }
                '\n' => {
                    self.pos += 1;
                    self.line += 1;
                    self.tok = Tok::Newline;
                    return Ok(());
                }
                ' ' | '\t' | '\r' | '\x0C' => {
                    self.pos += 1;
                    continue;
                }
                '<' => {
                    self.tok = match self.text[self.pos + 1] {
                        '=' => {
                            self.pos += 2;
                            Tok::Le
                        }
                        '>' => {
                            self.pos += 2;
                            Tok::Ne
                        }
                        '<' => {
                            self.pos += 2;
                            Tok::Shl
                        }
                        _ => {
                            self.pos += 1;
                            Tok::Lt
                        }
                    };
                    return Ok(());
                }
                '!' => {
                    self.tok = match self.text[self.pos + 1] {
                        '=' => {
                            self.pos += 2;
                            Tok::Ne
                        }
                        _ => return self.err("'!': expected '='"),
                    };
                    return Ok(());
                }
                '>' => {
                    self.tok = match self.text[self.pos + 1] {
                        '=' => {
                            self.pos += 2;
                            Tok::Ge
                        }
                        '>' => {
                            self.pos += 2;
                            Tok::Shr
                        }
                        _ => {
                            self.pos += 1;
                            Tok::Gt
                        }
                    };
                    return Ok(());
                }
                _ => return self.err(format!("'{}': unknown character", report_char(c))),
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
