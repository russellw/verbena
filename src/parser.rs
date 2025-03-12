use crate::vm::*;
use fastnum::D256;
use num_traits::FromPrimitive;
use std::collections::HashMap;
use std::mem;

#[derive(Clone, PartialEq)]
enum Tok {
    Num(D256),
    Str(String),
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

// The operator precedence parser uses a table of these
struct Op {
    prec: u8,
    left: u8,
}

#[derive(Debug)]
pub struct ParseError {
    pub line: usize,
    pub text: String,
    pub caret: usize,
    pub msg: String,
}

struct Parser {
    // There is a compile-time perfect hash package
    // but there are benchmarks showing HashMap to be faster
    keywords: HashMap<String, Tok>,

    ops: HashMap<Tok, Op>,

    // Decode the entire input text upfront
    // to make sure there are no situations in which decoding work is repeated
    chars: Vec<char>,

    // This is where the caret will point to in case of error
    // Most of the time, it points to the start of current token
    caret: usize,

    // Current position in the input text
    // Mostly tracked and used by the tokenizer
    // Most of the time, it points just after the current token
    pos: usize,

    // Current line number in the input text
    // This is tracked as we go along
    // rather than calculated on the spot in the event of error
    // because it will eventually be desirable to also track it at run time
    line: usize,

    tok: Tok,
    code: Vec<Inst>,
}

fn current_line(chars: &Vec<char>, caret: usize) -> (usize, usize) {
    assert!(caret <= chars.len());

    let mut i = caret;
    while 0 < i && chars[i - 1] != '\n' {
        i -= 1;
    }

    let mut j = caret;
    while j < chars.len() && chars[j] != '\n' {
        j += 1;
    }

    (i, j)
}

fn is_id_part(c: char) -> bool {
    c.is_alphanumeric() || c == '_' || c == '$' || c == '%'
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

        let mut ops = HashMap::new();

        Parser {
            keywords,
            ops,
            chars,
            caret: 0,
            pos: 0,
            line: 1,
            tok: Tok::Newline,
            code: Vec::<Inst>::new(),
        }
    }

