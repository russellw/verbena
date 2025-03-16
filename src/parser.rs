use crate::error::*;
use crate::vm::*;
use num_bigint::BigInt;
use num_traits::{One, Zero};
use std::collections::HashMap;
use std::fmt;
use std::mem;

// TODO: CamelCase consistency
#[derive(Clone, Hash, PartialEq, Eq)]
enum Tok {
    Dim,
    While,
    Wend,
    Endfor,
    Endwhile,
    Int(BigInt),
    Float(String),
    Str(String),
    Id(String),
    Colon,
    Pow,
    Gcd,
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
    ToStr,
    ToFloat,
    Div,
    Sqrt,
    Shl,
    Print,
    Mod,
    Floor,
    Let,
    If,

    Ceil,
    Round,
    RoundTiesEven,
    Trunc,
    Fract,
    MulAdd,
    DivEuclid,
    RemEuclid,
    PowI,
    Exp,
    Exp2,
    Ln,
    Log,
    Log2,
    Log10,
    Hypot,
    Sin,
    Cos,
    Tan,
    ASin,
    ACos,
    ATan,
    ATan2,
    ExpM1,
    Ln1P,
    SinH,
    CosH,
    TanH,
    ASinH,
    ACosH,
    ATanH,
    IsNan,
    IsFinite,
    IsInfinite,
    IsSubnormal,
    IsNormal,
    IsSignPositive,
    IsSignNegative,
    Recip,
    ToDegrees,
    ToRadians,
    NthRoot,
    TrailingZeros,
    Bit,
    SetBit,
    Cbrt,
    Max,
    Min,
    Midpoint,
    TotalCmp,
    Clamp,
    Abs,
    Signum,
    CopySign,

    Pi,
    Infinity,
    NegInfinity,
    Lcm,
    Nan,
    StrBase,
    ValBase,

    Len,
    Left,
    Right,
    Mid,
    Asc,
    Chr,
    Instr,
    UCase,
    LCase,
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

struct Parser<'a> {
    // There is a compile-time perfect hash package
    // but there are benchmarks showing HashMap to be faster
    keywords: HashMap<String, Tok>,

    // Table of infix operators
    ops: HashMap<Tok, Op>,

    // Decode the entire input text upfront
    // to make sure there are no situations in which decoding work is repeated
    text: &'a Vec<char>,

    // This is where the caret will point to in case of error
    // Most of the time, it points to the start of current token
    caret: usize,

    // Current position in the input text
    // Mostly tracked and used by the tokenizer
    // Most of the time, it points just after the current token
    pos: usize,

    // Current token
    tok: Tok,

    // Counter for generating temporary names
    tmp_count: usize,

    labels: HashMap<Tok, usize>,
    label_refs: Vec<LabelRef>,

    carets: Vec<usize>,
    code: Vec<Inst>,
}

fn is_id_part(c: char) -> bool {
    c.is_alphanumeric() || c == '_' || c == '$' || c == '%'
}

fn substr(text: &[char], i: usize, j: usize) -> String {
    text.iter().skip(i).take(j - i).collect()
}

