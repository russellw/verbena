use crate::ast::*;
use std::collections::HashMap;

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
enum Tok {
    Dowhile,
    While,
    Func,
    Atom(String),
    Colon,
    Newline,
    Nonlocal,
    Global,
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
    Null,
    IDiv,
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
    assign: bool,
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

    // Line number tracker for error reporting
    line: usize,

    // Current token
    tok: Tok,
}

fn is_id_part(c: char) -> bool {
    c.is_alphanumeric() || c == '_' || c == '$' || c == '?'
}

fn substr(buf: &[char], i: usize, j: usize) -> String {
    buf.iter().skip(i).take(j - i).collect()
}

impl Parser {
    fn new(file: String, text: Vec<char>) -> Self {
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
        keywords.insert("for".to_string(), Tok::For);
        keywords.insert("while".to_string(), Tok::While);
        keywords.insert("global".to_string(), Tok::Global);
        keywords.insert("nonlocal".to_string(), Tok::Nonlocal);
        keywords.insert("dowhile".to_string(), Tok::Dowhile);

        // Infix operators
        let mut ops = HashMap::new();
        let mut op = |tok: Tok, prec: u8, left: u8, s: &str, assign: bool| {
            ops.insert(
                tok,
                Op {
                    prec,
                    left,
                    s: s.to_string(),
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
            file,
            text,
            pos: 0,
            line: 0,
            tok: Tok::Newline,
        }
    }

    fn error_context(&self) -> Source {
        Source::new(Rc::clone(&self.file), self.line)
    }

    fn error<S: AsRef<str>>(&mut self, msg: S) -> String {
        format!("{}: {}", self.error_context(), msg.as_ref().to_string())
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

    fn quote(&mut self) {
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
                            let c = self.hex_to_char(i, i + 2);
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
                            let c = self.hex_to_char(i, j);
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
    }

    fn lex(&mut self) {
        while self.pos < self.buf.len() {
            self.start = self.pos;
            let c = self.buf[self.pos];
            match c {
                '"' | '\'' => {
                    self.quote();
                    return;
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
                    return;
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
                    return;
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
                    return;
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
                    return;
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
                    return;
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
                    return;
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
                    return;
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
                    // TODO: .digit
                    self.pos += 1;
                    self.tok = Tok::Dot;
                    return;
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
                    return;
                }
                '\n' => {
                    self.tok = Tok::Newline;
                    self.read();
                    self.pos = 0;
                    return;
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
                    return;
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
                    return;
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
                    return;
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
                                    return;
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

                        return;
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

                        return;
                    }
                    return Err(self.error("Unknown character"));
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
            return Err(self.error(format!("Expected {}", s)));
        }
    }

    fn id(&mut self) -> String {
        let s = match &self.tok {
            Tok::Id(s) => s.clone(),
            _ => return Err(self.error("Expected name")),
        };
        self.lex();
        s
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
        let ec = self.error_context();
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
                        let k = self.id();
                        v.push(Expr::Str(k));
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
            Tok::Id(s) => {
                let s = s.clone();
                self.lex();
                Expr::Id(ec, s)
            }
            Tok::Str(s) => {
                let s = s.clone();
                self.lex();
                Expr::Str(s)
            }
            _ => return Err(self.error(format!("{:?}: Expected expression", self.tok))),
        }
    }

    fn postfix(&mut self) -> Expr {
        let mut a = self.primary();
        loop {
            a = match &self.tok {
                Tok::Dot => {
                    let ec = self.error_context();
                    let a = Box::new(a);
                    self.lex();

                    let k = self.id();
                    let k = Expr::Str(k);
                    let k = Box::new(k);

                    Expr::Subscript(ec, a, k)
                }
                Tok::LSquare => {
                    let ec = self.error_context();
                    let a = Box::new(a);
                    self.lex();

                    let i = match self.tok {
                        Tok::Colon => Expr::Num(0.0),
                        _ => self.expr(),
                    };
                    let i = Box::new(i);

                    let a = match self.tok {
                        Tok::RSquare => Expr::Subscript(ec, a, i),
                        Tok::Colon => {
                            self.lex();

                            let j = match self.tok {
                                Tok::RSquare => Expr::Null,
                                _ => self.expr(),
                            };
                            let j = Box::new(j);

                            Expr::Slice(ec, a, i, j)
                        }
                        _ => return Err(self.error(format!("{:?}: Expected ':' or ']'", self.tok))),
                    };

                    self.require(Tok::RSquare, "']'");
                    a
                }
                Tok::LParen => {
                    let ec = self.error_context();
                    let mut v = Vec::<Expr>::new();
                    self.lex();
                    self.comma_separated(&mut v, Tok::RParen);
                    self.require(Tok::RParen, "')'");
                    Expr::Call(ec, Box::new(a), v)
                }
                _ => {
                    return a;
                }
            };
        }
    }

    fn prefix(&mut self) -> Expr {
        match &self.tok {
            Tok::Not => {
                self.lex();
                let a = self.prefix();
                Expr::Prefix(Source::blank(), Inst::Not, Box::new(a))
            }
            Tok::Sub => {
                let ec = self.error_context();
                self.lex();
                let a = self.prefix();
                Expr::Prefix(ec, Inst::Neg, Box::new(a))
            }
            Tok::BitNot => {
                let ec = self.error_context();
                self.lex();
                let a = self.prefix();
                Expr::Prefix(ec, Inst::BitNot, Box::new(a))
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
            let ec = self.error_context();
            let tok = self.tok.clone();
            self.lex();
            let b = self.infix(o.prec + o.left);
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

    fn expr(&mut self) -> Expr {
        self.infix(0)
    }

    // Statements
    fn block_end(&self) -> bool {
        matches!(self.tok, Tok::Else | Tok::End | Tok::Eof)
    }

    fn stmt(&mut self, v: &mut Vec<Stmt>) {
        let ec = self.error_context();
        let r = match self.tok {
            Tok::Func => {
                self.lex();
                let name = self.id();
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

                let mut body = Vec::<Stmt>::new();
                self.block(&mut body);

                self.require(Tok::End, "'end'");
                Stmt::Func(name, params, body)
            }
            Tok::For => {
                self.lex();
                let name = self.id();
                self.require(Tok::Colon, "':'");
                let collection = self.expr();
                self.require(Tok::Newline, "newline");

                let mut body = Vec::<Stmt>::new();
                self.block(&mut body);

                self.require(Tok::End, "'end'");
                Stmt::For(name, collection, body)
            }
            Tok::While => {
                self.lex();
                let cond = self.expr();
                self.require(Tok::Newline, "newline");

                let mut body = Vec::<Stmt>::new();
                self.block(&mut body);

                self.require(Tok::End, "'end'");
                Stmt::While(cond, body)
            }
            Tok::Dowhile => {
                self.lex();
                let cond = self.expr();
                self.require(Tok::Newline, "newline");

                let mut body = Vec::<Stmt>::new();
                self.block(&mut body);

                self.require(Tok::End, "'end'");
                Stmt::Dowhile(cond, body)
            }
            Tok::Assert => {
                self.lex();
                let cond = self.expr();
                let msg = if self.eat(Tok::Comma) {
                    match &self.tok {
                        Tok::Str(s) => {
                            let s = s.to_string();
                            self.lex();
                            s
                        }
                        _ => return Err(self.error("Expected string")),
                    }
                } else {
                    "Assert failed".to_string()
                };
                let msg = format!("{}: {}", ec, msg);
                Stmt::Assert(ec, cond, msg)
            }
            Tok::If => {
                self.lex();
                let cond = self.expr();
                self.require(Tok::Newline, "newline");

                let mut yes = Vec::<Stmt>::new();
                self.block(&mut yes);

                let mut no = Vec::<Stmt>::new();
                if self.eat(Tok::Else) {
                    self.require(Tok::Newline, "newline");
                    self.block(&mut no);
                }

                self.require(Tok::End, "'end'");
                Stmt::If(cond, yes, no)
            }
            Tok::Global => {
                let ec = self.error_context();
                self.lex();
                loop {
                    let name = self.id();
                    v.push(Stmt::Global(ec.clone(), name));
                    if !self.eat(Tok::Comma) {
                        break;
                    }
                }
                return;
            }
            Tok::Nonlocal => {
                let ec = self.error_context();
                self.lex();
                loop {
                    let name = self.id();
                    v.push(Stmt::Nonlocal(ec.clone(), name));
                    if !self.eat(Tok::Comma) {
                        break;
                    }
                }
                return;
            }
            Tok::Return => {
                self.lex();
                let a = if self.tok == Tok::Newline {
                    Expr::Null
                } else {
                    self.expr()
                };
                Stmt::Return(a)
            }
            Tok::Prin => {
                self.lex();
                let mut w = Vec::<Expr>::new();
                self.comma_separated(&mut w, Tok::Newline);
                for a in w {
                    v.push(Stmt::Prin(a));
                }
                return;
            }
            Tok::Newline => return,
            Tok::Print => {
                self.lex();
                let mut w = Vec::<Expr>::new();
                self.comma_separated(&mut w, Tok::Newline);
                for a in w {
                    v.push(Stmt::Prin(a));
                }
                v.push(Stmt::Prin(Expr::Str("\n".to_string())));
                return;
            }
            _ => {
                let a = self.expr();
                if self.tok == Tok::Colon {
                    if let Expr::Id(ec, s) = a {
                        self.lex();
                        v.push(Stmt::Label(ec, s));
                        return;
                    }
                }
                Stmt::Expr(a)
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

    fn parse(&mut self) -> Result<Vec<Stmt>, String> {
        // Start the tokenizer
        self.read();
        self.lex();

        // Parse
        let mut v = Vec::<Stmt>::new();
        self.block(&mut v);

        // Check for extra 'end' etc
        if self.tok != Tok::Eof {
            return Err(self.error("Unmatched terminator"));
        }

        v
    }
}

pub fn parse(file: String) -> Vec<Stmt> {
    // Read the file contents into a string
    let contents = match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", filename, e);
            process::exit(1);
        }
    };

    // Convert the string to a vector of chars
    let chars: Vec<char> = contents.chars().collect();

    let mut parser = Parser::new(file, chars);
    parser.parse()
}