    fn err<S: AsRef<str>>(&self, msg: S) -> ParseError {
        let (i, j) = current_line(&self.chars, self.caret);
        ParseError {
            line: self.line,
            text: substr(&self.chars, i, j),
            caret: self.caret - i,
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

    fn hex_to_char(&mut self, i: usize, j: usize) -> Result<char, ParseError> {
        self.caret = i;
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
            self.caret = self.pos;
            let c = self.chars[self.pos];
            match c {
                '"' => {
                    let mut i = self.pos + 1;
                    let mut v = Vec::<char>::new();
                    while self.chars[i] != '"' {
                        let mut c = self.chars[i];
                        i += 1;
                        match c {
                            '\n' => {
                                self.caret = i - 1;
                                return Err(self.err("Unterminated string"));
                            }
                            '\\' => {
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
                                    'u' => {
                                        if self.chars[i] != '{' {
                                            self.caret = i;
                                            return Err(self.err("Expected '{'"));
                                        }
                                        i += 1;

                                        let mut j = i;
                                        while self.chars[j].is_digit(16) {
                                            j += 1;
                                        }
                                        let c = self.hex_to_char(i, j)?;
                                        i = j;

                                        if self.chars[i] != '}' {
                                            self.caret = i;
                                            return Err(self.err("Expected '}'"));
                                        }
                                        i += 1;

                                        c
                                    }
                                    _ => {
                                        self.caret = i - 1;
                                        return Err(self.err("Unknown escape character"));
                                    }
                                }
                            }
                            _ => {}
                        }
                        v.push(c);
                    }
                    self.pos = i + 1;
                    self.tok = Tok::Str(String::from_iter(v));
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
                        _ => {
                            self.caret = self.pos + 1;
                            return Err(self.err("Expected '='"));
                        }
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
                    if c.is_digit(10) {
                        let mut i = self.pos;
                        let mut v = Vec::<char>::new();

                        // Possible alternative radix
                        if c == '0' {
                            match self.chars[i + 1] {
                                'x' | 'X' => {
                                    i += 2;
                                    while self.chars[i].is_digit(16) || self.chars[i] == '_' {
                                        if self.chars[i] != '_' {
                                            v.push(self.chars[i])
                                        }
                                        i += 1;
                                    }
                                    self.pos = i;
                                    let s: String = v.into_iter().collect();
                                    let n = match u128::from_str_radix(&s, 16) {
                                        Ok(n) => n,
                                        Err(e) => return Err(self.err(e.to_string())),
                                    };
                                    // TODO: NO_TRAPS?
                                    self.tok = Tok::Num(D256::from_u128(n).unwrap());
                                    return Ok(());
                                }
                                'b' | 'B' => {
                                    i += 2;
                                    while self.chars[i].is_digit(2) || self.chars[i] == '_' {
                                        if self.chars[i] != '_' {
                                            v.push(self.chars[i])
                                        }
                                        i += 1;
                                    }
                                    self.pos = i;
                                    let s: String = v.into_iter().collect();
                                    let n = match u128::from_str_radix(&s, 2) {
                                        Ok(n) => n,
                                        Err(e) => return Err(self.err(e.to_string())),
                                    };
                                    self.tok = Tok::Num(D256::from_u128(n).unwrap());
                                    return Ok(());
                                }
                                'o' | 'O' => {
                                    i += 2;
                                    while self.chars[i].is_digit(8) || self.chars[i] == '_' {
                                        if self.chars[i] != '_' {
                                            v.push(self.chars[i])
                                        }
                                        i += 1;
                                    }
                                    self.pos = i;
                                    let s: String = v.into_iter().collect();
                                    let n = match u128::from_str_radix(&s, 8) {
                                        Ok(n) => n,
                                        Err(e) => return Err(self.err(e.to_string())),
                                    };
                                    self.tok = Tok::Num(D256::from_u128(n).unwrap());
                                    return Ok(());
                                }
                                _ => {}
                            }
                        }

                        // Decimal, integer part
                        loop {
                            let c = self.chars[i];
                            if c != '_' {
                                v.push(c);
                            }
                            i += 1;
                            if !(self.chars[i].is_digit(10) || self.chars[i] == '_') {
                                break;
                            }
                        }

                        // Decimal point
                        if self.chars[i] == '.' {
                            loop {
                                let c = self.chars[i];
                                if c != '_' {
                                    v.push(c);
                                }
                                i += 1;
                                if !(self.chars[i].is_digit(10) || self.chars[i] == '_') {
                                    break;
                                }
                            }
                        }

                        // Exponent
                        match self.chars[i] {
                            'e' | 'E' => {
                                v.push('e');
                                i += 1;
                                match self.chars[i] {
                                    '+' | '-' => {
                                        v.push(self.chars[i]);
                                        i += 1;
                                    }
                                    _ => {}
                                }
                                while self.chars[i].is_digit(10) || self.chars[i] == '_' {
                                    if self.chars[i] != '_' {
                                        v.push(self.chars[i])
                                    }
                                    i += 1;
                                }
                            }
                            _ => {}
                        }

                        self.pos = i;

                        // Convert
                        let s: String = v.into_iter().collect();
                        let a = match D256::from_str(&s, NO_TRAPS) {
                            Ok(a) => a,
                            Err(e) => return Err(self.err(e.to_string())),
                        };
                        self.tok = Tok::Num(a);
                        return Ok(());
                    }
                    return Err(self.err("Unknown character"));
                }
            }
        }
        self.tok = Tok::Eof;
        Ok(())
    }

    fn require(&mut self, tok: Tok, s: &str) -> Result<(), ParseError> {
        if self.tok != tok {
            return Err(self.err(format!("Expected {}", s)));
        }
        self.lex()?;
        Ok(())
    }

    fn expr(&mut self) -> Result<(), ParseError> {
        match &self.tok {
            Tok::Str(s) => {
                self.code.push(Inst::Const(Val::string(s)));
                self.lex()?;
            }
            Tok::Num(a) => {
                self.code.push(Inst::Const(Val::Num(*a)));
                self.lex()?;
            }
            Tok::False => {
                self.code.push(Inst::Const(ZERO));
                self.lex()?;
            }
            Tok::LParen => {
                self.lex()?;
                self.expr()?;
                self.require(Tok::RParen, "')'")?;
            }
            Tok::True => {
                self.code.push(Inst::Const(ONE));
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
