use crate::ErrorContext;
use crate::val::*;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum Inst {
    Const(Val),
    Pop,
    LoadGlobal(String),
    StoreGlobal(String),
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
    Lambda(Rc<FuncDef>),
}

pub struct FuncDef {
    pub params: usize,

    // The last instruction is guaranteed to be Return or other terminator
    // so the interpreter does not need to worry about falling off the end
    pub insts: Vec<Inst>,

    // The vector of error contexts runs in parallel with the vector of instructions
    // to provide necessary information to the user when an error occurs
    // without wasting cache on rarely-used data in normal operation
    pub ecs: Vec<ErrorContext>,
}

impl std::fmt::Debug for FuncDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        for (i, a) in self.insts.iter().enumerate() {
            writeln!(f, "{}\t{:?}", i, a)?;
        }
        Ok(())
    }
}
