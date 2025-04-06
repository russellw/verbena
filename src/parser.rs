use crate::ast::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::process;

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
enum Tok {
    Try,
    Catch,
    Throw,
    Dowhile,
    While,
    Func,
    Case,
    Atom(String),
    Input,
    Colon,
    Newline,
    Elif,
    EPrint,
    EPrin,
    Outer,
    Typeof,
    LParen,
    RParen,
    Assert,
    Dot,
    LSquare,
    RSquare,
    Eof,
    For,
    Return,
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
    Pipe,
    BitXorAssign,
    BitXor,
    SubAssign,
    Minus,
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
    ShlAssign,
    Shl,
    Print,
    Prin,
    If,
}

// The operator precedence parser uses a table of these
#[derive(Clone)]
struct Op {
    prec: u8,
    left: u8,
    s: String,
    compound_assign: bool,
}

struct Parser {
    // There is a compile-time perfect hash package
    // but there are benchmarks showing HashMap to be faster
    keywords: HashMap<String, Tok>,

    // Table of infix operators
    ops: HashMap<Tok, Op>,

    // Input
    file: String,
    text: Vec<char>,

    // Current position in the text
    pos: usize,

    // Line number tracker for err reporting
    line: usize,

    // Current token
    tok: Tok,
}

fn is_id_start(c: char) -> bool {
    c.is_alphabetic() || c == '_' || c == '$'
}

fn is_id_part(c: char) -> bool {
    c.is_alphanumeric() || c == '_' || c == '$'
}

fn substr(text: &[char], i: usize, j: usize) -> String {
    text.iter().skip(i).take(j - i).collect()
}

impl Parser {
    fn new(file: String, text: Vec<char>) -> Self {
        // Keywords
        let mut keywords = HashMap::new();
        keywords.insert("assert".to_string(), Tok::Assert);
        keywords.insert("case".to_string(), Tok::Case);
        keywords.insert("elif".to_string(), Tok::Elif);
        keywords.insert("if".to_string(), Tok::If);
        keywords.insert("eprin".to_string(), Tok::EPrin);
        keywords.insert("prin".to_string(), Tok::Prin);
        keywords.insert("eprint".to_string(), Tok::EPrint);
        keywords.insert("print".to_string(), Tok::Print);
        keywords.insert("else".to_string(), Tok::Else);
        keywords.insert("fn".to_string(), Tok::Func);
        keywords.insert("input".to_string(), Tok::Input);
        keywords.insert("end".to_string(), Tok::End);
        keywords.insert("return".to_string(), Tok::Return);
        keywords.insert("typeof".to_string(), Tok::Typeof);
        keywords.insert("for".to_string(), Tok::For);
        keywords.insert("while".to_string(), Tok::While);
        keywords.insert("outer".to_string(), Tok::Outer);
        keywords.insert("dowhile".to_string(), Tok::Dowhile);
        keywords.insert("try".to_string(), Tok::Try);
        keywords.insert("throw".to_string(), Tok::Throw);
        keywords.insert("catch".to_string(), Tok::Catch);

        // Infix operators
        let mut ops = HashMap::new();
        let mut op = |tok: Tok, prec: u8, left: u8, s: &str, compound_assign: bool| {
            ops.insert(
                tok,
                Op {
                    prec,
                    left,
                    s: s.to_string(),
                    compound_assign,
                },
            );
        };

        let mut prec = 99u8;
        op(Tok::Pow, prec, 0, "**", false);

        prec -= 1;
        op(Tok::Mul, prec, 1, "*", false);
        op(Tok::Div, prec, 1, "/", false);
        op(Tok::Mod, prec, 1, "%", false);

        prec -= 1;
        op(Tok::Add, prec, 1, "+", false);
        op(Tok::Minus, prec, 1, "-", false);

        prec -= 1;
        op(Tok::Shl, prec, 1, "<<", false);
        op(Tok::Shr, prec, 1, ">>", false);
        op(Tok::LShr, prec, 1, ">>>", false);

        prec -= 1;
        op(Tok::BitAnd, prec, 1, "&", false);

        prec -= 1;
        op(Tok::BitXor, prec, 1, "^", false);

        prec -= 1;
        op(Tok::Pipe, prec, 1, "|", false);

        prec -= 1;
        op(Tok::Eq, prec, 1, "===", false);
        op(Tok::Ne, prec, 1, "!==", false);
        op(Tok::Lt, prec, 1, "<", false);
        op(Tok::Le, prec, 1, "<=", false);
        op(Tok::Gt, prec, 1, ">", false);
        op(Tok::Ge, prec, 1, ">=", false);

        prec -= 1;
        op(Tok::And, prec, 1, "&&", false);

        prec -= 1;
        op(Tok::Or, prec, 1, "||", false);

        prec -= 1;
        op(Tok::Assign, prec, 0, "=", false);
        op(Tok::AddAssign, prec, 0, "+", true);
        op(Tok::SubAssign, prec, 0, "-", true);
        op(Tok::MulAssign, prec, 0, "*", true);
        op(Tok::DivAssign, prec, 0, "/", true);
        op(Tok::ModAssign, prec, 0, "%", true);
        op(Tok::ShlAssign, prec, 0, "<<", true);
        op(Tok::ShrAssign, prec, 0, ">>", true);
        op(Tok::LShrAssign, prec, 0, ">>>", true);
        op(Tok::BitAndAssign, prec, 0, "&", true);
        op(Tok::BitOrAssign, prec, 0, "|", true);
        op(Tok::BitXorAssign, prec, 0, "^", true);
        op(Tok::PowAssign, prec, 0, "**", true);

        Parser {
            keywords,
            ops,
            file,
            text,
            pos: 0,
            line: 1,
            tok: Tok::Newline,
        }
    }

