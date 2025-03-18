use crate::ast::*;
use crate::compile_error::*;
use std::collections::HashMap;
use std::mem;

// TODO: CamelCase consistency
#[derive(Clone, Hash, PartialEq, Eq)]
enum Tok {
    Dim,
    While,
    Wend,
    Endfor,
    Endwhile,
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
    Next,
    Gosub,
    Return,
    Goto,
    Comma,
    Then,
    Else,
    End,
    Endif,
    Star,
    Caret,
    Plus,
    Tilde,
    To,
    Minus,
    Slash,
    And,
    Or,
    Not,
    Lt,
    Le,
    Gt,
    Ge,
    Step,
    Eq,
    Ne,
    Shr,
    Div,
    Shl,
    Print,
    Mod,
    Let,
    If,
}

// The operator precedence parser uses a table of these
#[derive(Clone)]
struct Op {
    prec: u8,
    left: u8,
}

struct Parser {
    // There is a compile-time perfect hash package
    // but there are benchmarks showing HashMap to be faster
    keywords: HashMap<String, Tok>,

    // Table of infix operators
    ops: HashMap<Tok, Op>,

    // Name of the input file
    // or suitable descriptor, if the input did not come from a file
    file: String,

    // Decode the entire input text upfront
    // to make sure there are no situations in which decoding work is repeated
    text: Vec<char>,

    // This is where the caret will point to in case of error
    // Most of the time, it points to the start of current token
    start: usize,

    // Current position in the input text
    // Mostly tracked and used by the tokenizer
    // Most of the time, it points just after the current token
    pos: usize,

    // Current token
    tok: Tok,

    // Output
    code: Vec<Stmt>,
}

fn is_id_part(c: char) -> bool {
    c.is_alphanumeric() || c == '_' || c == '$' || c == '%'
}

fn substr(text: &[char], i: usize, j: usize) -> String {
    text.iter().skip(i).take(j - i).collect()
}

impl Parser {
    fn new(file: &str, text: &str) -> Self {
        // Keywords
        let mut keywords = HashMap::new();
        keywords.insert("dim".to_string(), Tok::Dim);
        keywords.insert("assert".to_string(), Tok::Assert);
        keywords.insert("mod".to_string(), Tok::Mod);
        keywords.insert("let".to_string(), Tok::Let);
        keywords.insert("if".to_string(), Tok::If);
        keywords.insert("print".to_string(), Tok::Print);
        keywords.insert("div".to_string(), Tok::Div);
        keywords.insert("and".to_string(), Tok::And);
        keywords.insert("input".to_string(), Tok::Input);
        keywords.insert("or".to_string(), Tok::Or);
        keywords.insert("then".to_string(), Tok::Then);
        keywords.insert("not".to_string(), Tok::Not);
        keywords.insert("else".to_string(), Tok::Else);
        keywords.insert("endif".to_string(), Tok::Endif);
        keywords.insert("end".to_string(), Tok::End);
        keywords.insert("gosub".to_string(), Tok::Gosub);
        keywords.insert("return".to_string(), Tok::Return);
        keywords.insert("goto".to_string(), Tok::Goto);
        keywords.insert("to".to_string(), Tok::To);
        keywords.insert("for".to_string(), Tok::For);
        keywords.insert("next".to_string(), Tok::Next);
        keywords.insert("step".to_string(), Tok::Step);
        keywords.insert("while".to_string(), Tok::While);
        keywords.insert("wend".to_string(), Tok::Wend);
        keywords.insert("endfor".to_string(), Tok::Endfor);
        keywords.insert("endwhile".to_string(), Tok::Endwhile);

        // Infix operators
        let mut ops = HashMap::new();
        let mut add = |o: Tok, prec: u8, left: u8| {
            ops.insert(o, Op { prec, left });
        };

        let mut prec = 99u8;
        add(Tok::Caret, prec, 0);

        prec -= 1;
        add(Tok::Star, prec, 1);
        add(Tok::Slash, prec, 1);
        add(Tok::Div, prec, 1);
        add(Tok::Mod, prec, 1);

        prec -= 1;
        add(Tok::Plus, prec, 1);
        add(Tok::Minus, prec, 1);

        prec -= 1;
        add(Tok::Shl, prec, 1);
        add(Tok::Shr, prec, 1);

        prec -= 1;
        add(Tok::Eq, prec, 1);
        add(Tok::Ne, prec, 1);
        add(Tok::Lt, prec, 1);
        add(Tok::Gt, prec, 1);
        add(Tok::Le, prec, 1);
        add(Tok::Ge, prec, 1);

        // Decode text
        let mut chars: Vec<char> = text.chars().collect();
        if !text.ends_with('\n') {
            chars.push('\n');
        }

        // Parser object
        Parser {
            keywords,
            ops,
            file: file.to_string(),
            text: chars,
            start: 0,
            pos: 0,
            tok: Tok::Newline,
            code: Vec::new(),
        }
    }