impl<'a> Parser<'a> {
    fn new(text: &'a Vec<char>) -> Self {
        // Keywords
        let mut keywords = HashMap::new();
        keywords.insert("dim".to_string(), Tok::Dim);
        keywords.insert("lcm".to_string(), Tok::Lcm);
        keywords.insert("gcd".to_string(), Tok::Gcd);
        keywords.insert("assert".to_string(), Tok::Assert);
        keywords.insert("int".to_string(), Tok::ToInt);
        keywords.insert("float".to_string(), Tok::ToFloat);
        keywords.insert("val".to_string(), Tok::ToFloat);
        keywords.insert("str".to_string(), Tok::ToStr);
        keywords.insert("str$".to_string(), Tok::ToStr);
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
        keywords.insert("floor".to_string(), Tok::Floor);
        keywords.insert("pow".to_string(), Tok::Pow);
        keywords.insert("strbase".to_string(), Tok::StrBase);
        keywords.insert("valbase".to_string(), Tok::ValBase);

        keywords.insert("div_euclid".to_string(), Tok::DivEuclid);
        keywords.insert("ceil".to_string(), Tok::Ceil);
        keywords.insert("round".to_string(), Tok::Round);
        keywords.insert("round_ties_even".to_string(), Tok::RoundTiesEven);
        keywords.insert("trunc".to_string(), Tok::Trunc);
        keywords.insert("fract".to_string(), Tok::Fract);
        keywords.insert("mul_add".to_string(), Tok::MulAdd);
        keywords.insert("rem_euclid".to_string(), Tok::RemEuclid);
        keywords.insert("pow_i".to_string(), Tok::PowI);
        keywords.insert("exp".to_string(), Tok::Exp);
        keywords.insert("exp2".to_string(), Tok::Exp2);
        keywords.insert("ln".to_string(), Tok::Ln);
        keywords.insert("log".to_string(), Tok::Log);
        keywords.insert("log2".to_string(), Tok::Log2);
        keywords.insert("log10".to_string(), Tok::Log10);
        keywords.insert("hypot".to_string(), Tok::Hypot);
        keywords.insert("sin".to_string(), Tok::Sin);
        keywords.insert("cos".to_string(), Tok::Cos);
        keywords.insert("tan".to_string(), Tok::Tan);
        keywords.insert("asin".to_string(), Tok::ASin);
        keywords.insert("acos".to_string(), Tok::ACos);
        keywords.insert("atan".to_string(), Tok::ATan);
        keywords.insert("atan2".to_string(), Tok::ATan2);
        keywords.insert("exp_m1".to_string(), Tok::ExpM1);
        keywords.insert("ln1p".to_string(), Tok::Ln1P);
        keywords.insert("sinh".to_string(), Tok::SinH);
        keywords.insert("cosh".to_string(), Tok::CosH);
        keywords.insert("tanh".to_string(), Tok::TanH);
        keywords.insert("asinh".to_string(), Tok::ASinH);
        keywords.insert("acosh".to_string(), Tok::ACosH);
        keywords.insert("atanh".to_string(), Tok::ATanH);
        keywords.insert("is_nan".to_string(), Tok::IsNan);
        keywords.insert("is_finite".to_string(), Tok::IsFinite);
        keywords.insert("is_infinite".to_string(), Tok::IsInfinite);
        keywords.insert("is_subnormal".to_string(), Tok::IsSubnormal);
        keywords.insert("is_normal".to_string(), Tok::IsNormal);
        keywords.insert("is_sign_positive".to_string(), Tok::IsSignPositive);
        keywords.insert("is_sign_negative".to_string(), Tok::IsSignNegative);
        keywords.insert("recip".to_string(), Tok::Recip);
        keywords.insert("to_degrees".to_string(), Tok::ToDegrees);
        keywords.insert("to_radians".to_string(), Tok::ToRadians);
        keywords.insert("nth_root".to_string(), Tok::NthRoot);
        keywords.insert("trailing_zeros".to_string(), Tok::TrailingZeros);
        keywords.insert("bit".to_string(), Tok::Bit);
        keywords.insert("set_bit".to_string(), Tok::SetBit);
        keywords.insert("cbrt".to_string(), Tok::Cbrt);
        keywords.insert("max".to_string(), Tok::Max);
        keywords.insert("min".to_string(), Tok::Min);
        keywords.insert("midpoint".to_string(), Tok::Midpoint);
        keywords.insert("total_cmp".to_string(), Tok::TotalCmp);
        keywords.insert("clamp".to_string(), Tok::Clamp);
        keywords.insert("abs".to_string(), Tok::Abs);
        keywords.insert("signum".to_string(), Tok::Signum);
        keywords.insert("copy_sign".to_string(), Tok::CopySign);

        keywords.insert("pi".to_string(), Tok::Pi);
        keywords.insert("infinity".to_string(), Tok::Infinity);
        keywords.insert("neg_infinity".to_string(), Tok::NegInfinity);
        keywords.insert("nan".to_string(), Tok::Nan);

        keywords.insert("len".to_string(), Tok::Len);
        keywords.insert("left$".to_string(), Tok::Left);
        keywords.insert("right$".to_string(), Tok::Right);
        keywords.insert("mid$".to_string(), Tok::Mid);
        keywords.insert("asc".to_string(), Tok::Asc);
        keywords.insert("chr$".to_string(), Tok::Chr);
        keywords.insert("instr".to_string(), Tok::Instr);
        keywords.insert("ucase$".to_string(), Tok::UCase);
        keywords.insert("lcase$".to_string(), Tok::LCase);

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
            text,
            caret: 0,
            pos: 0,
            tok: Tok::Newline,
            tmp_count: 0,
            labels: HashMap::<Tok, usize>::new(),
            label_refs: Vec::<LabelRef>::new(),
            carets: Vec::<usize>::new(),
            code: Vec::<Inst>::new(),
        }
    }

    fn tmp_name(&mut self) -> String {
        let i = self.tmp_count;
        self.tmp_count = i + 1;
        format!("__{}", i)
    }

    fn err<S: AsRef<str>>(&self, msg: S) -> Error {
        Error {
            caret: self.caret,
            msg: msg.as_ref().to_string(),
        }
    }

    // Tokenizer
    fn eol(&mut self) {
        let mut i = self.pos;
        while self.text[i] != '\n' {
            i += 1;
        }
        self.pos = i;
    }

    fn hex_to_char(&mut self, i: usize, j: usize) -> Result<char, Error> {
        self.caret = i;
        let s: String = substr(&self.text, i, j);
        let n = match u32::from_str_radix(&s, 16) {
            Ok(n) => n,
            Err(e) => return Err(self.err(e.to_string())),
        };
        match char::from_u32(n) {
            Some(c) => Ok(c),
            None => Err(self.err("Not a valid Unicode character")),
        }
    }

    fn lex_int(&mut self, radix: u32) -> Result<(), Error> {
        let mut i = self.pos + 2;
        let mut v = Vec::<char>::new();
        while self.text[i].is_digit(radix) || self.text[i] == '_' {
            if self.text[i] != '_' {
                v.push(self.text[i])
            }
            i += 1;
        }
        self.pos = i;
        let s: String = v.into_iter().collect();
        let a = match BigInt::parse_bytes(s.as_bytes(), radix) {
            Some(a) => a,
            None => return Err(self.err("Expected integer")),
        };
        self.tok = Tok::Int(a);
        Ok(())
    }

    fn lex(&mut self) -> Result<(), Error> {
        while self.pos < self.text.len() {
            self.caret = self.pos;
            let c = self.text[self.pos];
            match c {
                '"' => {
                    let mut i = self.pos + 1;
                    let mut v = Vec::<char>::new();
                    while self.text[i] != '"' {
                        let mut c = self.text[i];
                        i += 1;
                        match c {
                            '\n' => {
                                self.caret = i - 1;
                                return Err(self.err("Unterminated string"));
                            }
                            '\\' => {
                                c = self.text[i];
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
                                        if self.text[i] != '{' {
                                            self.caret = i;
                                            return Err(self.err("Expected '{'"));
                                        }
                                        i += 1;

                                        let mut j = i;
                                        while self.text[j].is_ascii_hexdigit() {
                                            j += 1;
                                        }
                                        let c = self.hex_to_char(i, j)?;
                                        i = j;

                                        if self.text[i] != '}' {
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
                            self.caret = self.pos + 1;
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
                        let mut i = self.pos;
                        loop {
                            i += 1;
                            if !is_id_part(self.text[i]) {
                                break;
                            }
                        }
                        let s = substr(&self.text, self.pos, i).to_lowercase();
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
                            match self.text[self.pos + 1] {
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
                            let c = self.text[i];
                            if c != '_' {
                                v.push(c);
                            }
                            i += 1;
                            if !(self.text[i].is_ascii_digit() || self.text[i] == '_') {
                                break;
                            }
                        }

                        // Decimal point
                        if self.text[i] == '.' {
                            loop {
                                let c = self.text[i];
                                if c != '_' {
                                    v.push(c);
                                }
                                i += 1;
                                if !(self.text[i].is_ascii_digit() || self.text[i] == '_') {
                                    break;
                                }
                            }
                        }

                        // Exponent
                        match self.text[i] {
                            'e' | 'E' => {
                                v.push('e');
                                i += 1;
                                match self.text[i] {
                                    '+' | '-' => {
                                        v.push(self.text[i]);
                                        i += 1;
                                    }
                                    _ => {}
                                }
                                while self.text[i].is_ascii_digit() || self.text[i] == '_' {
                                    if self.text[i] != '_' {
                                        v.push(self.text[i])
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
                        if let Ok(a) = s.parse::<BigInt>() {
                            self.tok = Tok::Int(a);
                            return Ok(());
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

    fn require(&mut self, tok: Tok, s: &str) -> Result<(), Error> {
        if self.tok != tok {
            return Err(self.err(format!("Expected {}", s)));
        }
        self.lex()?;
        Ok(())
    }

    // Expressions
    fn add(&mut self, inst: Inst) {
        self.carets.push(self.caret);
        self.code.push(inst);
    }

    fn primary1(&mut self, inst: Inst) -> Result<(), Error> {
        self.lex()?;
        self.primary()?;
        self.add(inst);
        Ok(())
    }

    fn primary2(&mut self, inst: Inst) -> Result<(), Error> {
        self.lex()?;
        self.require(Tok::LParen, "'('")?;
        self.expr()?;
        self.require(Tok::Comma, "','")?;
        self.expr()?;
        self.add(inst);
        self.require(Tok::RParen, "')'")?;
        Ok(())
    }

    fn primary3(&mut self, inst: Inst) -> Result<(), Error> {
        self.lex()?;
        self.require(Tok::LParen, "'('")?;
        self.expr()?;
        self.require(Tok::Comma, "','")?;
        self.expr()?;
        self.require(Tok::Comma, "','")?;
        self.expr()?;
        self.add(inst);
        self.require(Tok::RParen, "')'")?;
        Ok(())
    }

    fn primary(&mut self) -> Result<(), Error> {
        match &self.tok {
            Tok::StrBase => {
                self.primary2(Inst::StrBase)?;
            }
            Tok::ValBase => {
                self.primary2(Inst::ValBase)?;
            }
            Tok::Gcd => {
                self.primary2(Inst::Gcd)?;
            }
            Tok::Lcm => {
                self.primary2(Inst::Lcm)?;
            }
            Tok::BitAnd => {
                self.primary2(Inst::BitAnd)?;
            }
            Tok::Pow => {
                self.primary2(Inst::Pow)?;
            }
            Tok::BitOr => {
                self.primary2(Inst::BitOr)?;
            }
            Tok::BitXor => {
                self.primary2(Inst::BitXor)?;
            }
            Tok::Sqrt => {
                self.primary1(Inst::Sqrt)?;
            }
            Tok::Floor => {
                self.primary1(Inst::Floor)?;
            }
            Tok::ToInt => {
                self.primary1(Inst::ToInt)?;
            }
            Tok::ToFloat => {
                self.primary1(Inst::ToFloat)?;
            }
            Tok::ToStr => {
                self.primary1(Inst::ToStr)?;
            }
            Tok::Id(name) => {
                self.add(Inst::Load(name.to_string()));
                self.lex()?;
            }
            Tok::Str(s) => {
                self.add(Inst::Const(Val::string(s)));
                self.lex()?;
            }
            Tok::Int(a) => {
                self.add(Inst::Const(Val::Int(a.clone())));
                self.lex()?;
            }
            Tok::Float(s) => match s.parse::<f64>() {
                Ok(a) => {
                    self.add(Inst::Const(Val::Float(a)));
                    self.lex()?;
                }
                Err(e) => return Err(self.err(e.to_string())),
            },
            Tok::False => {
                self.add(Inst::Const(Val::Int(BigInt::zero())));
                self.lex()?;
            }
            Tok::LParen => {
                self.lex()?;
                self.expr()?;
                self.require(Tok::RParen, "')'")?;
            }
            Tok::True => {
                self.add(Inst::Const(Val::Int(BigInt::one())));
                self.lex()?;
            }

            Tok::DivEuclid => {
                self.primary2(Inst::DivEuclid)?;
            }
            Tok::Ceil => {
                self.primary1(Inst::Ceil)?;
            }
            Tok::Round => {
                self.primary1(Inst::Round)?;
            }
            Tok::RoundTiesEven => {
                self.primary1(Inst::RoundTiesEven)?;
            }
            Tok::Trunc => {
                self.primary1(Inst::Trunc)?;
            }
            Tok::Fract => {
                self.primary1(Inst::Fract)?;
            }
            Tok::MulAdd => {
                self.primary3(Inst::MulAdd)?;
            }
            Tok::RemEuclid => {
                self.primary2(Inst::RemEuclid)?;
            }
            Tok::PowI => {
                self.primary2(Inst::PowI)?;
            }
            Tok::Exp => {
                self.primary1(Inst::Exp)?;
            }
            Tok::Exp2 => {
                self.primary1(Inst::Exp2)?;
            }
            Tok::Ln => {
                self.primary1(Inst::Ln)?;
            }
            Tok::Log => {
                self.primary2(Inst::Log)?;
            }
            Tok::Log2 => {
                self.primary1(Inst::Log2)?;
            }
            Tok::Log10 => {
                self.primary1(Inst::Log10)?;
            }
            Tok::Hypot => {
                self.primary2(Inst::Hypot)?;
            }
            Tok::Sin => {
                self.primary1(Inst::Sin)?;
            }
            Tok::Cos => {
                self.primary1(Inst::Cos)?;
            }
            Tok::Tan => {
                self.primary1(Inst::Tan)?;
            }
            Tok::ASin => {
                self.primary1(Inst::ASin)?;
            }
            Tok::ACos => {
                self.primary1(Inst::ACos)?;
            }
            Tok::ATan => {
                self.primary1(Inst::ATan)?;
            }
            Tok::ATan2 => {
                self.primary2(Inst::ATan2)?;
            }
            Tok::ExpM1 => {
                self.primary1(Inst::ExpM1)?;
            }
            Tok::Ln1P => {
                self.primary1(Inst::Ln1P)?;
            }
            Tok::SinH => {
                self.primary1(Inst::SinH)?;
            }
            Tok::CosH => {
                self.primary1(Inst::CosH)?;
            }
            Tok::TanH => {
                self.primary1(Inst::TanH)?;
            }
            Tok::ASinH => {
                self.primary1(Inst::ASinH)?;
            }
            Tok::ACosH => {
                self.primary1(Inst::ACosH)?;
            }
            Tok::ATanH => {
                self.primary1(Inst::ATanH)?;
            }
            Tok::IsNan => {
                self.primary1(Inst::IsNan)?;
            }
            Tok::IsFinite => {
                self.primary1(Inst::IsFinite)?;
            }
            Tok::IsInfinite => {
                self.primary1(Inst::IsInfinite)?;
            }
            Tok::IsSubnormal => {
                self.primary1(Inst::IsSubnormal)?;
            }
            Tok::IsNormal => {
                self.primary1(Inst::IsNormal)?;
            }
            Tok::IsSignPositive => {
                self.primary1(Inst::IsSignPositive)?;
            }
            Tok::IsSignNegative => {
                self.primary1(Inst::IsSignNegative)?;
            }
            Tok::Recip => {
                self.primary1(Inst::Recip)?;
            }
            Tok::ToDegrees => {
                self.primary1(Inst::ToDegrees)?;
            }
            Tok::ToRadians => {
                self.primary1(Inst::ToRadians)?;
            }
            Tok::NthRoot => {
                self.primary2(Inst::NthRoot)?;
            }
            Tok::TrailingZeros => {
                self.primary1(Inst::TrailingZeros)?;
            }
            Tok::Bit => {
                self.primary2(Inst::Bit)?;
            }
            Tok::SetBit => {
                self.primary3(Inst::SetBit)?;
            }
            Tok::Cbrt => {
                self.primary1(Inst::Cbrt)?;
            }
            Tok::Max => {
                self.primary2(Inst::Max)?;
            }
            Tok::Min => {
                self.primary2(Inst::Min)?;
            }
            Tok::Midpoint => {
                self.primary2(Inst::Midpoint)?;
            }
            Tok::TotalCmp => {
                self.primary2(Inst::TotalCmp)?;
            }
            Tok::Clamp => {
                self.primary3(Inst::Clamp)?;
            }
            Tok::Abs => {
                self.primary1(Inst::Abs)?;
            }
            Tok::Signum => {
                self.primary1(Inst::Signum)?;
            }
            Tok::CopySign => {
                self.primary2(Inst::CopySign)?;
            }

            Tok::Pi => {
                self.add(Inst::Const(Val::Float(std::f64::consts::PI)));
                self.lex()?;
            }
            Tok::Infinity => {
                self.add(Inst::Const(Val::Float(std::f64::INFINITY)));
                self.lex()?;
            }
            Tok::NegInfinity => {
                self.add(Inst::Const(Val::Float(std::f64::NEG_INFINITY)));
                self.lex()?;
            }
            Tok::Nan => {
                self.add(Inst::Const(Val::Float(std::f64::NAN)));
                self.lex()?;
            }

            Tok::Len => {
                self.primary1(Inst::Len)?;
            }
            Tok::Left => {
                self.primary2(Inst::Left)?;
            }
            Tok::Right => {
                self.primary2(Inst::Right)?;
            }
            Tok::Mid => {
                self.primary3(Inst::Mid)?;
            }
            Tok::Asc => {
                self.primary1(Inst::Asc)?;
            }
            Tok::Chr => {
                self.primary1(Inst::Chr)?;
            }
            Tok::Instr => {
                self.primary2(Inst::Instr)?;
            }
            Tok::UCase => {
                self.primary1(Inst::UCase)?;
            }
            Tok::LCase => {
                self.primary1(Inst::LCase)?;
            }

            _ => return Err(self.err("Expected expression")),
        }
        Ok(())
    }

    fn prefix(&mut self) -> Result<(), Error> {
        match &self.tok {
            Tok::Minus => {
                self.lex()?;
                self.prefix()?;
                self.add(Inst::Neg);
            }
            Tok::Tilde => {
                self.lex()?;
                self.prefix()?;
                self.add(Inst::BitNot);
            }
            _ => {
                self.primary()?;
            }
        }
        Ok(())
    }

    fn infix(&mut self, prec: u8) -> Result<(), Error> {
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
            self.add(o.inst);
        }
    }

    fn not(&mut self) -> Result<(), Error> {
        if self.tok == Tok::Not {
            self.lex()?;
            self.not()?;
            self.add(Inst::Not);
        } else {
            self.infix(0)?;
        }
        Ok(())
    }

    fn and(&mut self) -> Result<(), Error> {
        self.not()?;
        if self.tok == Tok::And {
            let to_after = self.code.len();
            self.add(Inst::DupBrFalse(0));
            self.add(Inst::Pop);
            self.lex()?;
            self.and()?;
            self.code[to_after] = Inst::DupBrFalse(self.code.len());
        }
        Ok(())
    }

    fn or(&mut self) -> Result<(), Error> {
        self.and()?;
        if self.tok == Tok::Or {
            let to_after = self.code.len();
            self.add(Inst::DupBrTrue(0));
            self.add(Inst::Pop);
            self.lex()?;
            self.or()?;
            self.code[to_after] = Inst::DupBrFalse(self.code.len());
        }
        Ok(())
    }

    fn expr(&mut self) -> Result<(), Error> {
        self.or()
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

    fn vertical_if(&mut self) -> Result<(), Error> {
        // Condition
        let to_else = self.code.len();
        self.add(Inst::BrFalse(0));

        // If true
        self.vertical_stmts()?;

        if self.tok == Tok::Else {
            self.lex()?;
            let to_after = self.code.len();
            self.add(Inst::Br(0));

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

    fn stmt(&mut self) -> Result<(), Error> {
        let tok = self.tok.clone();
        match tok {
            Tok::Dim => {
                self.lex()?;

                // Name
                let name = match &self.tok {
                    Tok::Id(name) => name.clone(),
                    _ => return Err(self.err("Expected array name")),
                };
                self.lex()?;

                // Count
                self.primary()?;

                // Allocate and store
                // TODO: caret should be on the count, not after it
                self.add(Inst::Dim);
                self.add(Inst::Store(name));
            }
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
                self.add(Inst::Store(counter_name.clone()));

                self.require(Tok::To, "TO")?;

                // Final value
                self.expr()?;
                let final_name = self.tmp_name();
                self.add(Inst::Store(final_name.clone()));

                let loop_target: usize;
                let to_after: usize;

                if self.tok == Tok::Step {
                    // Step
                    let step_name = self.tmp_name();
                    self.lex()?;
                    let step_down = self.tok == Tok::Minus;
                    self.expr()?;
                    self.add(Inst::Store(step_name.clone()));

                    // Condition
                    loop_target = self.code.len();
                    self.add(Inst::Load(counter_name.clone()));
                    self.add(Inst::Load(final_name));
                    self.add(if step_down { Inst::Ge } else { Inst::Le });
                    to_after = self.code.len();
                    self.add(Inst::BrFalse(0));

                    self.require(Tok::Newline, "newline")?;

                    // Body
                    self.vertical_stmts()?;

                    // Increment
                    self.add(Inst::Load(counter_name.clone()));
                    self.add(Inst::Load(step_name.clone()));
                } else {
                    // Condition
                    loop_target = self.code.len();
                    self.add(Inst::Load(counter_name.clone()));
                    self.add(Inst::Load(final_name));
                    self.add(Inst::Le);
                    to_after = self.code.len();
                    self.add(Inst::BrFalse(0));

                    self.require(Tok::Newline, "newline")?;

                    // Body
                    self.vertical_stmts()?;

                    // Increment
                    self.add(Inst::Load(counter_name.clone()));
                    self.add(Inst::Const(Val::Int(BigInt::one())));
                }

                // Increment
                self.add(Inst::Add);
                self.add(Inst::Store(counter_name.clone()));

                // Loop
                self.add(Inst::Br(loop_target));

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
                self.add(Inst::BrFalse(0));

                self.require(Tok::Newline, "newline")?;

                // Body
                self.vertical_stmts()?;

                // Loop
                self.add(Inst::Br(loop_target));

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
            Tok::Assert => {
                self.lex()?;
                self.expr()?;
                self.add(Inst::Assert);
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
                                self.add(Inst::BrFalse(0));

                                // If true
                                self.horizontal_stmts()?;

                                if self.tok == Tok::Else {
                                    // Else
                                    let to_after = self.code.len();
                                    self.add(Inst::Br(0));
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
                self.add(Inst::Br(0));
            }
            Tok::Id(name) => {
                self.lex()?;
                self.require(Tok::Eq, "'='")?;
                self.expr()?;
                self.add(Inst::Store(name.to_string()));
            }
            Tok::Let => {
                self.lex()?;
                let tok = self.tok.clone();
                match tok {
                    Tok::Id(name) => {
                        self.lex()?;
                        self.require(Tok::Eq, "'='")?;
                        self.expr()?;
                        self.add(Inst::Store(name.to_string()));
                    }
                    _ => return Err(self.err("Expected name")),
                }
            }
            Tok::Print => {
                self.lex()?;
                if self.tok == Tok::Colon || self.tok == Tok::Newline || self.is_end() {
                    self.add(Inst::Const(Val::string("\n")));
                    self.add(Inst::Print);
                    return Ok(());
                }
                loop {
                    self.expr()?;
                    self.add(Inst::Print);
                    match self.tok {
                        Tok::Semi => {
                            self.lex()?;
                        }
                        Tok::Comma => {
                            self.lex()?;
                            self.add(Inst::Const(Val::string("\t")));
                            self.add(Inst::Print);
                        }
                        _ => {
                            self.add(Inst::Const(Val::string("\n")));
                            self.add(Inst::Print);
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

    fn horizontal_stmts(&mut self) -> Result<(), Error> {
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

    fn vertical_stmts(&mut self) -> Result<(), Error> {
        loop {
            match self.tok {
                Tok::Int(_) | Tok::Float(_) => {
                    self.labels.insert(self.tok.clone(), self.code.len());
                    self.lex()?;
                }
                Tok::Id(_) => {
                    if self.text[self.pos] == ':' {
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

    fn parse(&mut self) -> Result<Program, Error> {
        // Shebang line
        if self.text[0] == '#' && self.text[1] == '!' {
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

        Ok(Program::new(
            mem::take(&mut self.carets),
            mem::take(&mut self.code),
        ))
    }
}

pub fn prep(text: &str) -> Vec<char> {
    let mut chars: Vec<char> = text.chars().collect();
    if !text.ends_with('\n') {
        chars.push('\n');
    }
    chars
}

pub fn parse(text: &Vec<char>) -> Result<Program, Error> {
    assert_eq!(*text.last().unwrap(), '\n');
    let mut parser = Parser::new(text);
    parser.parse()
}
