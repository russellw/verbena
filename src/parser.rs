use crate::ast::*;
use crate::compile_error::*;
use crate::error_context::*;
use crate::program::*;
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Cursor};
use std::mem;
use std::rc::Rc;

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
enum Tok {
    Dowhile,
    While,
    Func,
    PrefixedInt(String),
    Num(String),
    Str(String),
    Id(String),
    Colon,
    Newline,
    LParen,
    RParen,
    Assert,
    Dot,
    LSquare,
    RSquare,
    Eof,
    For,
    Return,
    Goto,
    Comma,
    Else,
    End,
    MulAssign,
    Mul,
    PowAssign,
    Pow,
    AddAssign,
    RBrace,
    LBrace,
    Add,
    BitNot,
    BitAndAssign,
    BitAnd,
    BitOrAssign,
    BitOr,
    BitXorAssign,
    BitXor,
    SubAssign,
    Sub,
    ModAssign,
    Mod,
    Assign,
    DivAssign,
    Div,
    And,
    Or,
    Not,
    Lt,
    Le,
    Gt,
    Ge,
    Eq,
    Ne,
    ShrAssign,
    Shr,
    LShrAssign,
    LShr,
    IDivAssign,
    True,
    False,
    Null,
    IDiv,
    ShlAssign,
    Shl,
    Print,
    Prin,
    If,
    Nan,
    Inf,
    Pi,
}

// The operator precedence parser uses a table of these
#[derive(Clone)]
struct Op {
    prec: u8,
    left: u8,
    inst: Inst,
    assign: bool,
}

struct Parser<R: BufRead> {
    // There is a compile-time perfect hash package
    // but there are benchmarks showing HashMap to be faster
    keywords: HashMap<String, Tok>,

    // Table of infix operators
    ops: HashMap<Tok, Op>,

    // Name of the input file
    // or suitable descriptor, if the input did not come from a file
    file: Rc<String>,

    // Input reader
    reader: R,

    // Current line buffer
    // TODO: does this need to be by char?
    buf: Vec<char>,

    // Line number tracker for error reporting
    line: usize,

    // Token position in the current line
    start: usize,

    // Current position in the current line
    pos: usize,

    // Current token
    tok: Tok,
}

fn is_id_part(c: char) -> bool {
    c.is_alphanumeric() || c == '_' || c == '$' || c == '?'
}

fn substr(buf: &[char], i: usize, j: usize) -> String {
    buf.iter().skip(i).take(j - i).collect()
}

fn parse_prefixed_int(input: String) -> Result<u64, String> {
    let (base, digits) = if input.starts_with("0x") || input.starts_with("0X") {
        (16, &input[2..])
    } else if input.starts_with("0b") || input.starts_with("0B") {
        (2, &input[2..])
    } else if input.starts_with("0o") || input.starts_with("0O") {
        (8, &input[2..])
    } else {
        panic!()
    };

    match u64::from_str_radix(digits, base) {
        Ok(value) => Ok(value),
        Err(e) => Err(format!(
            "Failed to parse '{}' as base {}: {}",
            digits, base, e
        )),
    }
}

