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
    Sub(ErrorContext),
    Mul(ErrorContext),
    IDiv(ErrorContext),
    FDiv(ErrorContext),
    Mod(ErrorContext),
    Shl(ErrorContext),
    Shr(ErrorContext),
    BitAnd(ErrorContext),
    BitOr(ErrorContext),
    BitXor(ErrorContext),
    BitNot(ErrorContext),
    Neg(ErrorContext),
    Not,
    Eq,
    Ne,
    Lt,
    Gt,
    Le,
    Ge,
    Pow(ErrorContext),

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
