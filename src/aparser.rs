use crate::ast::*;
use crate::compile_error::*;
use std::collections::HashMap;

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

struct Parser<'a> {
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
    text: &'a Vec<char>,

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

impl<'a> Parser<'a> {
    fn new(file: &str, text: &'a Vec<char>) -> Self {
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

        // Parser object
        Parser {
            keywords,
            ops,
            file,
            text,
            start: 0,
            pos: 0,
            tok: Tok::Newline,
            code: Vec::<Inst>::new(),
        }
    }
}