    fn err<S: AsRef<str>>(&mut self, msg: S) -> CompileError {
        CompileError::new(
            mem::take(&mut self.file),
            &self.text,
            self.start,
            msg.as_ref().to_string(),
        )
    }

    // Tokenizer
    fn digits(&mut self) {
        let mut i = self.pos;
        while self.text[i].is_ascii_digit() || self.text[i] == '_' {
            i += 1;
        }
        self.pos = i
    }

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

    fn eol(&mut self) {
        let mut i = self.pos;
        while self.text[i] != '\n' {
            i += 1;
        }
        self.pos = i;
    }

    fn lex(&mut self) -> Result<(), CompileError> {
        while self.pos < self.text.len() {
            self.start = self.pos;
            let c = self.text[self.pos];
            match c {
                '"' => {
                    let mut i = self.pos + 1;
                    while self.text[i] != '"' {
                        match self.text[i] {
                            '\n' => {
                                return Err(self.err("Unterminated string"));
                            }
                            '\\' => match self.text[i] {
                                '\\' | '"' => {
                                    i += 1;
                                }
                                _ => {}
                            },
                            _ => {
                                i += 1;
                            }
                        }
                    }
                    self.tok = Tok::Str(substr(&self.text, self.pos, i));
                    self.pos = i + 1;
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
                '~' => {
                    self.pos += 1;
                    self.tok = Tok::Tilde;
                    return Ok(());
                }
                '^' => {
                    self.pos += 1;
                    self.tok = Tok::Caret;
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
                '\\' => {
                    self.pos += 1;
                    self.tok = Tok::Div;
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
                    if self.text[self.pos] == '=' {
                        self.pos += 1;
                    }
                    self.tok = Tok::Eq;
                    return Ok(());
                }
                '\n' => {
                    self.pos += 1;
                    self.tok = Tok::Newline;
                    return Ok(());
                }
                ' ' | '\t' | '\r' | '\x0c' => {
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
                        _ => {
                            self.start = self.pos + 1;
                            return Err(self.err("Expected '='"));
                        }
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
                _ => {
                    if c.is_alphabetic() || c == '_' {
                        let i = self.pos;

                        // Word
                        self.lex_id();
                        let s = substr(&self.text, i, self.pos).to_lowercase();

                        // Comment
                        if s == "rem" {
                            self.eol();
                            continue;
                        }

                        // Keyword?
                        self.tok = match self.keywords.get(&s) {
                            Some(tok) => tok.clone(),
                            None => Tok::Id(s),
                        };

                        return Ok(());
                    }
                    if c.is_ascii_digit() {
                        let i = self.pos;

                        // Alternative radix
                        if c == '0' {
                            match self.text[i + 1] {
                                'x' | 'X' | 'b' | 'B' | 'o' | 'O' => {
                                    self.lex_id();
                                    let s = substr(&self.text, i, self.pos);
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
                        if self.text[self.pos] == '.' {
                            self.pos += 1;
                            self.digits();
                            is_float = true;
                        }

                        // Exponent
                        match self.text[self.pos] {
                            'e' | 'E' => {
                                self.pos += 1;
                                match self.text[self.pos] {
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
                        let s = substr(&self.text, i, self.pos);
                        self.tok = if is_float { Tok::Float(s) } else { Tok::Int(s) };

                        return Ok(());
                    }
                    return Err(self.err("Unknown character"));
                }
            }
        }
        self.tok = Tok::Eof;
        Ok(())
    }

    fn require(&mut self, tok: Tok, s: &str) -> Result<(), CompileError> {
        if self.tok != tok {
            return Err(self.err(format!("Expected {}", s)));
        }
        self.lex()?;
        Ok(())
    }

    // Expressions
    fn primary(&mut self) -> Result<Expr, CompileError> {
        match &self.tok {
            Tok::LSquare => {
                self.lex()?;
                let mut v = Vec::<Expr>::new();
                if self.tok != Tok::RSquare {
                    loop {
                        v.push(self.expr()?);
                        if self.tok != Tok::Comma {
                            break;
                        }
                        self.lex()?;
                    }
                }
                self.require(Tok::RSquare, "']'")?;
                Ok(Expr::List(v))
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
                Ok(Expr::Id(s))
            }
            Tok::Int(s) => {
                let s = s.clone();
                self.lex()?;
                Ok(Expr::Int(s))
            }
            Tok::Float(s) => {
                let s = s.clone();
                self.lex()?;
                Ok(Expr::Float(s))
            }
            Tok::Str(s) => {
                let s = s.clone();
                self.lex()?;
                Ok(Expr::Str(s))
            }
            _ => Err(self.err("Expected expression")),
        }
    }

    fn expr(&mut self) -> Result<Expr, CompileError> {
        self.primary()
    }
}
