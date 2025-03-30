use crate::ErrorContext;
use crate::val::*;

#[derive(Debug, Clone)]
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
    Div,
    Mod,
    Shl,
    LShr,
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
    Call(usize),
    Object(usize),
    List(usize),
    Subscript,
    Dup2Subscript,
    Slice,
    Prin,
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
        writeln!(f)?;
        for (i, a) in self.code.iter().enumerate() {
            writeln!(f, "{}\t{:?}", i, a)?;
        }
        Ok(())
    }
}
