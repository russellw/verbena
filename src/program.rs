use crate::ErrorContext;
use crate::val::*;

#[derive(Debug)]
pub enum Inst {
    // Stack & Memory Operations
    Const(Val),
    Pop,
    Load(ErrorContext, String),
    Store(String),

    // Control Flow
    Br(usize),
    BrFalse(usize),
    DupBrTrue(usize),
    DupBrFalse(usize),
    Return,
    Exit,

    Call(ErrorContext, String, usize),
}

pub struct Program {
    pub code: Vec<Inst>,
}

impl Program {
    pub fn new(code: Vec<Inst>) -> Self {
        Program { code }
    }
}
