use crate::ErrorContext;
use crate::val::*;

#[derive(Debug)]
pub enum Inst {
    // Stack & Memory Operations
    Const(Val),
    Pop,
    Load(ErrorContext, String),
    Store(String),
    StoreAt(ErrorContext),

    // Control Flow
    Br(usize),
    BrTrue(usize),
    BrFalse(usize),
    DupBrTrue(usize),
    DupBrFalse(usize),
    Return,
    Exit,

    Assert(ErrorContext, String),
    Call(ErrorContext, String, usize),
    CallIndirect(ErrorContext, usize),
}

pub struct Program {
    pub code: Vec<Inst>,
}

impl Program {
    pub fn new(code: Vec<Inst>) -> Self {
        Program { code }
    }
}

impl std::fmt::Debug for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\n")?;
        for (i, a) in self.code.iter().enumerate() {
            write!(f, "{}\t{:?}\n", i, a)?;
        }
        Ok(())
    }
}
