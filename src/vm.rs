use fastnum::decimal::Context;
use fastnum::{D256, dec256};
use std::fmt;
use std::rc::Rc;

// fastnum provides Inf, so we might as well use it
pub const NO_TRAPS: Context = Context::default().without_traps();

#[derive(Clone, Debug, PartialEq)]
pub enum Val {
    Num(D256),
    Str(Rc<String>),
}

impl Val {
    pub fn string<S: Into<String>>(s: S) -> Self {
        Val::Str(Rc::new(s.into()))
    }

    pub fn as_string(&self) -> String {
        match self {
            Val::Num(a) => a.to_string(),
            Val::Str(s) => s.to_string(),
        }
    }

    pub fn truth(&self) -> bool {
        match self {
            Val::Num(a) => !a.is_zero(),
            Val::Str(s) => s.len() != 0,
        }
    }
}

pub const ZERO: Val = Val::Num(dec256!(0).with_ctx(NO_TRAPS));
pub const ONE: Val = Val::Num(dec256!(1).with_ctx(NO_TRAPS));

impl fmt::Display for Val {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Val::Num(a) => write!(f, "{}", a),
            Val::Str(s) => write!(f, "{}", s),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Inst {
    Pow,
    Mod,
    IDiv,
    Gt,
    Ge,
    Ne,
    And,
    Or,
    Xor,
    Add,
    Sub,
    Mul,
    Div,
    Goto(usize),
    End,
    Const(Val),
    Print,
    Eq,
    Lt,
    Le,
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
        self.stack.pop().unwrap()
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
                Inst::Eq => {
                    let b = self.pop();
                    let a = self.pop();
                    let r = if a == b { ONE } else { ZERO };
                    self.push(r);
                }
                Inst::Lt => {
                    let b = self.pop();
                    let a = self.pop();
                    let r = match (&a, &b) {
                        (Val::Num(a), Val::Num(b)) => a < b,
                        (Val::Str(a), Val::Str(b)) => a < b,
                        _ => {
                            let a = a.as_string();
                            let b = b.as_string();
                            a < b
                        }
                    };
                    let r = if r { ONE } else { ZERO };
                    self.push(r);
                }
                Inst::Le => {
                    let b = self.pop();
                    let a = self.pop();
                    let r = match (&a, &b) {
                        (Val::Num(a), Val::Num(b)) => a <= b,
                        (Val::Str(a), Val::Str(b)) => a <= b,
                        _ => {
                            let a = a.as_string();
                            let b = b.as_string();
                            a <= b
                        }
                    };
                    let r = if r { ONE } else { ZERO };
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
                            let n = match usize::try_from(*a) {
                                Ok(n) => n,
                                Err(_) => {
                                    return Err(
                                        "*: cannot convert number to repeat count".to_string()
                                    );
                                }
                            };
                            Val::string(b.repeat(n))
                        }
                        (Val::Str(a), Val::Num(b)) => {
                            let n = match usize::try_from(*b) {
                                Ok(n) => n,
                                Err(_) => {
                                    return Err(
                                        "*: cannot convert number to repeat count".to_string()
                                    );
                                }
                            };
                            Val::string(a.repeat(n))
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
                _ => todo!(),
            }
        }
        return Ok(());
    }
}
