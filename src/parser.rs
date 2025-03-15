use crate::vm::*;
use num_bigint::BigInt;
use num_traits::{One, Zero};
use std::collections::HashMap;
use std::fmt;
use std::mem;

#[derive(Clone, Hash, PartialEq, Eq)]
enum Tok {
    While,
    Wend,
    Endfor,
    Endwhile,
    Int(BigInt),
    Float(String),
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
    For,
    Next,
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
    True,
    False,
    BitXor,
    BitAnd,
    And,
    BitOr,
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
    ToInt,
    Div,
    Sqrt,
    Shl,
    Print,
    Mod,
    Let,
    If,
}

impl fmt::Display for Tok {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Tok::Int(a) => write!(f, "{}", a),
            Tok::Float(s) => write!(f, "{}", s),
            Tok::Id(s) => write!(f, "{}", s),
            _ => panic!(),
        }
    }
}

// The operator precedence parser uses a table of these
#[derive(Clone)]
struct Op {
    prec: u8,
    left: u8,
    inst: Inst,
}

// GOTO and GOSUB can go forward as well as back
// That means they can only be resolved at the end, when all labels have been seen
// so need to keep track of them until then
struct LabelRef {
    // Index in the code vector
    i: usize,

    // Line number or label referred to
    label: Tok,
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

    // Table of infix operators
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

    // Counter for generating temporary names
    tmp_count: usize,

    labels: HashMap<Tok, usize>,
    label_refs: Vec<LabelRef>,

    code: Vec<Inst>,
}