impl<R: BufRead> Parser<R> {
    fn new(file: &str, reader: R) -> Self {
        // Keywords
        let mut keywords = HashMap::new();
        keywords.insert("assert".to_string(), Tok::Assert);
        keywords.insert("if".to_string(), Tok::If);
        keywords.insert("prin".to_string(), Tok::Prin);
        keywords.insert("print".to_string(), Tok::Print);
        keywords.insert("else".to_string(), Tok::Else);
        keywords.insert("fn".to_string(), Tok::Func);
        keywords.insert("end".to_string(), Tok::End);
        keywords.insert("return".to_string(), Tok::Return);
        keywords.insert("goto".to_string(), Tok::Goto);
        keywords.insert("for".to_string(), Tok::For);
        keywords.insert("while".to_string(), Tok::While);
        keywords.insert("dowhile".to_string(), Tok::Dowhile);
        keywords.insert("true".to_string(), Tok::True);
        keywords.insert("false".to_string(), Tok::False);
        keywords.insert("null".to_string(), Tok::Null);
        keywords.insert("nan".to_string(), Tok::Nan);
        keywords.insert("inf".to_string(), Tok::Inf);
        keywords.insert("pi".to_string(), Tok::Pi);

        // Infix operators
        let mut ops = HashMap::new();
        let mut op = |tok: Tok, prec: u8, left: u8, inst: Inst, assign: bool| {
            ops.insert(
                tok,
                Op {
                    prec,
                    left,
                    inst,
                    assign,
                },
            );
        };

        let mut prec = 99u8;
        op(Tok::Pow, prec, 0, Inst::Pow, false);

        prec -= 1;
        op(Tok::Mul, prec, 1, Inst::Mul, false);
        op(Tok::Div, prec, 1, Inst::Div, false);
        op(Tok::IDiv, prec, 1, Inst::IDiv, false);
        op(Tok::Mod, prec, 1, Inst::Mod, false);

        prec -= 1;
        op(Tok::Add, prec, 1, Inst::Add, false);
        op(Tok::Sub, prec, 1, Inst::Sub, false);

        prec -= 1;
        op(Tok::Shl, prec, 1, Inst::Shl, false);
        op(Tok::Shr, prec, 1, Inst::Shr, false);
        op(Tok::LShr, prec, 1, Inst::LShr, false);

        prec -= 1;
        op(Tok::BitAnd, prec, 1, Inst::BitAnd, false);

        prec -= 1;
        op(Tok::BitXor, prec, 1, Inst::BitXor, false);

        prec -= 1;
        op(Tok::BitOr, prec, 1, Inst::BitOr, false);

        prec -= 1;
        op(Tok::Eq, prec, 1, Inst::Eq, false);
        op(Tok::Ne, prec, 1, Inst::Ne, false);
        op(Tok::Lt, prec, 1, Inst::Lt, false);
        op(Tok::Le, prec, 1, Inst::Le, false);
        op(Tok::Gt, prec, 1, Inst::Gt, false);
        op(Tok::Ge, prec, 1, Inst::Ge, false);

        prec -= 1;
        op(Tok::And, prec, 1, Inst::Pop, false);

        prec -= 1;
        op(Tok::Or, prec, 1, Inst::Pop, false);

        prec -= 1;
        op(Tok::Assign, prec, 0, Inst::Pop, true);
        op(Tok::AddAssign, prec, 0, Inst::Add, true);
        op(Tok::SubAssign, prec, 0, Inst::Sub, true);
        op(Tok::MulAssign, prec, 0, Inst::Mul, true);
        op(Tok::IDivAssign, prec, 0, Inst::IDiv, true);
        op(Tok::DivAssign, prec, 0, Inst::Div, true);
        op(Tok::ModAssign, prec, 0, Inst::Mod, true);
        op(Tok::ShlAssign, prec, 0, Inst::Shl, true);
        op(Tok::ShrAssign, prec, 0, Inst::Shr, true);
        op(Tok::LShrAssign, prec, 0, Inst::LShr, true);
        op(Tok::BitAndAssign, prec, 0, Inst::BitAnd, true);
        op(Tok::BitAndAssign, prec, 0, Inst::BitAnd, true);
        op(Tok::BitOrAssign, prec, 0, Inst::BitOr, true);
        op(Tok::BitXorAssign, prec, 0, Inst::BitXor, true);
        op(Tok::PowAssign, prec, 0, Inst::Pow, true);

        Parser {
            keywords,
            ops,
            file: file.to_string().into(),
            reader,
            buf: Vec::new(),
            line: 0,
            start: 0,
            pos: 0,
            tok: Tok::Newline,
        }
    }

    fn error_context(&self) -> ErrorContext {
        ErrorContext::new(Rc::clone(&self.file), self.line)
    }

    fn error<S: AsRef<str>>(&mut self, msg: S) -> CompileError {
        CompileError::new(self.error_context(), msg.as_ref().to_string())
    }

    // Read a new line from the reader
    // Postconditions:
    // buf contains zero or more characters followed by \n
    // line has been incremented
    // OR:
    // buf is empty
    // EOF has been reached
    fn read(&mut self) -> Result<(), CompileError> {
        let mut s = String::new();
        match self.reader.read_line(&mut s) {
            Ok(0) => {
                self.buf = Vec::new();
            }
            Ok(_) => {
                self.buf = s.chars().collect();
                // The last line of a file is not guaranteed to end with \n
                // but the tokenizer needs every line to end with one
                if !s.ends_with('\n') {
                    self.buf.push('\n');
                }
                self.line += 1;
            }
            Err(e) => {
                return Err(CompileError::new(
                    // TODO: Is the error context correct here?
                    self.error_context(),
                    // TODO: Check how this looks on invalid UTF-8
                    format!("IO error: {}", e),
                ));
            }
        }

        Ok(())
    }

