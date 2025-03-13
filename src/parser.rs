use crate::vm::*;
use fastnum::D256;
use num_traits::FromPrimitive;
use std::collections::HashMap;
use std::mem;

#[derive(Clone, Hash, PartialEq, Eq)]
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
    Then,
    Else,
    End,
    Star,
    Caret,
    Plus,
    Tilde,
    Minus,
    Slash,
    True,
    False,
    Xor,
    And,
    Or,
    Not,
    Lt,
    Le,
    Gt,
    Ge,
    Eq,
    Ne,
    LShr,
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

    pub line: usize,
    pub text: String,
    pub caret: usize,
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

    labels: HashMap<Tok, usize>,
    label_refs: Vec<LabelRef>,

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
        // Keywords
        let mut keywords = HashMap::new();
        keywords.insert("mod".to_string(), Tok::Mod);
        keywords.insert("let".to_string(), Tok::Let);
        keywords.insert("if".to_string(), Tok::If);
        keywords.insert("print".to_string(), Tok::Print);
        keywords.insert("true".to_string(), Tok::True);
        keywords.insert("false".to_string(), Tok::False);
        keywords.insert("div".to_string(), Tok::Div);
        keywords.insert("and".to_string(), Tok::And);
        keywords.insert("xor".to_string(), Tok::Xor);
        keywords.insert("or".to_string(), Tok::Or);
        keywords.insert("then".to_string(), Tok::Then);
        keywords.insert("not".to_string(), Tok::Not);
        keywords.insert("else".to_string(), Tok::Else);
        keywords.insert("end".to_string(), Tok::End);

        // Infix operators
        let mut ops = HashMap::new();
        let mut add = |o: Tok, prec: u8, left: u8, inst: Inst| {
            ops.insert(o, Op { prec, left, inst });
        };

        let mut prec = 99u8;
        add(Tok::Caret, prec, 0, Inst::Pow);

        prec -= 1;
        add(Tok::Star, prec, 1, Inst::Mul);
        add(Tok::Slash, prec, 1, Inst::Div);
        add(Tok::Div, prec, 1, Inst::IDiv);
        add(Tok::Mod, prec, 1, Inst::Mod);

        prec -= 1;
        add(Tok::Plus, prec, 1, Inst::Add);
        add(Tok::Minus, prec, 1, Inst::Sub);

        prec -= 1;
        add(Tok::Shl, prec, 1, Inst::Shl);
        add(Tok::Shr, prec, 1, Inst::Shr);
        add(Tok::LShr, prec, 1, Inst::LShr);

        prec -= 1;
        add(Tok::Eq, prec, 1, Inst::Eq);
        add(Tok::Ne, prec, 1, Inst::Ne);
        add(Tok::Lt, prec, 1, Inst::Lt);
        add(Tok::Gt, prec, 1, Inst::Gt);
        add(Tok::Le, prec, 1, Inst::Le);
        add(Tok::Ge, prec, 1, Inst::Ge);

        prec -= 1;
        add(Tok::And, prec, 1, Inst::And);

        prec -= 1;
        add(Tok::Xor, prec, 1, Inst::Xor);

        prec -= 1;
        add(Tok::Or, prec, 1, Inst::Or);

        // Parser object
        Parser {
            keywords,
            ops,
            chars,
            caret: 0,
            pos: 0,
            line: 1,
            tok: Tok::Newline,
            labels: HashMap::<Tok, usize>::new(),
            label_refs: Vec::<LabelRef>::new(),
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
                            if self.chars[self.pos + 2] == '>' {
                                self.pos += 3;
                                Tok::LShr
                            } else {
                                self.pos += 2;
                                Tok::Shr
                            }
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

    fn primary(&mut self) -> Result<(), ParseError> {
        match &self.tok {
            Tok::Id(name) => {
                self.code.push(Inst::Load(name.to_string()));
                self.lex()?;
            }
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

    fn is_end_stmt(&self) -> bool {
        // TODO: Is this going to be used by anything other than PRINT?
        self.tok == Tok::Newline || self.tok == Tok::Colon || self.tok == Tok::Else
    }

    fn stmt(&mut self) -> Result<(), ParseError> {
        let tok = self.tok.clone();
        match tok {
            Tok::If => {
                self.lex()?;
                self.expr()?;
                let to_else = self.code.len();
                self.code.push(Inst::BrFalse(0));
                self.require(Tok::Then, "THEN")?;
                self.stmt()?;
                self.code[to_else] = Inst::BrFalse(self.code.len());
            }
            Tok::Num(_) => {
                self.labels.insert(tok, self.code.len());
                self.lex()?;
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
                    if self.is_end_stmt() {
                        break;
                    }
                }
            }
            _ => return Err(self.err("Syntax error")),
        }
        Ok(())
    }

    fn horizontal_stmts(&mut self) -> Result<(), ParseError> {
        if self.tok == Tok::Newline {
            return Ok(());
        }
        loop {
            self.stmt()?;
            match self.tok {
                Tok::Colon => {
                    self.lex()?;
                }
                Tok::Newline | Tok::Else => {
                    return Ok(());
                }
                _ => {
                    return Err(self.err("Syntax error"));
                }
            }
        }
    }

    fn vertical_stmts(&mut self) -> Result<(), ParseError> {
        loop {
            match self.tok {
                Tok::Eof | Tok::Else | Tok::End => {
                    return Ok(());
                }
                _ => {
                    self.horizontal_stmts()?;
                    self.require(Tok::Newline, "newline")?;
                }
            }
        }
    }

    fn parse(&mut self) -> Result<Vec<Inst>, ParseError> {
        if self.chars[0] == '#' && self.chars[1] == '!' {
            self.eol();
            self.lex()?;
        }
        self.lex()?;
        self.vertical_stmts()?;
        match self.tok {
            Tok::Eof => Ok(mem::take(&mut self.code)),
            Tok::Else | Tok::End => Err(self.err("Unmatched terminator")),
            _ => Err(self.err("Syntax error")),
        }
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