fn current_line(chars: &[char], caret: usize) -> (usize, usize) {
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

fn substr(chars: &[char], i: usize, j: usize) -> String {
    chars.iter().skip(i).take(j - i).collect()
}

impl Parser {
    fn new(chars: Vec<char>) -> Self {
        // Keywords
        let mut keywords = HashMap::new();
        keywords.insert("int".to_string(), Tok::ToInt);
        keywords.insert("mod".to_string(), Tok::Mod);
        keywords.insert("let".to_string(), Tok::Let);
        keywords.insert("if".to_string(), Tok::If);
        keywords.insert("print".to_string(), Tok::Print);
        keywords.insert("true".to_string(), Tok::True);
        keywords.insert("false".to_string(), Tok::False);
        keywords.insert("div".to_string(), Tok::Div);
        keywords.insert("bitxor".to_string(), Tok::BitXor);
        keywords.insert("and".to_string(), Tok::And);
        keywords.insert("or".to_string(), Tok::Or);
        keywords.insert("bitand".to_string(), Tok::BitAnd);
        keywords.insert("bitor".to_string(), Tok::BitOr);
        keywords.insert("then".to_string(), Tok::Then);
        keywords.insert("not".to_string(), Tok::Not);
        keywords.insert("else".to_string(), Tok::Else);
        keywords.insert("endif".to_string(), Tok::Endif);
        keywords.insert("end".to_string(), Tok::End);
        keywords.insert("goto".to_string(), Tok::Goto);
        keywords.insert("to".to_string(), Tok::To);
        keywords.insert("for".to_string(), Tok::For);
        keywords.insert("next".to_string(), Tok::Next);
        keywords.insert("step".to_string(), Tok::Step);
        keywords.insert("while".to_string(), Tok::While);
        keywords.insert("wend".to_string(), Tok::Wend);
        keywords.insert("endfor".to_string(), Tok::Endfor);
        keywords.insert("endwhile".to_string(), Tok::Endwhile);
        keywords.insert("sqr".to_string(), Tok::Sqrt);
        keywords.insert("sqrt".to_string(), Tok::Sqrt);

        // Infix operators
        let mut ops = HashMap::new();
        let mut add = |o: Tok, prec: u8, left: u8, inst: Inst| {
            ops.insert(o, Op { prec, left, inst });
        };

        let mut prec = 99u8;
        add(Tok::Caret, prec, 0, Inst::Pow);

        prec -= 1;
        add(Tok::Star, prec, 1, Inst::Mul);
        add(Tok::Slash, prec, 1, Inst::FDiv);
        add(Tok::Div, prec, 1, Inst::IDiv);
        add(Tok::Mod, prec, 1, Inst::Mod);

        prec -= 1;
        add(Tok::Plus, prec, 1, Inst::Add);
        add(Tok::Minus, prec, 1, Inst::Sub);

        prec -= 1;
        add(Tok::Shl, prec, 1, Inst::Shl);
        add(Tok::Shr, prec, 1, Inst::Shr);

        prec -= 1;
        add(Tok::Eq, prec, 1, Inst::Eq);
        add(Tok::Ne, prec, 1, Inst::Ne);
        add(Tok::Lt, prec, 1, Inst::Lt);
        add(Tok::Gt, prec, 1, Inst::Gt);
        add(Tok::Le, prec, 1, Inst::Le);
        add(Tok::Ge, prec, 1, Inst::Ge);

        // Parser object
        Parser {
            keywords,
            ops,
            chars,
            caret: 0,
            pos: 0,
            line: 1,
            tok: Tok::Newline,
            tmp_count: 0,
            labels: HashMap::<Tok, usize>::new(),
            label_refs: Vec::<LabelRef>::new(),
            code: Vec::<Inst>::new(),
        }
    }

    fn tmp_name(&mut self) -> String {
        let i = self.tmp_count;
        self.tmp_count = i + 1;
        format!("__{}", i)
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

    // Tokenizer
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

    fn lex_int(&mut self, radix: u32) -> Result<(), ParseError> {
        let mut i = self.pos + 2;
        let mut v = Vec::<char>::new();
        while self.chars[i].is_digit(radix) || self.chars[i] == '_' {
            if self.chars[i] != '_' {
                v.push(self.chars[i])
            }
            i += 1;
        }
        self.pos = i;
        let s: String = v.into_iter().collect();
        let a = match BigInt::parse_bytes(s.as_bytes(), radix) {
            Some(a) => a,
            None => return Err(self.err("Expected integer".to_string())),
        };
        self.tok = Tok::Int(a);
        Ok(())
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
                                        while self.chars[j].is_ascii_hexdigit() {
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
                    if c.is_ascii_digit() {
                        // Alternative radix
                        if c == '0' {
                            match self.chars[self.pos + 1] {
                                'x' | 'X' => return self.lex_int(16),
                                'b' | 'B' => return self.lex_int(2),
                                'o' | 'O' => return self.lex_int(8),
                                _ => {}
                            }
                        }

                        let mut i = self.pos;
                        let mut v = Vec::<char>::new();

                        // Decimal, integer part
                        loop {
                            let c = self.chars[i];
                            if c != '_' {
                                v.push(c);
                            }
                            i += 1;
                            if !(self.chars[i].is_ascii_digit() || self.chars[i] == '_') {
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
                                if !(self.chars[i].is_ascii_digit() || self.chars[i] == '_') {
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
                                while self.chars[i].is_ascii_digit() || self.chars[i] == '_' {
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

                        // Int
                        match s.parse::<BigInt>() {
                            Ok(a) => {
                                self.tok = Tok::Int(a);
                                return Ok(());
                            }
                            Err(_) => {}
                        };

                        // Float
                        self.tok = Tok::Float(s);
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

    // Expressions
    fn primary(&mut self) -> Result<(), ParseError> {
        match &self.tok {
            Tok::Sqrt => {
                self.lex()?;
                self.primary()?;
                self.code.push(Inst::Sqrt);
            }
            Tok::ToInt => {
                self.lex()?;
                self.primary()?;
                self.code.push(Inst::Floor);
            }
            Tok::Id(name) => {
                self.code.push(Inst::Load(name.to_string()));
                self.lex()?;
            }
            Tok::Str(s) => {
                self.code.push(Inst::Const(Val::string(s)));
                self.lex()?;
            }
            Tok::Int(a) => {
                self.code.push(Inst::Const(Val::Int(a.clone())));
                self.lex()?;
            }
            Tok::Float(s) => match s.parse::<f64>() {
                Ok(a) => {
                    self.code.push(Inst::Const(Val::Float(a)));
                    self.lex()?;
                }
                Err(e) => return Err(self.err(e.to_string())),
            },
            Tok::False => {
                self.code.push(Inst::Const(Val::Int(BigInt::zero())));
                self.lex()?;
            }
            Tok::LParen => {
                self.lex()?;
                self.expr()?;
                self.require(Tok::RParen, "')'")?;
            }
            Tok::True => {
                self.code.push(Inst::Const(Val::Int(BigInt::one())));
                self.lex()?;
            }
            _ => return Err(self.err("Expected expression")),
        }
        Ok(())
    }

    fn prefix(&mut self) -> Result<(), ParseError> {
        match &self.tok {
            Tok::Minus => {
                self.lex()?;
                self.prefix()?;
                self.code.push(Inst::Neg);
            }
            Tok::Not => {
                self.lex()?;
                self.prefix()?;
                self.code.push(Inst::Not);
            }
            Tok::Tilde => {
                self.lex()?;
                self.prefix()?;
                self.code.push(Inst::BitNot);
            }
            _ => {
                self.primary()?;
            }
        }
        Ok(())
    }

    fn infix(&mut self, prec: u8) -> Result<(), ParseError> {
        // Operator precedence parser
        self.prefix()?;
        loop {
            let tok = self.tok.clone();
            let o = match self.ops.get(&tok) {
                Some(o) => o.clone(),
                None => return Ok(()),
            };
            if o.prec < prec {
                return Ok(());
            }
            self.lex()?;
            self.infix(o.prec + o.left)?;
            self.code.push(o.inst);
        }
    }

    fn expr(&mut self) -> Result<(), ParseError> {
        self.infix(0)
    }

    // Statements
    fn is_end(&self) -> bool {
        matches!(
            self.tok,
            Tok::Else
                | Tok::End
                | Tok::Endif
                | Tok::Eof
                | Tok::Wend
                | Tok::Endwhile
                | Tok::Endfor
                | Tok::Next
        )
    }

    fn vertical_if(&mut self) -> Result<(), ParseError> {
        // Condition
        let to_else = self.code.len();
        self.code.push(Inst::BrFalse(0));

        // If true
        self.vertical_stmts()?;

        if self.tok == Tok::Else {
            self.lex()?;
            let to_after = self.code.len();
            self.code.push(Inst::Br(0));

            // Resolve branch targets
            self.code[to_else] = Inst::BrFalse(self.code.len());

            // If false
            self.vertical_stmts()?;

            // Resolve branch targets
            self.code[to_after] = Inst::Br(self.code.len());
        } else {
            // Resolve branch targets
            self.code[to_else] = Inst::BrFalse(self.code.len());
        }

        // End
        match self.tok {
            Tok::End => {
                self.lex()?;
                if self.tok == Tok::If {
                    self.lex()?;
                }
            }
            Tok::Endif => {
                self.lex()?;
            }
            _ => return Err(self.err("Expected END")),
        }
        Ok(())
    }

    fn stmt(&mut self) -> Result<(), ParseError> {
        let tok = self.tok.clone();
        match tok {
            Tok::For => {
                self.lex()?;

                // Counter
                let counter_name = match &self.tok {
                    Tok::Id(name) => name.clone(),
                    _ => return Err(self.err("Expected variable name")),
                };
                self.lex()?;

                self.require(Tok::Eq, "'='")?;

                // Initial value
                self.expr()?;
                self.code.push(Inst::Store(counter_name.clone()));

                self.require(Tok::To, "TO")?;

                // Final value
                self.expr()?;
                let final_name = self.tmp_name();
                self.code.push(Inst::Store(final_name.clone()));

                let loop_target: usize;
                let to_after: usize;

                if self.tok == Tok::Step {
                    // Step
                    let step_name = self.tmp_name();
                    self.lex()?;
                    let step_down = self.tok == Tok::Minus;
                    self.expr()?;
                    self.code.push(Inst::Store(step_name.clone()));

                    // Condition
                    loop_target = self.code.len();
                    self.code.push(Inst::Load(counter_name.clone()));
                    self.code.push(Inst::Load(final_name));
                    self.code.push(if step_down { Inst::Ge } else { Inst::Le });
                    to_after = self.code.len();
                    self.code.push(Inst::BrFalse(0));

                    self.require(Tok::Newline, "newline")?;

                    // Body
                    self.vertical_stmts()?;

                    // Increment
                    self.code.push(Inst::Load(counter_name.clone()));
                    self.code.push(Inst::Load(step_name.clone()));
                } else {
                    // Condition
                    loop_target = self.code.len();
                    self.code.push(Inst::Load(counter_name.clone()));
                    self.code.push(Inst::Load(final_name));
                    self.code.push(Inst::Le);
                    to_after = self.code.len();
                    self.code.push(Inst::BrFalse(0));

                    self.require(Tok::Newline, "newline")?;

                    // Body
                    self.vertical_stmts()?;

                    // Increment
                    self.code.push(Inst::Load(counter_name.clone()));
                    self.code.push(Inst::Const(Val::Int(BigInt::one())));
                }

                // Increment
                self.code.push(Inst::Add);
                self.code.push(Inst::Store(counter_name.clone()));

                // Loop
                self.code.push(Inst::Br(loop_target));

                // Resolve branch targets
                self.code[to_after] = Inst::BrFalse(self.code.len());

                // End
                match self.tok {
                    Tok::End => {
                        self.lex()?;
                        if self.tok == Tok::For {
                            self.lex()?;
                        }
                    }
                    Tok::Next => {
                        self.lex()?;
                        if let Tok::Id(name) = &self.tok {
                            if *name != counter_name {
                                return Err(self.err(format!(
                                    "FOR {} does not match NEXT {}",
                                    counter_name, name
                                )));
                            }
                            self.lex()?;
                        }
                    }
                    Tok::Endfor => {
                        self.lex()?;
                    }
                    _ => return Err(self.err("Expected END")),
                }
            }
            Tok::While => {
                self.lex()?;

                // Condition
                let loop_target = self.code.len();
                self.expr()?;
                let to_after = self.code.len();
                self.code.push(Inst::BrFalse(0));

                self.require(Tok::Newline, "newline")?;

                // Body
                self.vertical_stmts()?;

                // Loop
                self.code.push(Inst::Br(loop_target));

                // Resolve branch targets
                self.code[to_after] = Inst::BrFalse(self.code.len());

                // End
                match self.tok {
                    Tok::End => {
                        self.lex()?;
                        if self.tok == Tok::While {
                            self.lex()?;
                        }
                    }
                    Tok::Endwhile | Tok::Wend => {
                        self.lex()?;
                    }
                    _ => return Err(self.err("Expected END")),
                }
            }
            Tok::If => {
                self.lex()?;

                // Condition
                self.expr()?;

                match self.tok {
                    Tok::Newline => {
                        self.vertical_if()?;
                    }
                    Tok::Then => {
                        self.lex()?;
                        match self.tok {
                            Tok::Newline => {
                                self.vertical_if()?;
                            }
                            _ => {
                                // Condition
                                let to_else = self.code.len();
                                self.code.push(Inst::BrFalse(0));

                                // If true
                                self.horizontal_stmts()?;

                                if self.tok == Tok::Else {
                                    // Else
                                    let to_after = self.code.len();
                                    self.code.push(Inst::Br(0));
                                    self.lex()?;

                                    // Resolve branch targets
                                    self.code[to_else] = Inst::BrFalse(self.code.len());

                                    // If false
                                    self.horizontal_stmts()?;

                                    // Resolve branch targets
                                    self.code[to_after] = Inst::Br(self.code.len());
                                } else {
                                    // Resolve branch targets
                                    self.code[to_else] = Inst::BrFalse(self.code.len());
                                }
                            }
                        }
                    }
                    _ => return Err(self.err("Syntax error")),
                }
            }
            Tok::Goto => {
                // TODO: Check order of processing input
                self.lex()?;
                let label = self.tok.clone();
                match label {
                    Tok::Int(_) | Tok::Float(_) | Tok::Id(_) => {}
                    _ => return Err(self.err("Expected label")),
                }
                self.lex()?;
                self.label_refs.push(LabelRef {
                    i: self.code.len(),
                    label,
                });
                self.code.push(Inst::Br(0));
            }
            Tok::Id(name) => {
                self.lex()?;
                self.require(Tok::Eq, "'='")?;
                self.expr()?;
                self.code.push(Inst::Store(name.to_string()));
            }
            Tok::Let => {
                self.lex()?;
                let tok = self.tok.clone();
                match tok {
                    Tok::Id(name) => {
                        self.lex()?;
                        self.require(Tok::Eq, "'='")?;
                        self.expr()?;
                        self.code.push(Inst::Store(name.to_string()));
                    }
                    _ => return Err(self.err("Expected name")),
                }
            }
            Tok::Print => {
                self.lex()?;
                if self.tok == Tok::Colon || self.tok == Tok::Newline || self.is_end() {
                    self.code.push(Inst::Const(Val::string("\n")));
                    self.code.push(Inst::Print);
                    return Ok(());
                }
                loop {
                    self.expr()?;
                    self.code.push(Inst::Print);
                    match self.tok {
                        Tok::Semi => {
                            self.lex()?;
                        }
                        Tok::Comma => {
                            self.lex()?;
                            self.code.push(Inst::Const(Val::string("\t")));
                            self.code.push(Inst::Print);
                        }
                        _ => {
                            self.code.push(Inst::Const(Val::string("\n")));
                            self.code.push(Inst::Print);
                            break;
                        }
                    }
                    if self.tok == Tok::Colon || self.tok == Tok::Newline || self.is_end() {
                        break;
                    }
                }
            }
            _ => return Err(self.err("Syntax error")),
        }
        Ok(())
    }

    fn horizontal_stmts(&mut self) -> Result<(), ParseError> {
        if self.tok == Tok::Newline || self.is_end() {
            return Ok(());
        }
        loop {
            self.stmt()?;
            if self.tok == Tok::Colon {
                self.lex()?;
            } else {
                return Ok(());
            }
        }
    }

    fn vertical_stmts(&mut self) -> Result<(), ParseError> {
        loop {
            match self.tok {
                Tok::Int(_) | Tok::Float(_) => {
                    self.labels.insert(self.tok.clone(), self.code.len());
                    self.lex()?;
                }
                Tok::Id(_) => {
                    if self.chars[self.pos] == ':' {
                        self.labels.insert(self.tok.clone(), self.code.len());
                        self.lex()?;
                        self.lex()?;
                    }
                }
                _ => {}
            }
            if self.is_end() {
                return Ok(());
            }
            self.horizontal_stmts()?;
            self.require(Tok::Newline, "newline")?;
        }
    }

    fn parse(&mut self) -> Result<Vec<Inst>, ParseError> {
        // Shebang line
        if self.chars[0] == '#' && self.chars[1] == '!' {
            self.eol();
            self.lex()?;
        }

        // Start the tokenizer
        self.lex()?;

        // Parse
        self.vertical_stmts()?;

        // For backward compatibility, accept trailing END
        if self.tok == Tok::End {
            self.lex()?;
            self.require(Tok::Newline, "newline")?;
        }

        // Check for extra stuff we couldn't parse
        if self.tok != Tok::Eof {
            return Err(self.err("Unmatched terminator"));
        }

        // Resolve labels
        for label_ref in &self.label_refs {
            let i = label_ref.i;
            let label = &label_ref.label;
            let target = match self.labels.get(label) {
                Some(target) => target,
                None => return Err(self.err(format!("Label '{}' is not defined", label))),
            };
            self.code[i] = match self.code[i] {
                Inst::Br(_) => Inst::Br(*target),
                Inst::BrFalse(_) => Inst::BrFalse(*target),
                _ => panic!(),
            }
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
