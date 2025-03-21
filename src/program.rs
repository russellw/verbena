use crate::val::*;

#[derive(Debug, Clone)]
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
    Gosub(usize),
    Return,
    Exit,
    Assert(ErrorContext),
}

pub struct Program {
    pub code: Vec<Inst>,
}

impl Program {
    pub fn new(code: Vec<Inst>) -> Self {
        Program { code }
    }
}
