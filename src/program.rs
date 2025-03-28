use crate::ErrorContext;
use crate::val::*;

#[derive(Debug)]
pub enum Inst {
    Const(Val),
    Pop,
    Load(String),
    Store(String),
    StoreAt,

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

    Assert(String),
    Call(String, usize),
    CallIndirect(usize),
}

pub struct Program {
    pub code: Vec<Inst>,

    // The vector of error contexts runs in parallel with the vector of instructions
    // to provide necessary information to the user when an error occurs
    // without wasting cache on rarely-used data in normal operation
    pub ecs: Vec<ErrorContext>,
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
