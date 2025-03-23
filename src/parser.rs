use crate::ast::*;
use crate::compile_error::*;
use crate::error_context::*;
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Cursor};
use std::mem;
use std::rc::Rc;

// TODO: CamelCase consistency
#[derive(Clone, Hash, PartialEq, Eq)]
enum Tok {
    While,
    Int(String),
    Float(String),
    Str(String),
    Id(String),
    Colon,
    Input,
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
    Star,
    Pow,
    Plus,
    Tilde,
    Minus,
    Slash,
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
    Div,
    Shl,
    Print,
    Mod,
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
    c.is_alphanumeric() || c == '_' || c == '$'
}

fn substr(buf: &[char], i: usize, j: usize) -> String {
    buf.iter().skip(i).take(j - i).collect()
}

impl<R: BufRead> Parser<R> {
    fn new(file: &str, reader: R) -> Self {
        // Keywords
        let mut keywords = HashMap::new();
        keywords.insert("assert".to_string(), Tok::Assert);
        keywords.insert("mod".to_string(), Tok::Mod);
        keywords.insert("in".to_string(), Tok::In);
        keywords.insert("if".to_string(), Tok::If);
        keywords.insert("print".to_string(), Tok::Print);
        keywords.insert("div".to_string(), Tok::Div);
        keywords.insert("and".to_string(), Tok::And);
        keywords.insert("input".to_string(), Tok::Input);
        keywords.insert("or".to_string(), Tok::Or);
        keywords.insert("not".to_string(), Tok::Not);
        keywords.insert("else".to_string(), Tok::Else);
        keywords.insert("end".to_string(), Tok::End);
        keywords.insert("return".to_string(), Tok::Return);
        keywords.insert("goto".to_string(), Tok::Goto);
        keywords.insert("for".to_string(), Tok::For);
        keywords.insert("while".to_string(), Tok::While);

        // Infix operators
        let mut ops = HashMap::new();
        let mut add = |o: Tok, prec: u8, left: u8, name: &str| {
            ops.insert(
                o,
                Op {
                    prec,
                    left,
                    name: name.to_string(),
                },
            );
        };

        let mut prec = 99u8;
        add(Tok::Pow, prec, 0, "_pow");

        prec -= 1;
        add(Tok::Star, prec, 1, "_mul");
        add(Tok::Slash, prec, 1, "_fdiv");
        add(Tok::Div, prec, 1, "_idiv");
        add(Tok::Mod, prec, 1, "_mod");

        prec -= 1;
        add(Tok::Plus, prec, 1, "_add");
        add(Tok::Minus, prec, 1, "_sub");

        prec -= 1;
        add(Tok::Shl, prec, 1, "_shl");
        add(Tok::Shr, prec, 1, "_shr");

        prec -= 1;
        add(Tok::Eq, prec, 1, "_eq");
        add(Tok::Ne, prec, 1, "_ne");
        add(Tok::Lt, prec, 1, "_lt");
        add(Tok::Le, prec, 1, "_le");
        add(Tok::Gt, prec, 1, "_gt");
        add(Tok::Ge, prec, 1, "_ge");

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

    fn errorContext(&self) -> ErrorContext {
        ErrorContext::new(Rc::clone(&self.file), self.line)
    }

    fn error<S: AsRef<str>>(&mut self, msg: S) -> CompileError {
        CompileError::new(self.errorContext(), msg.as_ref().to_string())
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
                    self.errorContext(),
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
        self.tok = Tok::Str(String::from_iter(v));
        self.pos = i + 1;
        return Ok(());
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
                    self.tok = Tok::Tilde;
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
                    if self.buf[self.pos] == '=' {
                        self.pos += 1;
                    }
                    self.tok = Tok::Eq;
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
                    self.tok = match self.buf[self.pos + 1] {
                        '=' => {
                            self.pos += 2;
                            Tok::Ne
                        }
                        _ => {
                            self.start = self.pos + 1;
                            return Err(self.error("Expected '='"));
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
        let ec = self.errorContext();
        match &self.tok {
            Tok::LSquare => {
                let mut v = Vec::<Expr>::new();
                self.lex()?;
                if self.tok != Tok::RSquare {
                    self.comma_separated(&mut v)?;
                }
                self.require(Tok::RSquare, "']'")?;
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
            _ => Err(self.error("Expected expression")),
        }
    }

    fn postfix(&mut self) -> Result<Expr, CompileError> {
        let ec = self.errorContext();
        let a = self.primary()?;
        match &self.tok {
            Tok::Id(_) | Tok::Int(_) | Tok::Float(_) | Tok::Str(_) => {
                let b = self.postfix()?;
                Ok(Expr::Call(ec, Box::new(a), vec![b]))
            }
            Tok::LSquare => {
                let mut v = Vec::<Expr>::new();
                self.lex()?;
                if self.tok != Tok::RSquare {
                    self.comma_separated(&mut v)?;
                }
                self.require(Tok::RSquare, "']'")?;
                Ok(Expr::Call(ec, Box::new(a), v))
            }
            Tok::LParen => {
                let mut v = Vec::<Expr>::new();
                self.lex()?;
                if self.tok != Tok::RParen {
                    self.comma_separated(&mut v)?;
                }
                self.require(Tok::RParen, "']'")?;
                Ok(Expr::Call(ec, Box::new(a), v))
            }
            _ => Ok(a),
        }
    }

    fn prefix(&mut self) -> Result<Expr, CompileError> {
        // TODO: !
        match &self.tok {
            Tok::Minus => {
                let ec = self.errorContext();
                self.lex()?;
                let a = self.prefix()?;
                Ok(Expr::Call(
                    ec.clone(),
                    Box::new(Expr::Id(ec, "_neg".to_string())),
                    vec![a],
                ))
            }
            Tok::Tilde => {
                let ec = self.errorContext();
                self.lex()?;
                let a = self.prefix()?;
                Ok(Expr::Call(
                    ec.clone(),
                    Box::new(Expr::Id(ec, "_bitnot".to_string())),
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
            let ec = self.errorContext();
            self.lex()?;
            let b = self.infix(o.prec + o.left)?;
            a = Expr::Call(ec.clone(), Box::new(Expr::Id(ec, o.name)), vec![a, b]);
        }
    }

    fn not(&mut self) -> Result<Expr, CompileError> {
        if self.tok == Tok::Not {
            let ec = self.errorContext();
            self.lex()?;
            let a = self.not()?;
            Ok(Expr::Call(
                ec.clone(),
                Box::new(Expr::Id(ec, "_not".to_string())),
                vec![a],
            ))
        } else {
            self.infix(0)
        }
    }

    fn and(&mut self) -> Result<Expr, CompileError> {
        let a = self.not()?;
        if self.tok == Tok::And {
            self.lex()?;
            let b = self.and()?;
            return Ok(Expr::And(Box::new(a), Box::new(b)));
        }
        Ok(a)
    }

    fn or(&mut self) -> Result<Expr, CompileError> {
        let a = self.and()?;
        if self.tok == Tok::Or {
            self.lex()?;
            let b = self.or()?;
            return Ok(Expr::Or(Box::new(a), Box::new(b)));
        }
        Ok(a)
    }

    fn expr(&mut self) -> Result<Expr, CompileError> {
        self.or()
    }

    // Statements
    fn id(&mut self) -> Result<String, CompileError> {
        let s = match &self.tok {
            Tok::Id(s) => {
                let s = s.clone();
                s
            }
            _ => return Err(self.error("Expected name")),
        };
        self.lex()?;
        Ok(s)
    }

    fn label(&mut self) -> Result<Expr, CompileError> {
        let label = match &self.tok {
            Tok::Int(_) | Tok::Float(_) | Tok::Id(_) => self.primary()?,
            _ => return Err(self.error("Expected label")),
        };
        self.lex()?;
        Ok(label)
    }

    fn stmt(&mut self) -> Result<Stmt, CompileError> {
        // TODO: optimize
        let tok = self.tok.clone();
        match tok {
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
                Ok(Stmt::Assert(self.errorContext(), cond))
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
                Ok(Stmt::Goto(self.errorContext(), label))
            }
            Tok::Return => {
                self.lex()?;
                Ok(Stmt::Return)
            }
            Tok::Print => {
                self.lex()?;
                let mut v = Vec::<Expr>::new();
                self.comma_separated(&mut v)?;
                Ok(Stmt::Print(v))
            }
            Tok::Id(s) => {
                if self.buf[self.pos] == ':' {
                    return Ok(Stmt::Label(self.errorContext(), self.id()?));
                }
                return Err(self.error("Syntax error"));
            }
            // TODO
            _ => return Err(self.error("Syntax error")),
        }
    }

    fn is_end(&self) -> bool {
        matches!(self.tok, Tok::Else | Tok::End | Tok::Eof)
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
