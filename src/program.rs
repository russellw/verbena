use crate::ErrorContext;
use crate::val::*;

#[derive(Debug)]
pub enum Inst {
    Const(Val),
    Pop,
    Load(ErrorContext, String),
    Store(String),
    StoreAt(ErrorContext),

    Br(usize),
    BrTrue(usize),
    BrFalse(usize),
    DupBrTrue(usize),
    DupBrFalse(usize),
    Return,
    Exit,

    Add,
    Sub,
    Mul,
    IDiv,
    FDiv,
    Mod,
    Shl,
    Shr,
    BitAnd,
    BitOr,
    BitXor,
    BitNot,
    Neg,
    Not,
    Eq,
    Ne,
    Lt,
    Gt,
    Le,
    Ge,
    Pow,

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
