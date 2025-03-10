use fastnum::decimal::Context;
use fastnum::{dec256, D256};
use std::fmt;
use std::rc::Rc;

// fastnum provides Inf, so we might as well use it
pub const NO_TRAPS: Context = Context::default().without_traps();

#[derive(Clone, Debug)]
pub enum Val {
    Num(D256),
    Str(Rc<String>),
}

pub type ValResult = Result<Val, String>;

impl Val {
    pub fn num(a: D256) -> Self {
        Val::Num(a)
    }

    pub fn string<S: Into<String>>(s: S) -> Self {
        Val::Str(Rc::new(s.into()))
    }

    pub fn as_string(&self) -> String {
        match self {
            Val::Str(s) => s.to_string(),
            Val::Num(a) => a.to_string(),
        }
    }

    pub fn truth(&self) -> bool {
        match self {
            Val::Str(s) => s.len() != 0,
            Val::Num(a) => !a.is_zero(),
        }
    }
}

impl fmt::Display for Val {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Val::Num(a) => write!(f, "{}", a),
            Val::Str(s) => write!(f, "{}", s),
        }
    }
}

impl PartialEq for Val {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Val::Num(a), Val::Num(b)) => a == b,
            (Val::Str(a), Val::Str(b)) => a == b,
            _ => false,
        }
    }
}

#[derive(Debug)]
pub enum Inst {
    Add,
    Sub,
    Mul,
    Div,
    Goto(usize),
    End,
    Const(Val),
    Print,
}

#[derive(Debug)]
pub struct VM {
    code: Vec<Inst>,
    pc: usize,
    stack: Vec<Val>,
}

impl VM {
    pub fn new(code: Vec<Inst>) -> Self {
        VM {
            code,
            pc: 0,
            stack: Vec::new(),
        }
    }

    fn push(&mut self, val: Val) {
        self.stack.push(val);
    }

    fn pop(&mut self) -> Val {
        self.stack.pop().expect("stack underflow")
    }

    pub fn run(&mut self) -> Result<(), String> {
        while self.pc < self.code.len() {
            let i = self.pc;
            self.pc += 1;
            match &self.code[i] {
                Inst::Print => {
                    let a = self.pop();
                    print!("{}", a)
                }
                Inst::Const(a) => {
                    self.push(a.clone());
                }
                Inst::Add => {
                    let b = self.pop();
                    let a = self.pop();
                    let r = match (&a, &b) {
                        (Val::Num(a), Val::Num(b)) => Val::Num(*a + *b),
                        (Val::Str(a), Val::Str(b)) => {
                            let mut r = String::with_capacity(a.len() + b.len());
                            r.push_str(&a);
                            r.push_str(&b);
                            Val::string(r)
                        }
                        _ => {
                            let a = a.as_string();
                            let b = b.as_string();
                            let mut r = String::with_capacity(a.len() + b.len());
                            r.push_str(&a);
                            r.push_str(&b);
                            Val::string(r)
                        }
                    };
                    self.push(r);
                }
                Inst::Sub => {
                    let b = self.pop();
                    let a = self.pop();
                    let r = match (&a, &b) {
                        (Val::Num(a), Val::Num(b)) => Val::Num(*a - *b),
                        _ => {
                            return Err("-: expected numbers".to_string());
                        }
                    };
                    self.push(r);
                }
                Inst::Div => {
                    let b = self.pop();
                    let a = self.pop();
                    let r = match (&a, &b) {
                        (Val::Num(a), Val::Num(b)) => Val::Num(*a - *b),
                        _ => {
                            return Err("/: expected numbers".to_string());
                        }
                    };
                    self.push(r);
                }
                Inst::Mul => {
                    let b = self.pop();
                    let a = self.pop();
                    let r = match (&a, &b) {
                        (Val::Num(a), Val::Num(b)) => Val::Num(*a * *b),
                        (Val::Num(a), Val::Str(b)) => {
                            let count = match usize::try_from(*a) {
                                Ok(n) => n,
                                Err(_) => {
                                    return Err(
                                        "*: cannot convert number to repeat count".to_string()
                                    )
                                }
                            };
                            Val::Str(Rc::new(b.repeat(count)))
                        }
                        (Val::Str(a), Val::Num(b)) => {
                            let count = match usize::try_from(*b) {
                                Ok(n) => n,
                                Err(_) => {
                                    return Err(
                                        "*: cannot convert number to repeat count".to_string()
                                    )
                                }
                            };
                            Val::Str(Rc::new(a.repeat(count)))
                        }
                        _ => {
                            return Err("*: expected numbers".to_string());
                        }
                    };
                    self.push(r);
                }
                Inst::Goto(target) => {
                    self.pc = *target;
                }
                Inst::End => {
                    return Ok(());
                }
                _ => {
                    panic!("")
                }
            }
        }
        return Ok(());
    }
}
