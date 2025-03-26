use crate::ast::*;
use crate::compile_error::*;
use crate::error_context::*;
use crate::str32::*;
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Cursor};
use std::mem;
use std::rc::Rc;

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
enum Tok {
    DoWhile,
    While,
    Int(String),
    Float(String),
    Str(Str32),
    Id(String),
    Colon,
    Newline,
    LParen,
    RParen,
    Assert,
    LSquare,
    RSquare,
    Eof,
    Semi,
    For,
    In,
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
    FDivAssign,
    FDiv,
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
    IDivAssign,
    True,
    False,
    Null,
    IDiv,
    ShlAssign,
    Shl,
    Println,
    Print,
    If,
}

// The operator precedence parser uses a table of these
#[derive(Clone)]
struct Op {
    prec: u8,
    left: u8,
    name: String,
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

impl<R: BufRead> Parser<R> {
    fn new(file: &str, reader: R) -> Self {
        // Keywords
        let mut keywords = HashMap::new();
        keywords.insert("assert".to_string(), Tok::Assert);
        keywords.insert("in".to_string(), Tok::In);
        keywords.insert("if".to_string(), Tok::If);
        keywords.insert("print".to_string(), Tok::Print);
        keywords.insert("println".to_string(), Tok::Println);
        keywords.insert("else".to_string(), Tok::Else);
        keywords.insert("end".to_string(), Tok::End);
        keywords.insert("return".to_string(), Tok::Return);
        keywords.insert("goto".to_string(), Tok::Goto);
        keywords.insert("for".to_string(), Tok::For);
        keywords.insert("while".to_string(), Tok::While);
        keywords.insert("do_while".to_string(), Tok::DoWhile);
        keywords.insert("true".to_string(), Tok::True);
        keywords.insert("false".to_string(), Tok::False);
        keywords.insert("null".to_string(), Tok::Null);

        // Infix operators
        let mut ops = HashMap::new();
        let mut op = |tok: Tok, prec: u8, left: u8, name: &str| {
            ops.insert(
                tok,
                Op {
                    prec,
                    left,
                    name: name.to_string(),
                },
            );
        };

        let mut prec = 99u8;
        op(Tok::Pow, prec, 0, "_pow");

        prec -= 1;
        op(Tok::Mul, prec, 1, "_mul");
        op(Tok::FDiv, prec, 1, "_fdiv");
        op(Tok::IDiv, prec, 1, "_idiv");
        op(Tok::Mod, prec, 1, "_mod");

        prec -= 1;
        op(Tok::Add, prec, 1, "_add");
        op(Tok::Sub, prec, 1, "_sub");

        prec -= 1;
        op(Tok::Shl, prec, 1, "_shl");
        op(Tok::Shr, prec, 1, "_shr");

        prec -= 1;
        op(Tok::BitAnd, prec, 1, "_bit_and");

        prec -= 1;
        op(Tok::BitXor, prec, 1, "_bit_xor");

        prec -= 1;
        op(Tok::BitOr, prec, 1, "_bit_or");

        prec -= 1;
        op(Tok::Eq, prec, 1, "_eq");
        op(Tok::Ne, prec, 1, "_ne");
        op(Tok::Lt, prec, 1, "_lt");
        op(Tok::Le, prec, 1, "_le");
        op(Tok::Gt, prec, 1, "_gt");
        op(Tok::Ge, prec, 1, "_ge");

        prec -= 1;
        op(Tok::And, prec, 1, "");

        prec -= 1;
        op(Tok::Or, prec, 1, "");

        prec -= 1;
        op(Tok::Assign, prec, 0, "");
        op(Tok::AddAssign, prec, 0, "add");
        op(Tok::SubAssign, prec, 0, "sub");
        op(Tok::MulAssign, prec, 0, "mul");
        op(Tok::IDivAssign, prec, 0, "idiv");
        op(Tok::FDivAssign, prec, 0, "fdiv");
        op(Tok::ModAssign, prec, 0, "mod");
        op(Tok::ShlAssign, prec, 0, "shl");
        op(Tok::ShrAssign, prec, 0, "shr");
        op(Tok::BitAndAssign, prec, 0, "bit_and");
        op(Tok::BitOrAssign, prec, 0, "bit_or");
        op(Tok::BitXorAssign, prec, 0, "bit_xor");
        op(Tok::PowAssign, prec, 0, "pow");

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
        self.tok = Tok::Str(Str32::from_vec(v));
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
                ';' => {
                    self.pos += 1;
                    self.tok = Tok::Semi;
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
                            Tok::FDivAssign
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
                            Tok::FDiv
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
                    if c.is_ascii_digit() {
                        let i = self.pos;

                        // Alternative radix
                        if c == '0' {
                            match self.buf[i + 1] {
                                'x' | 'X' | 'b' | 'B' | 'o' | 'O' => {
                                    self.lex_id();
                                    let s = substr(&self.buf, i, self.pos);
                                    self.tok = Tok::Int(s);
                                    return Ok(());
                                }
                                _ => {}
                            }
                        }

                        // Integer
                        self.digits();
                        let mut is_float = false;

                        // Decimal point
                        if self.buf[self.pos] == '.' {
                            self.pos += 1;
                            self.digits();
                            is_float = true;
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
                                is_float = true;
                            }
                            _ => {}
                        }

                        // Token
                        let s = substr(&self.buf, i, self.pos);
                        self.tok = if is_float { Tok::Float(s) } else { Tok::Int(s) };

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

    // Expressions
    fn comma_separated(&mut self, v: &mut Vec<Expr>) -> Result<(), CompileError> {
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
        match &self.tok {
            Tok::True => {
                self.lex()?;
                Ok(Expr::True)
            }
            Tok::False => {
                self.lex()?;
                Ok(Expr::False)
            }
            Tok::Null => {
                self.lex()?;
                Ok(Expr::Null)
            }
            Tok::LBrace => {
                let mut v = Vec::<Expr>::new();
                self.lex()?;
                if self.tok != Tok::RBrace {
                    self.comma_separated(&mut v)?;
                }
                self.require(Tok::RBrace, "'}'")?;
                Ok(Expr::Call(
                    ec.clone(),
                    Box::new(Expr::Id(ec, "_list".to_string())),
                    v,
                ))
            }
            Tok::LParen => {
                self.lex()?;
                let a = self.expr()?;
                self.require(Tok::RParen, "')'")?;
                Ok(a)
            }
            Tok::LSquare => {
                self.lex()?;
                let a = self.expr()?;
                self.require(Tok::RSquare, "']'")?;
                Ok(a)
            }
            Tok::Id(s) => {
                let s = s.clone();
                self.lex()?;
                Ok(Expr::Id(ec, s))
            }
            Tok::Int(s) => {
                let s = s.clone();
                self.lex()?;
                Ok(Expr::Int(ec, s))
            }
            Tok::Float(s) => {
                let s = s.clone();
                self.lex()?;
                Ok(Expr::Float(ec, s))
            }
            Tok::Str(s) => {
                let s = s.clone();
                self.lex()?;
                Ok(Expr::Str(s))
            }
            _ => Err(self.error(format!("{:?}: Expected expression", self.tok))),
        }
    }

    fn postfix(&mut self) -> Result<Expr, CompileError> {
        let mut a = self.primary()?;
        loop {
            a = match &self.tok {
                Tok::Id(_) | Tok::Int(_) | Tok::Float(_) | Tok::Str(_) => {
                    let ec = self.error_context();
                    let b = self.postfix()?;
                    Expr::Call(ec, Box::new(a), vec![b])
                }
                Tok::LSquare => {
                    let ec = self.error_context();
                    let mut v = Vec::<Expr>::new();
                    self.lex()?;
                    if self.tok != Tok::RSquare {
                        self.comma_separated(&mut v)?;
                    }
                    self.require(Tok::RSquare, "']'")?;
                    Expr::Call(ec, Box::new(a), v)
                }
                Tok::LParen => {
                    let ec = self.error_context();
                    let mut v = Vec::<Expr>::new();
                    self.lex()?;
                    if self.tok != Tok::RParen {
                        self.comma_separated(&mut v)?;
                    }
                    self.require(Tok::RParen, "']'")?;
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
                let ec = self.error_context();
                self.lex()?;
                let a = self.prefix()?;
                Ok(Expr::Call(
                    ec.clone(),
                    Box::new(Expr::Id(ec, "_not".to_string())),
                    vec![a],
                ))
            }
            Tok::Sub => {
                let ec = self.error_context();
                self.lex()?;
                let a = self.prefix()?;
                Ok(Expr::Call(
                    ec.clone(),
                    Box::new(Expr::Id(ec, "_neg".to_string())),
                    vec![a],
                ))
            }
            Tok::BitNot => {
                let ec = self.error_context();
                self.lex()?;
                let a = self.prefix()?;
                Ok(Expr::Call(
                    ec.clone(),
                    Box::new(Expr::Id(ec, "_bit_not".to_string())),
                    vec![a],
                ))
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
            a = match tok {
                Tok::Assign => Expr::Assign(ec, Box::new(a), Box::new(b)),
                Tok::And => Expr::And(Box::new(a), Box::new(b)),
                Tok::Or => Expr::Or(Box::new(a), Box::new(b)),
                _ => {
                    if o.name.starts_with('_') {
                        Expr::Call(ec.clone(), Box::new(Expr::Id(ec, o.name)), vec![a, b])
                    } else {
                        Expr::OpAssign(ec, o.name, Box::new(a), Box::new(b))
                    }
                }
            }
        }
    }

    fn expr(&mut self) -> Result<Expr, CompileError> {
        self.infix(0)
    }

    // Statements
    fn id(&mut self) -> Result<String, CompileError> {
        let s = match &self.tok {
            Tok::Id(s) => s.clone(),
            _ => return Err(self.error("Expected name")),
        };
        self.lex()?;
        Ok(s)
    }

    fn is_end(&self) -> bool {
        matches!(self.tok, Tok::Else | Tok::End | Tok::Eof)
    }

    fn maybe_comma_separated(&mut self, v: &mut Vec<Expr>) -> Result<(), CompileError> {
        if matches!(self.tok, Tok::Semi | Tok::Newline) || self.is_end() {
            return Ok(());
        }
        self.comma_separated(v)
    }

    fn stmt(&mut self) -> Result<Stmt, CompileError> {
        let ec = self.error_context();
        match self.tok {
            Tok::For => {
                self.lex()?;
                let name = self.id()?;
                self.require(Tok::In, "in")?;
                let collection = self.expr()?;
                self.require(Tok::Newline, "newline")?;
                let mut v = Vec::<Stmt>::new();
                self.stmts(&mut v)?;
                match &self.tok {
                    Tok::End => {
                        self.lex()?;
                    }
                    _ => return Err(self.error("Expected END")),
                }
                Ok(Stmt::For(name, collection, v))
            }
            Tok::While => {
                self.lex()?;
                let cond = self.expr()?;
                self.require(Tok::Newline, "newline")?;
                let mut v = Vec::<Stmt>::new();
                self.stmts(&mut v)?;
                match &self.tok {
                    Tok::End => {
                        self.lex()?;
                    }
                    _ => return Err(self.error("Expected END")),
                }
                Ok(Stmt::While(cond, v))
            }
            Tok::Assert => {
                self.lex()?;
                let cond = self.expr()?;
                let a = Expr::Call(
                    ec.clone(),
                    Box::new(Expr::Id(ec, "_assert".to_string())),
                    vec![cond],
                );
                Ok(Stmt::Expr(a))
            }
            Tok::If => {
                self.lex()?;
                let cond = self.expr()?;
                let mut yes = Vec::<Stmt>::new();
                let mut no = Vec::<Stmt>::new();
                self.require(Tok::Newline, "newline")?;
                self.stmts(&mut yes)?;
                if self.tok == Tok::Else {
                    self.lex()?;
                    self.stmts(&mut no)?;
                }
                match &self.tok {
                    Tok::End => {
                        self.lex()?;
                    }
                    _ => return Err(self.error("Expected END")),
                }
                Ok(Stmt::If(cond, yes, no))
            }
            Tok::Goto => {
                // TODO: Check order of processing input
                self.lex()?;
                let label = self.id()?;
                Ok(Stmt::Goto(self.error_context(), label))
            }
            Tok::Return => {
                self.lex()?;
                Ok(Stmt::Return)
            }
            Tok::Print => {
                self.lex()?;
                let mut v = Vec::<Expr>::new();
                self.comma_separated(&mut v)?;
                Ok(Stmt::Print(ec, v))
            }
            Tok::Println => {
                self.lex()?;
                let mut v = Vec::<Expr>::new();
                self.maybe_comma_separated(&mut v)?;
                v.push(Expr::Str("\n".to_string()));
                Ok(Stmt::Print(ec, v))
            }
            _ => {
                let a = self.expr()?;
                if self.tok == Tok::Colon {
                    match a {
                        Expr::Id(ec, s) => {
                            self.lex()?;
                            return Ok(Stmt::Label(ec, s));
                        }
                        _ => {}
                    }
                }
                Ok(Stmt::Expr(a))
            }
        }
    }

    fn stmts(&mut self, v: &mut Vec<Stmt>) -> Result<(), CompileError> {
        while !self.is_end() {
            if self.eat(Tok::Newline)? {
                continue;
            }
            v.push(self.stmt()?);
            match self.tok {
                Tok::Newline | Tok::Semi => {
                    self.lex()?;
                }
                _ => {
                    if !self.is_end() {
                        return Err(self.error("Syntax error"));
                    }
                }
            }
        }
        Ok(())
    }

    fn parse(&mut self) -> Result<AST, CompileError> {
        // Start the tokenizer
        self.read()?;
        self.lex()?;

        // Parse
        let mut v = Vec::<Stmt>::new();
        self.stmts(&mut v)?;

        // Check for extra stuff we couldn't parse
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
