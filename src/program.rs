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

    // I/O Operations
    Input(String),

    // List Operations
    Dim(String),
    List(usize),
    StoreSubscript,
}

pub struct Program {
    pub code: Vec<Inst>,
}

impl Program {
    pub fn new(code: Vec<Inst>) -> Self {
        Program { code }
    }
}