    fn src(&self) -> Src {
        Src {
            file: self.file.clone(),
            line: self.line,
        }
    }

    fn err<S: AsRef<str>>(&self, msg: S) -> ! {
        eprintln!("{}: {}", self.src(), msg.as_ref());
        process::exit(1);
    }

    // Tokenizer
    fn lex_id(&mut self) {
        let mut i = self.pos;
        loop {
            i += 1;
            if !is_id_part(self.text[i]) {
                break;
            }
        }
        self.pos = i
    }

    fn lex_num(&mut self) {
        let i = self.pos;

        // Integer
        self.lex_id();

        // Decimal point
        if self.text[self.pos] == '.' {
            self.pos += 1;
            self.lex_id();
        }

        // Exponent
        match self.text[i + 1] {
            'x' | 'X' => {}
            _ => match self.text[self.pos] {
                'e' | 'E' => {
                    self.pos += 1;
                    match self.text[self.pos] {
                        '+' | '-' => {
                            self.pos += 1;
                        }
                        _ => {}
                    }
                    self.lex_id();
                }
                _ => {}
            },
        }

        // Token
        let s = substr(&self.text, i, self.pos);
        self.tok = Tok::Atom(s);
    }

    fn lex(&mut self) {
        while self.pos < self.text.len() {
            let c = self.text[self.pos];
            match c {
                '"' | '\'' => {
                    let q = self.text[self.pos];
                    let mut i = self.pos + 1;
                    while self.text[i] != q {
                        let c = self.text[i];
                        if c == '\n' {
                            self.err("Unterminated string");
                        }
                        i += 1;

                        // Backslash can escape many things
                        // but most of them can be left to the JavaScript compiler to interpret
                        // The only things we need to worry about here are:
                        // Escaping a closing quote
                        // Escaping a backslash that might otherwise escape a closing quote
                        if c == '\\' && (self.text[i] == q || self.text[i] == '\\') {
                            i += 1;
                        }
                    }
                    i += 1;
                    let s = substr(&self.text, self.pos, i);
                    self.pos = i;
                    self.tok = Tok::Atom(s);
                    return;
                }
                ':' => {
                    self.pos += 1;
                    self.tok = Tok::Colon;
                    return;
                }
                '~' => {
                    self.pos += 1;
                    self.tok = Tok::BitNot;
                    return;
                }
                ',' => {
                    self.pos += 1;
                    self.tok = Tok::Comma;
                    return;
                }
                '+' => {
                    self.tok = match self.text[self.pos + 1] {
                        '=' => {
                            self.pos += 2;
                            Tok::AddAssign
                        }
                        _ => {
                            self.pos += 1;
                            Tok::Add
                        }
                    };
                    return;
                }
                '%' => {
                    self.tok = match self.text[self.pos + 1] {
                        '=' => {
                            self.pos += 2;
                            Tok::ModAssign
                        }
                        _ => {
                            self.pos += 1;
                            Tok::Mod
                        }
                    };
                    return;
                }
                '-' => {
                    self.tok = match self.text[self.pos + 1] {
                        '=' => {
                            self.pos += 2;
                            Tok::SubAssign
                        }
                        _ => {
                            self.pos += 1;
                            Tok::Minus
                        }
                    };
                    return;
                }
                '&' => {
                    self.tok = match self.text[self.pos + 1] {
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
                    return;
                }
                '|' => {
                    self.tok = match self.text[self.pos + 1] {
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
                            Tok::Pipe
                        }
                    };
                    return;
                }
                '^' => {
                    self.tok = match self.text[self.pos + 1] {
                        '=' => {
                            self.pos += 2;
                            Tok::BitXorAssign
                        }
                        _ => {
                            self.pos += 1;
                            Tok::BitXor
                        }
                    };
                    return;
                }
                '*' => {
                    self.tok = match self.text[self.pos + 1] {
                        '=' => {
                            self.pos += 2;
                            Tok::MulAssign
                        }
                        '*' => {
                            if self.text[self.pos + 2] == '=' {
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
                    return;
                }
                '/' => {
                    self.tok = match self.text[self.pos + 1] {
                        '=' => {
                            self.pos += 2;
                            Tok::DivAssign
                        }
                        _ => {
                            self.pos += 1;
                            Tok::Div
                        }
                    };
                    return;
                }
                '(' => {
                    self.pos += 1;
                    self.tok = Tok::LParen;
                    return;
                }
                ')' => {
                    self.pos += 1;
                    self.tok = Tok::RParen;
                    return;
                }
                '{' => {
                    self.pos += 1;
                    self.tok = Tok::LBrace;
                    return;
                }
                '}' => {
                    self.pos += 1;
                    self.tok = Tok::RBrace;
                    return;
                }
                '[' => {
                    self.pos += 1;
                    self.tok = Tok::LSquare;
                    return;
                }
                ']' => {
                    self.pos += 1;
                    self.tok = Tok::RSquare;
                    return;
                }
                '.' => {
                    if self.text[self.pos + 1].is_ascii_digit() {
                        self.lex_num();
                        return;
                    }
                    self.pos += 1;
                    self.tok = Tok::Dot;
                    return;
                }
                '=' => {
                    self.tok = match self.text[self.pos + 1] {
                        '=' => {
                            self.pos += 2;
                            Tok::Eq
                        }
                        _ => {
                            self.pos += 1;
                            Tok::Assign
                        }
                    };
                    return;
                }
                '\n' | '#' => {
                    while self.pos < self.text.len() {
                        let c = self.text[self.pos];
                        if c.is_whitespace() {
                            if c == '\n' {
                                self.line += 1;
                            }
                            self.pos += 1;
                        } else if c == '#' {
                            while self.text[self.pos] != '\n' {
                                self.pos += 1;
                            }
                        } else {
                            break;
                        }
                    }
                    self.tok = Tok::Newline;
                    return;
                }
                '<' => {
                    self.tok = match self.text[self.pos + 1] {
                        '=' => {
                            self.pos += 2;
                            Tok::Le
                        }
                        '<' => {
                            if self.text[self.pos + 2] == '=' {
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
                    return;
                }
                '!' => {
                    self.tok = match self.text[self.pos + 1] {
                        '=' => {
                            self.pos += 2;
                            Tok::Ne
                        }
                        _ => {
                            self.pos += 1;
                            Tok::Not
                        }
                    };
                    return;
                }
                '>' => {
                    self.tok = match self.text[self.pos + 1] {
                        '=' => {
                            self.pos += 2;
                            Tok::Ge
                        }
                        '>' => match self.text[self.pos + 2] {
                            '=' => {
                                self.pos += 3;
                                Tok::ShrAssign
                            }
                            '>' => {
                                if self.text[self.pos + 3] == '=' {
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
                    return;
                }
                _ => {
                    if c.is_whitespace() {
                        self.pos += 1;
                        continue;
                    }
                    if c.is_ascii_digit() {
                        self.lex_num();
                        return;
                    }
                    if is_id_part(c) {
                        let i = self.pos;

                        // Word
                        self.lex_id();
                        let s = substr(&self.text, i, self.pos);

                        // Keyword?
                        self.tok = match self.keywords.get(&s) {
                            Some(tok) => tok.clone(),
                            None => Tok::Atom(s),
                        };

                        return;
                    }
                    self.err("Unknown character");
                }
            }
        }
        self.tok = Tok::Eof;
    }

    fn eat(&mut self, tok: Tok) -> bool {
        if self.tok == tok {
            self.lex();
            return true;
        }
        false
    }

    fn require(&mut self, tok: Tok, s: &str) {
        if !self.eat(tok) {
            self.err(format!("Expected {}", s));
        }
    }

    fn atom(&mut self) -> String {
        if let Tok::Atom(s) = &self.tok {
            let s = s.clone();
            self.lex();
            return s;
        }
        self.err("Expected name");
    }

    fn id(&mut self) -> String {
        if let Tok::Atom(s) = &self.tok {
            let c = s.chars().nth(0).unwrap();
            if is_id_start(c) {
                let s = s.clone();
                self.lex();
                return s;
            }
        }
        self.err("Expected identifier");
    }

    fn str_literal(&mut self) -> String {
        if let Tok::Atom(s) = &self.tok {
            let c = s.chars().nth(0).unwrap();
            if c == '\'' || c == '"' {
                let s = s.clone();
                self.lex();
                return s;
            }
        }
        self.err("Expected string");
    }

    // Expressions
    fn comma_separated(&mut self, v: &mut Vec<Expr>, end: Tok) {
        if self.tok == end {
            return;
        }
        loop {
            v.push(self.expr());
            if !self.eat(Tok::Comma) {
                break;
            }
        }
    }

    fn primary(&mut self) -> Expr {
        match &self.tok {
            Tok::LSquare => {
                let mut v = Vec::<Expr>::new();
                self.lex();
                self.comma_separated(&mut v, Tok::RSquare);
                self.require(Tok::RSquare, "']'");
                Expr::List(v)
            }
            Tok::LBrace => {
                let mut v = Vec::<Expr>::new();
                self.lex();
                if self.tok != Tok::RBrace {
                    loop {
                        let k = self.atom();
                        v.push(Expr::Atom(k));
                        self.require(Tok::Colon, "':'");
                        v.push(self.expr());
                        if !self.eat(Tok::Comma) {
                            break;
                        }
                    }
                }
                self.require(Tok::RBrace, "'}'");
                Expr::Object(v)
            }
            Tok::LParen => {
                self.lex();
                let a = self.expr();
                self.require(Tok::RParen, "')'");
                a
            }
            Tok::Atom(s) => {
                let s = s.clone();
                self.lex();
                Expr::Atom(s)
            }
            _ => {
                self.err(format!("{:?}: Expected expression", self.tok));
            }
        }
    }

    fn postfix(&mut self) -> Expr {
        let mut a = self.primary();
        loop {
            a = match &self.tok {
                Tok::Dot => {
                    let a = Box::new(a);
                    self.lex();

                    let k = self.id();
                    let k = format!("{:?}", k);
                    let k = Expr::Atom(k);
                    let k = Box::new(k);

                    Expr::Subscript(a, k)
                }
                Tok::LSquare => {
                    let a = Box::new(a);
                    self.lex();

                    // First subscript
                    let i = match self.tok {
                        Tok::Colon => Expr::Atom("0".to_string()),
                        _ => self.expr(),
                    };
                    let i = Box::new(i);

                    // Second subscript?
                    let a = match self.tok {
                        Tok::RSquare => Expr::Subscript(a, i),
                        Tok::Colon => {
                            self.lex();

                            let j = match self.tok {
                                Tok::RSquare => Expr::Atom("undefined".to_string()),
                                _ => self.expr(),
                            };
                            let j = Box::new(j);

                            Expr::Slice(a, i, j)
                        }
                        _ => {
                            self.err(format!("{:?}: Expected ':' or ']'", self.tok));
                        }
                    };

                    self.require(Tok::RSquare, "']'");
                    a
                }
                Tok::LParen => {
                    let mut v = Vec::<Expr>::new();
                    self.lex();
                    self.comma_separated(&mut v, Tok::RParen);
                    self.require(Tok::RParen, "')'");
                    Expr::Call(Box::new(a), v)
                }
                _ => {
                    return a;
                }
            };
        }
    }

    fn prefix(&mut self) -> Expr {
        match &self.tok {
            Tok::Typeof => {
                self.lex();
                let a = self.prefix();
                Expr::Typeof(Box::new(a))
            }
            Tok::Not => {
                self.lex();
                let a = self.prefix();
                Expr::Prefix("!".to_string(), Box::new(a))
            }
            Tok::Minus => {
                self.lex();
                let a = self.prefix();
                Expr::Prefix("-".to_string(), Box::new(a))
            }
            Tok::BitNot => {
                self.lex();
                let a = self.prefix();
                Expr::Prefix("~".to_string(), Box::new(a))
            }
            _ => self.postfix(),
        }
    }

    fn infix(&mut self, prec: u8) -> Expr {
        // Operator precedence parser
        let mut a = self.prefix();
        loop {
            let o = match self.ops.get(&self.tok) {
                Some(o) => o.clone(),
                None => return a,
            };
            if o.prec < prec {
                return a;
            }
            self.lex();
            let b = self.infix(o.prec + o.left);
            a = if o.s == "=" {
                Expr::Assign(Box::new(a), Box::new(b))
            } else if o.compound_assign {
                Expr::Assign(
                    Box::new(a.clone()),
                    Box::new(Expr::Infix(o.s, Box::new(a), Box::new(b))),
                )
            } else {
                Expr::Infix(o.s, Box::new(a), Box::new(b))
            }
        }
    }

    fn expr(&mut self) -> Expr {
        self.infix(0)
    }

    // Statements
    fn block_end(&self) -> bool {
        matches!(
            self.tok,
            Tok::Pipe | Tok::Catch | Tok::Else | Tok::Elif | Tok::End | Tok::Eof
        )
    }

    fn stmt(&mut self, v: &mut Vec<Stmt>) {
        let src = self.src();
        let r = match self.tok {
            Tok::Func => {
                self.lex();

                // Name
                let name = self.id();

                // Parameters
                self.require(Tok::LParen, "'('");
                let mut params = Vec::<String>::new();
                if self.tok != Tok::RParen {
                    loop {
                        params.push(self.id());
                        if !self.eat(Tok::Comma) {
                            break;
                        }
                    }
                }
                self.require(Tok::RParen, "')'");
                self.require(Tok::Newline, "newline");

                // Outer variables
                let mut outers = HashSet::<String>::new();
                while self.eat(Tok::Outer) {
                    loop {
                        outers.insert(self.id());
                        if !self.eat(Tok::Comma) {
                            break;
                        }
                    }
                    self.require(Tok::Newline, "newline");
                }

                // Body
                let mut body = Vec::<Stmt>::new();
                self.block(&mut body);

                // End
                self.require(Tok::End, "'end'");
                Stmt::Func(src, name, params, outers, body)
            }
            Tok::For => {
                self.lex();
                let item = self.id();
                match self.tok {
                    Tok::Colon => {
                        self.lex();
                        let collection = self.expr();
                        self.require(Tok::Newline, "newline");

                        // Body
                        let mut body = Vec::<Stmt>::new();
                        self.block(&mut body);

                        // End
                        self.require(Tok::End, "'end'");
                        Stmt::For(src, item, collection, body)
                    }
                    Tok::Comma => {
                        let idx = item;
                        self.lex();
                        let item = self.id();
                        self.require(Tok::Colon, "':'");
                        let collection = self.expr();
                        self.require(Tok::Newline, "newline");

                        // Body
                        let mut body = Vec::<Stmt>::new();
                        self.block(&mut body);

                        // End
                        self.require(Tok::End, "'end'");
                        Stmt::For2(src, idx, item, collection, body)
                    }
                    _ => {
                        self.err(format!("{:?}: Expected ',' or ':'", self.tok));
                    }
                }
            }
            Tok::While => {
                self.lex();
                let cond = self.expr();
                self.require(Tok::Newline, "newline");

                // Body
                let mut body = Vec::<Stmt>::new();
                self.block(&mut body);

                // End
                self.require(Tok::End, "'end'");
                Stmt::While(src, cond, body)
            }
            Tok::Dowhile => {
                self.lex();
                let cond = self.expr();
                self.require(Tok::Newline, "newline");

                // Body
                let mut body = Vec::<Stmt>::new();
                self.block(&mut body);

                // End
                self.require(Tok::End, "'end'");
                Stmt::Dowhile(src, cond, body)
            }
            Tok::Assert => {
                self.lex();
                let cond = self.expr();
                let msg = if self.eat(Tok::Comma) {
                    format!("{:?}", self.str_literal())
                } else {
                    "".to_string()
                };
                Stmt::Assert(src, cond, msg)
            }
            Tok::If => {
                self.lex();
                let cond = self.expr();
                self.require(Tok::Newline, "newline");

                // Then
                let mut yes = Vec::<Stmt>::new();
                self.block(&mut yes);

                // Else
                let mut no = Vec::<Stmt>::new();
                if self.eat(Tok::Else) {
                    self.require(Tok::Newline, "newline");
                    self.block(&mut no);
                }

                // End
                self.require(Tok::End, "'end'");
                Stmt::If(src, cond, yes, no)
            }
            Tok::Case => {
                self.lex();
                let subject = self.expr();
                self.require(Tok::Newline, "newline");

                let mut cases = Vec::<(Vec<Expr>, Vec<Stmt>)>::new();
                while !self.eat(Tok::End) {
                    // Patterns
                    let mut patterns = Vec::<Expr>::new();
                    match self.tok {
                        Tok::Pipe => {
                            self.lex();
                            self.comma_separated(&mut patterns, Tok::Eof);
                        }
                        Tok::Else => {
                            self.lex();
                        }
                        _ => self.err(format!("{:?}: Expected '|' or 'else'", self.tok)),
                    }
                    self.require(Tok::Newline, "newline");

                    // Body
                    let mut body = Vec::<Stmt>::new();
                    self.block(&mut body);

                    // Case
                    cases.push((patterns, body));
                }
                Stmt::Case(src, subject, cases)
            }
            Tok::Try => {
                self.lex();
                self.require(Tok::Newline, "newline");

                // Normal path
                let mut normal = Vec::<Stmt>::new();
                self.block(&mut normal);

                // Fallback path
                let mut fallback = Vec::<Stmt>::new();
                self.require(Tok::Catch, "'catch'");
                let name = self.id();
                self.require(Tok::Newline, "newline");
                self.block(&mut fallback);

                // End
                self.require(Tok::End, "'end'");
                Stmt::Try(src, normal, name, fallback)
            }
            Tok::Return => {
                self.lex();
                let a = if self.tok == Tok::Newline {
                    Expr::Atom("null".to_string())
                } else {
                    self.expr()
                };
                Stmt::Return(src, a)
            }
            Tok::Throw => {
                self.lex();
                let a = self.expr();
                Stmt::Throw(src, a)
            }
            Tok::Prin => {
                self.lex();
                let mut w = Vec::<Expr>::new();
                self.comma_separated(&mut w, Tok::Newline);
                for a in w {
                    v.push(Stmt::Prin(src.clone(), a));
                }
                return;
            }
            Tok::Print => {
                self.lex();
                let mut w = Vec::<Expr>::new();
                self.comma_separated(&mut w, Tok::Newline);
                for a in w {
                    v.push(Stmt::Prin(src.clone(), a));
                }
                v.push(Stmt::Prin(src, Expr::Atom("'\\n'".to_string())));
                return;
            }
            Tok::EPrint => {
                self.lex();
                let mut w = Vec::<Expr>::new();
                self.comma_separated(&mut w, Tok::Newline);
                for a in w {
                    v.push(Stmt::EPrin(src.clone(), a));
                }
                v.push(Stmt::EPrin(src, Expr::Atom("'\\n'".to_string())));
                return;
            }
            Tok::EPrin => {
                self.lex();
                let mut w = Vec::<Expr>::new();
                self.comma_separated(&mut w, Tok::Newline);
                for a in w {
                    v.push(Stmt::EPrin(src.clone(), a));
                }
                return;
            }
            _ => {
                let a = self.expr();
                if self.tok == Tok::Colon {
                    if let Expr::Atom(s) = a {
                        self.lex();
                        v.push(Stmt::Label(src, s));
                        return;
                    }
                }
                Stmt::Expr(src, a)
            }
        };
        v.push(r);
    }

    fn block(&mut self, v: &mut Vec<Stmt>) {
        while !self.block_end() {
            self.stmt(v);
            self.require(Tok::Newline, "newline");
        }
    }

    fn parse(&mut self) -> Vec<Stmt> {
        // Start the tokenizer
        self.lex();
        self.eat(Tok::Newline);

        // Parse
        let mut v = Vec::<Stmt>::new();
        self.block(&mut v);

        // Check for extra 'end' etc
        if self.tok != Tok::Eof {
            self.err("Unmatched terminator");
        }

        v
    }
}

pub fn parse(file: &str) -> Vec<Stmt> {
    let text = match fs::read_to_string(file) {
        Ok(a) => a,
        Err(e) => {
            // A parser library would need to return an error result
            // As this is a program rather than a library, we can promptly exit
            eprintln!("{}: {}", file, e);
            process::exit(1);
        }
    };
    let text: Vec<char> = text.chars().collect();
    let mut parser = Parser::new(file.to_string(), text);
    parser.parse()
}