    // Tokenizer
    fn digits(&mut self) {
        let mut i = self.pos;
        while self.buf[i].is_ascii_digit() || self.buf[i] == '_' {
            i += 1;
        }
        self.pos = i
    }

    fn lex_id(&mut self) {
        let mut i = self.pos;
        loop {
            i += 1;
            if !is_id_part(self.buf[i]) {
                break;
            }
        }
        self.pos = i
    }

    fn hex_to_char(&mut self, i: usize, j: usize) -> Result<char, CompileError> {
        self.start = i;
        let s: String = substr(&self.buf, i, j);
        let n = match u32::from_str_radix(&s, 16) {
            Ok(n) => n,
            Err(e) => return Err(self.error(e.to_string())),
        };
        match char::from_u32(n) {
            Some(c) => Ok(c),
            None => Err(self.error("Not a valid Unicode character")),
        }
    }

    fn quote(&mut self) -> Result<(), CompileError> {
        let q = self.buf[self.pos];
        let mut i = self.pos + 1;
        let mut v = Vec::<char>::new();
        while self.buf[i] != q {
            let mut c = self.buf[i];
            i += 1;
            match c {
                '\n' => {
                    return Err(self.error("Unterminated string"));
                }
                '\\' => {
                    c = self.buf[i];
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
                            if self.buf[i] != '{' {
                                self.start = i;
                                return Err(self.error("Expected '{'"));
                            }
                            i += 1;

                            let mut j = i;
                            while self.buf[j].is_ascii_hexdigit() {
                                j += 1;
                            }
                            let c = self.hex_to_char(i, j)?;
                            i = j;

                            if self.buf[i] != '}' {
                                self.start = i;
                                return Err(self.error("Expected '}'"));
                            }
                            i += 1;

                            c
                        }
                        _ => {
                            self.start = i - 1;
                            return Err(self.error("Unknown escape character"));
                        }
                    }
                }
                _ => {}
            }
            v.push(c);
        }
        self.tok = Tok::Str(v.into_iter().collect());
        self.pos = i + 1;
        Ok(())
    }

    fn lex(&mut self) -> Result<(), CompileError> {
        while self.pos < self.buf.len() {
            self.start = self.pos;
            let c = self.buf[self.pos];
            match c {
                '"' | '\'' => {
                    self.quote()?;
                    return Ok(());
                }
                '#' => {
                    let mut i = self.pos + 1;
                    while self.buf[i] != '\n' {
                        i += 1;
                    }
                    self.pos = i;
                    continue;
                }
                ':' => {
                    self.tok = Tok::Colon;
                    self.pos += 1;
                    return Ok(());
                }
                '~' => {
                    self.pos += 1;
                    self.tok = Tok::BitNot;
                    return Ok(());
                }
                ',' => {
                    self.pos += 1;
                    self.tok = Tok::Comma;
                    return Ok(());
                }
                '+' => {
                    self.tok = match self.buf[self.pos + 1] {
                        '=' => {
                            self.pos += 2;
                            Tok::AddAssign
                        }
                        _ => {
                            self.pos += 1;
                            Tok::Add
                        }
                    };
                    return Ok(());
                }
                '%' => {
                    self.tok = match self.buf[self.pos + 1] {
                        '=' => {
                            self.pos += 2;
                            Tok::ModAssign
                        }
                        _ => {
                            self.pos += 1;
                            Tok::Mod
                        }
                    };
                    return Ok(());
                }
                '-' => {
                    self.tok = match self.buf[self.pos + 1] {
                        '=' => {
                            self.pos += 2;
                            Tok::SubAssign
                        }
                        _ => {
                            self.pos += 1;
                            Tok::Sub
                        }
                    };
                    return Ok(());
                }
                '&' => {
                    self.tok = match self.buf[self.pos + 1] {
                        '=' => {
                            self.pos += 2;
                            Tok::BitAndAssign
                        }
                        '&' => {
                            self.pos += 2;
                            Tok::And
                        }
                        _ => {
                            self.pos += 1;
                            Tok::BitAnd
                        }
                    };
                    return Ok(());
                }
                '|' => {
                    self.tok = match self.buf[self.pos + 1] {
                        '=' => {
                            self.pos += 2;
                            Tok::BitOrAssign
                        }
                        '|' => {
                            self.pos += 2;
                            Tok::Or
                        }
                        _ => {
                            self.pos += 1;
                            Tok::BitOr
                        }
                    };
                    return Ok(());
                }
                '^' => {
                    self.tok = match self.buf[self.pos + 1] {
                        '=' => {
                            self.pos += 2;
                            Tok::BitXorAssign
                        }
                        _ => {
                            self.pos += 1;
                            Tok::BitXor
                        }
                    };
                    return Ok(());
                }
                '*' => {
                    self.tok = match self.buf[self.pos + 1] {
                        '=' => {
                            self.pos += 2;
                            Tok::MulAssign
                        }
                        '*' => {
                            if self.buf[self.pos + 2] == '=' {
                                self.pos += 3;
                                Tok::PowAssign
                            } else {
                                self.pos += 2;
                                Tok::Pow
                            }
                        }
                        _ => {
                            self.pos += 1;
                            Tok::Mul
                        }
                    };
                    return Ok(());
                }
                '/' => {
                    self.tok = match self.buf[self.pos + 1] {
                        '=' => {
                            self.pos += 2;
                            Tok::DivAssign
                        }
                        '/' => {
                            if self.buf[self.pos + 2] == '=' {
                                self.pos += 3;
                                Tok::IDivAssign
                            } else {
                                self.pos += 2;
                                Tok::IDiv
                            }
                        }
                        _ => {
                            self.pos += 1;
                            Tok::Div
                        }
                    };
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
                '{' => {
                    self.pos += 1;
                    self.tok = Tok::LBrace;
                    return Ok(());
                }
                '}' => {
                    self.pos += 1;
                    self.tok = Tok::RBrace;
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
                '.' => {
                    // TODO: .digit
                    self.pos += 1;
                    self.tok = Tok::Dot;
                    return Ok(());
                }
                '=' => {
                    self.tok = match self.buf[self.pos + 1] {
                        '=' => {
                            self.pos += 2;
                            Tok::Eq
                        }
                        _ => {
                            self.pos += 1;
                            Tok::Assign
                        }
                    };
                    return Ok(());
                }
                '\n' => {
                    self.tok = Tok::Newline;
                    self.read()?;
                    self.pos = 0;
                    return Ok(());
                }
                ' ' | '\t' | '\r' | '\x0c' => {
                    self.pos += 1;
                    continue;
                }
                '<' => {
                    self.tok = match self.buf[self.pos + 1] {
                        '=' => {
                            self.pos += 2;
                            Tok::Le
                        }
                        '<' => {
                            if self.buf[self.pos + 2] == '=' {
                                self.pos += 3;
                                Tok::ShlAssign
                            } else {
                                self.pos += 2;
                                Tok::Shl
                            }
                        }
                        _ => {
                            self.pos += 1;
                            Tok::Lt
                        }
                    };
                    return Ok(());
                }
                '!' => {
                    self.tok = match self.buf[self.pos + 1] {
                        '=' => {
                            self.pos += 2;
                            Tok::Ne
                        }
                        _ => {
                            self.pos += 1;
                            Tok::Not
                        }
                    };
                    return Ok(());
                }
                '>' => {
                    self.tok = match self.buf[self.pos + 1] {
                        '=' => {
                            self.pos += 2;
                            Tok::Ge
                        }
                        '>' => match self.buf[self.pos + 2] {
                            '=' => {
                                self.pos += 3;
                                Tok::ShrAssign
                            }
                            '>' => {
                                if self.buf[self.pos + 3] == '=' {
                                    self.pos += 4;
                                    Tok::LShrAssign
                                } else {
                                    self.pos += 3;
                                    Tok::LShr
                                }
                            }
                            _ => {
                                self.pos += 2;
                                Tok::Shr
                            }
                        },
                        _ => {
                            self.pos += 1;
                            Tok::Gt
                        }
                    };
                    return Ok(());
                }
                _ => {
                    if c.is_ascii_digit() {
                        let i = self.pos;

                        // Alternative radix
                        if c == '0' {
                            match self.buf[i + 1] {
                                'x' | 'X' | 'b' | 'B' | 'o' | 'O' => {
                                    self.lex_id();
                                    let s = substr(&self.buf, i, self.pos);
                                    self.tok = Tok::PrefixedInt(s);
                                    return Ok(());
                                }
                                _ => {}
                            }
                        }

                        // Integer
                        self.digits();

                        // Decimal point
                        if self.buf[self.pos] == '.' {
                            self.pos += 1;
                            self.digits();
                        }

                        // Exponent
                        match self.buf[self.pos] {
                            'e' | 'E' => {
                                self.pos += 1;
                                match self.buf[self.pos] {
                                    '+' | '-' => {
                                        self.pos += 1;
                                    }
                                    _ => {}
                                }
                                self.digits();
                            }
                            _ => {}
                        }

                        // Token
                        let s = substr(&self.buf, i, self.pos);
                        self.tok = Tok::Num(s);

                        return Ok(());
                    }
                    if is_id_part(c) {
                        let i = self.pos;

                        // Word
                        self.lex_id();
                        let s = substr(&self.buf, i, self.pos);

                        // Keyword?
                        self.tok = match self.keywords.get(&s) {
                            Some(tok) => tok.clone(),
                            None => Tok::Id(s),
                        };

                        return Ok(());
                    }
                    return Err(self.error("Unknown character"));
                }
            }
        }
        self.tok = Tok::Eof;
        Ok(())
    }

    fn eat(&mut self, tok: Tok) -> Result<bool, CompileError> {
        if self.tok == tok {
            self.lex()?;
            return Ok(true);
        }
        Ok(false)
    }

    fn require(&mut self, tok: Tok, s: &str) -> Result<(), CompileError> {
        if !self.eat(tok)? {
            return Err(self.error(format!("Expected {}", s)));
        }
        Ok(())
    }

    fn id(&mut self) -> Result<String, CompileError> {
        let s = match &self.tok {
            Tok::Id(s) => s.clone(),
            _ => return Err(self.error("Expected name")),
        };
        self.lex()?;
        Ok(s)
    }

    // Expressions
    fn comma_separated(&mut self, v: &mut Vec<Expr>, end: Tok) -> Result<(), CompileError> {
        if self.tok == end {
            return Ok(());
        }
        loop {
            v.push(self.expr()?);
            if !self.eat(Tok::Comma)? {
                break;
            }
        }
        Ok(())
    }

    fn primary(&mut self) -> Result<Expr, CompileError> {
        let ec = self.error_context();
        let r = match &self.tok {
            Tok::True => {
                self.lex()?;
                Expr::True
            }
            Tok::False => {
                self.lex()?;
                Expr::False
            }
            Tok::Null => {
                self.lex()?;
                Expr::Null
            }
            Tok::Inf => {
                self.lex()?;
                Expr::Inf
            }
            Tok::Nan => {
                self.lex()?;
                Expr::Nan
            }
            Tok::Pi => {
                self.lex()?;
                Expr::Pi
            }
            Tok::LSquare => {
                let mut v = Vec::<Expr>::new();
                self.lex()?;
                self.comma_separated(&mut v, Tok::RSquare)?;
                self.require(Tok::RSquare, "']'")?;
                Expr::List(v)
            }
            Tok::LBrace => {
                let mut v = Vec::<Expr>::new();
                self.lex()?;
                if self.tok != Tok::RBrace {
                    loop {
                        let k = self.id()?;
                        v.push(Expr::Str(k));
                        self.require(Tok::Colon, "':'")?;
                        v.push(self.expr()?);
                        if !self.eat(Tok::Comma)? {
                            break;
                        }
                    }
                }
                self.require(Tok::RBrace, "'}'")?;
                Expr::Object(v)
            }
            Tok::LParen => {
                self.lex()?;
                let a = self.expr()?;
                self.require(Tok::RParen, "')'")?;
                a
            }
            Tok::Id(s) => {
                let s = s.clone();
                self.lex()?;
                Expr::Id(ec, s)
            }
            Tok::PrefixedInt(s) => {
                let s = s.replace('_', "");
                let a = match parse_prefixed_int(s) {
                    Ok(a) => a,
                    Err(e) => return Err(self.error(&e)),
                };
                let a = a as f64;
                self.lex()?;
                Expr::Num(a)
            }
            Tok::Num(s) => {
                let s = s.replace('_', "");
                let a = match s.parse::<f64>() {
                    Ok(a) => a,
                    Err(e) => return Err(self.error(e.to_string())),
                };
                self.lex()?;
                Expr::Num(a)
            }
            Tok::Str(s) => {
                let s = s.clone();
                self.lex()?;
                Expr::Str(s)
            }
            _ => return Err(self.error(format!("{:?}: Expected expression", self.tok))),
        };
        Ok(r)
    }

    fn postfix(&mut self) -> Result<Expr, CompileError> {
        let mut a = self.primary()?;
        loop {
            a = match &self.tok {
                Tok::Dot => {
                    let ec = self.error_context();
                    let a = Box::new(a);
                    self.lex()?;

                    let k = self.id()?;
                    let k = Expr::Str(k);
                    let k = Box::new(k);

                    Expr::Subscript(ec, a, k)
                }
                Tok::LSquare => {
                    let ec = self.error_context();
                    let a = Box::new(a);
                    self.lex()?;

                    let i = match self.tok {
                        Tok::Colon => Expr::Num(0.0),
                        _ => self.expr()?,
                    };
                    let i = Box::new(i);

                    let a = match self.tok {
                        Tok::RSquare => Expr::Subscript(ec, a, i),
                        Tok::Colon => {
                            self.lex()?;

                            let j = match self.tok {
                                Tok::RSquare => Expr::Null,
                                _ => self.expr()?,
                            };
                            let j = Box::new(j);

                            Expr::Slice(ec, a, i, j)
                        }
                        _ => return Err(self.error(format!("{:?}: Expected ':' or ']'", self.tok))),
                    };

                    self.require(Tok::RSquare, "']'")?;
                    a
                }
                Tok::LParen => {
                    let ec = self.error_context();
                    let mut v = Vec::<Expr>::new();
                    self.lex()?;
                    self.comma_separated(&mut v, Tok::RParen)?;
                    self.require(Tok::RParen, "')'")?;
                    Expr::Call(ec, Box::new(a), v)
                }
                _ => {
                    return Ok(a);
                }
            };
        }
    }

    fn prefix(&mut self) -> Result<Expr, CompileError> {
        match &self.tok {
            Tok::Not => {
                self.lex()?;
                let a = self.prefix()?;
                Ok(Expr::Prefix(ErrorContext::blank(), Inst::Not, Box::new(a)))
            }
            Tok::Sub => {
                let ec = self.error_context();
                self.lex()?;
                let a = self.prefix()?;
                Ok(Expr::Prefix(ec, Inst::Neg, Box::new(a)))
            }
            Tok::BitNot => {
                let ec = self.error_context();
                self.lex()?;
                let a = self.prefix()?;
                Ok(Expr::Prefix(ec, Inst::BitNot, Box::new(a)))
            }
            _ => self.postfix(),
        }
    }

    fn infix(&mut self, prec: u8) -> Result<Expr, CompileError> {
        // Operator precedence parser
        let mut a = self.prefix()?;
        loop {
            let o = match self.ops.get(&self.tok) {
                Some(o) => o.clone(),
                None => return Ok(a),
            };
            if o.prec < prec {
                return Ok(a);
            }
            let ec = self.error_context();
            let tok = self.tok.clone();
            self.lex()?;
            let b = self.infix(o.prec + o.left)?;
            let a1 = Box::new(a);
            let b1 = Box::new(b);
            a = match tok {
                Tok::Assign => Expr::Assign(ec, a1, b1),
                Tok::And => Expr::And(a1, b1),
                Tok::Or => Expr::Or(a1, b1),
                _ => {
                    if o.assign {
                        Expr::InfixAssign(ec, o.inst, a1, b1)
                    } else {
                        Expr::Infix(ec, o.inst, a1, b1)
                    }
                }
            }
        }
    }

    fn expr(&mut self) -> Result<Expr, CompileError> {
        self.infix(0)
    }

    // Statements
    fn block_end(&self) -> bool {
        matches!(self.tok, Tok::Else | Tok::End | Tok::Eof)
    }

    fn stmt(&mut self) -> Result<Stmt, CompileError> {
        let ec = self.error_context();
        let r = match self.tok {
            Tok::Func => {
                self.lex()?;
                let name = self.id()?;
                self.require(Tok::LParen, "'('")?;
                let mut params = Vec::<String>::new();
                if self.tok != Tok::RParen {
                    loop {
                        params.push(self.id()?);
                        if !self.eat(Tok::Comma)? {
                            break;
                        }
                    }
                }
                self.require(Tok::RParen, "')'")?;
                self.require(Tok::Newline, "newline")?;

                let mut body = Vec::<Stmt>::new();
                self.block(&mut body)?;

                self.require(Tok::End, "'end'")?;
                Stmt::Func(name, params, body)
            }
            Tok::For => {
                self.lex()?;
                let name = self.id()?;
                self.require(Tok::Colon, "':'")?;
                let collection = self.expr()?;
                self.require(Tok::Newline, "newline")?;

                let mut body = Vec::<Stmt>::new();
                self.block(&mut body)?;

                self.require(Tok::End, "'end'")?;
                Stmt::For(name, collection, body)
            }
            Tok::While => {
                self.lex()?;
                let cond = self.expr()?;
                self.require(Tok::Newline, "newline")?;

                let mut body = Vec::<Stmt>::new();
                self.block(&mut body)?;

                self.require(Tok::End, "'end'")?;
                Stmt::While(cond, body)
            }
            Tok::Dowhile => {
                self.lex()?;
                let cond = self.expr()?;
                self.require(Tok::Newline, "newline")?;

                let mut body = Vec::<Stmt>::new();
                self.block(&mut body)?;

                self.require(Tok::End, "'end'")?;
                Stmt::Dowhile(cond, body)
            }
            Tok::Assert => {
                self.lex()?;
                let cond = self.expr()?;
                let msg = if self.eat(Tok::Comma)? {
                    match &self.tok {
                        Tok::Str(s) => {
                            let s = s.to_string();
                            self.lex()?;
                            s
                        }
                        _ => return Err(self.error("Expected string")),
                    }
                } else {
                    "Assert failed".to_string()
                };
                Stmt::Assert(ec, cond, msg)
            }
            Tok::If => {
                self.lex()?;
                let cond = self.expr()?;
                self.require(Tok::Newline, "newline")?;

                let mut yes = Vec::<Stmt>::new();
                self.block(&mut yes)?;

                let mut no = Vec::<Stmt>::new();
                if self.eat(Tok::Else)? {
                    self.require(Tok::Newline, "newline")?;
                    self.block(&mut no)?;
                }

                self.require(Tok::End, "'end'")?;
                Stmt::If(cond, yes, no)
            }
            Tok::Goto => {
                // TODO: Check order of processing input
                self.lex()?;
                let label = self.id()?;
                Stmt::Goto(self.error_context(), label)
            }
            Tok::Return => {
                self.lex()?;
                let a = if self.tok == Tok::Newline {
                    Expr::Null
                } else {
                    self.expr()?
                };
                Stmt::Return(a)
            }
            Tok::Prin => {
                self.lex()?;
                let mut v = Vec::<Expr>::new();
                self.comma_separated(&mut v, Tok::Newline)?;
                Stmt::Prin(v)
            }
            Tok::Print => {
                self.lex()?;
                let mut v = Vec::<Expr>::new();
                self.comma_separated(&mut v, Tok::Newline)?;
                v.push(Expr::Str("\n".to_string()));
                Stmt::Prin(v)
            }
            _ => {
                let a = self.expr()?;
                if self.tok == Tok::Colon {
                    if let Expr::Id(ec, s) = a {
                        self.lex()?;
                        return Ok(Stmt::Label(ec, s));
                    }
                }
                Stmt::Expr(a)
            }
        };
        Ok(r)
    }

    fn block(&mut self, v: &mut Vec<Stmt>) -> Result<(), CompileError> {
        while !self.block_end() {
            if self.tok != Tok::Newline {
                v.push(self.stmt()?);
            }
            self.require(Tok::Newline, "newline")?;
        }
        Ok(())
    }

    fn parse(&mut self) -> Result<AST, CompileError> {
        // Start the tokenizer
        self.read()?;
        self.lex()?;

        // Parse
        let mut v = Vec::<Stmt>::new();
        self.block(&mut v)?;

        // Check for extra 'end' etc
        if self.tok != Tok::Eof {
            return Err(self.error("Unmatched terminator"));
        }

        Ok(AST {
            file: Rc::clone(&self.file),
            code: mem::take(&mut v),
        })
    }
}

pub fn parse<R: BufRead>(file: &str, reader: R) -> Result<AST, CompileError> {
    let mut parser = Parser::new(file, reader);
    parser.parse()
}

pub fn parse_str(file: &str, text: &str) -> Result<AST, CompileError> {
    let cursor = Cursor::new(text);
    let reader = BufReader::new(cursor);
    parse(file, reader)
}
