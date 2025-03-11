use crate::vm::*;
use std::collections::HashMap;
use std::fmt;
use std::mem;

#[derive(Clone, PartialEq)]
enum Tok {
    StrLiteral(String),
    Id(String),
    Colon,
    Newline,
    LParen,
    RParen,
    LSquare,
    RSquare,
    Eof,
    Semi,
    Comma,
    Star,
    Plus,
    Minus,
    Slash,
    True,
    False,
    And,
    Or,
    Not,
    Lt,
    Le,
    Gt,
    Ge,
    Eq,
    Ne,
    Shr,
    Shl,
    Print,
    Mod,
    Let,
    If,
}

pub struct ParseError {
    line: usize,
    msg: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Parse error at line {}: {}", self.line, self.msg)
    }
}

struct Parser {
    keywords: HashMap<String, Tok>,
    chars: Vec<char>,
    pos: usize,
    line: usize,
    tok: Tok,
    code: Vec<Inst>,
}

fn is_id_part(c: char) -> bool {
    c.is_alphanumeric() || c == '_' || c == '$' || c == '%'
}

fn report_char(c: char) -> String {
    if c.is_ascii_graphic() {
        c.to_string()
    } else {
        format!("\\u{:x}", c as u32)
    }
}

fn substr(chars: &Vec<char>, i: usize, j: usize) -> String {
    chars.iter().skip(i).take(j - i).collect()
}

impl Parser {
    fn new(chars: Vec<char>) -> Self {
        let mut keywords = HashMap::new();
        keywords.insert("mod".to_string(), Tok::Mod);
        keywords.insert("let".to_string(), Tok::Let);
        keywords.insert("if".to_string(), Tok::If);
        keywords.insert("print".to_string(), Tok::Print);
        keywords.insert("true".to_string(), Tok::True);
        keywords.insert("false".to_string(), Tok::False);
        keywords.insert("and".to_string(), Tok::And);
        keywords.insert("or".to_string(), Tok::Or);
        keywords.insert("not".to_string(), Tok::Not);

        Parser {
            keywords,
            chars,
            pos: 0,
            line: 1,
            tok: Tok::Newline,
            code: Vec::<Inst>::new(),
        }
    }

    fn err<S: AsRef<str>>(&self, msg: S) -> ParseError {
        ParseError {
            line: self.line,
            msg: msg.as_ref().to_string(),
        }
    }

    fn eol(&mut self) {
        let mut i = self.pos;
        while self.chars[i] != '\n' {
            i += 1;
        }
        self.pos = i;
    }

    fn hex_to_char(&self, i: usize, j: usize) -> Result<char, ParseError> {
        let s: String = substr(&self.chars, i, j);
        let n = match u32::from_str_radix(&s, 16) {
            Ok(n) => n,
            Err(e) => return Err(self.err(e.to_string())),
        };
        match char::from_u32(n) {
            Some(c) => Ok(c),
            None => Err(self.err("Not a valid Unicode character")),
        }
    }

    fn lex(&mut self) -> Result<(), ParseError> {
        while self.pos < self.chars.len() {
            let c = self.chars[self.pos];
            match c {
                '"' => {
                    let mut i = self.pos + 1;
                    let mut v = Vec::<char>::new();
                    while self.chars[i] != '"' {
                        let mut c = self.chars[i];
                        i += 1;
                        if c == '\\' {
                            c = self.chars[i];
                            i += 1;
                            c = match c {
                                't' => '\t',
                                'r' => '\r',
                                'n' => '\n',
                                '\'' => '\'',
                                '"' => '"',
                                '0' => '\0',
                                '\\' => '\\',
                                'x' => {
                                    let c = self.hex_to_char(i, i + 2)?;
                                    i += 2;
                                    c
                                }
                                _ => {
                                    return Err(self.err(format!(
                                        "'{}': Unknown escape character",
                                        report_char(c)
                                    )));
                                }
                            }
                        }
                        v.push(c);
                    }
                    self.pos = i + 1;
                    self.tok = Tok::StrLiteral(String::from_iter(v));
                    return Ok(());
                }
                '\'' => {
                    self.eol();
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
                ';' => {
                    self.pos += 1;
                    self.tok = Tok::Semi;
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
                    if self.chars[self.pos] == '=' {
                        self.pos += 1;
                    }
                    self.tok = Tok::Eq;
                    return Ok(());
                }
                '\n' => {
                    self.pos += 1;
                    self.line += 1;
                    self.tok = Tok::Newline;
                    return Ok(());
                }
                ' ' | '\t' | '\r' | '\x0c' => {
                    self.pos += 1;
                    continue;
                }
                '<' => {
                    self.tok = match self.chars[self.pos + 1] {
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
                    self.tok = match self.chars[self.pos + 1] {
                        '=' => {
                            self.pos += 2;
                            Tok::Ne
                        }
                        _ => return Err(self.err("'!': Expected '='")),
                    };
                    return Ok(());
                }
                '>' => {
                    self.tok = match self.chars[self.pos + 1] {
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
                _ => {
                    if c.is_alphabetic() || c == '_' {
                        let mut i = self.pos;
                        loop {
                            i += 1;
                            if !is_id_part(self.chars[i]) {
                                break;
                            }
                        }
                        let s = substr(&self.chars, self.pos, i).to_lowercase();
                        self.pos = i;
                        if s == "rem" {
                            self.eol();
                            continue;
                        }
                        self.tok = match self.keywords.get(&s) {
                            Some(tok) => tok.clone(),
                            None => Tok::Id(s),
                        };
                        return Ok(());
                    }
                    return Err(self.err(format!("'{}': Unknown character", report_char(c))));
                }
            }
        }
        self.tok = Tok::Eof;
        Ok(())
    }

    fn expr(&mut self) -> Result<(), ParseError> {
        match &self.tok {
            Tok::StrLiteral(s) => {
                self.code.push(Inst::Const(Val::string(s)));
                self.lex()?;
            }
            _ => return Err(self.err("Expected expression")),
        }
        Ok(())
    }

    fn stmt(&mut self) -> Result<(), ParseError> {
        match self.tok {
            Tok::Newline => self.lex()?,
            Tok::Print => {
                self.lex()?;
                self.expr()?;
                self.code.push(Inst::Print);
                self.code.push(Inst::Const(Val::string("\n")));
                self.code.push(Inst::Print);
            }
            _ => return Err(self.err("Expected PRINT")),
        }
        Ok(())
    }

    fn parse(&mut self) -> Result<Vec<Inst>, ParseError> {
        if self.chars[0] == '#' && self.chars[1] == '!' {
            self.eol();
            self.lex()?;
        }
        self.lex()?;
        while self.tok != Tok::Eof {
            self.stmt()?;
        }
        Ok(mem::take(&mut self.code))
    }
}

pub fn parse(text: &str) -> Result<Vec<Inst>, ParseError> {
    let mut chars: Vec<char> = text.chars().collect();
    if !text.ends_with('\n') {
        chars.push('\n');
    }
    let mut parser = Parser::new(chars);
    parser.parse()
}
