use crate::ErrorContext;
use crate::str32::*;
use std::rc::Rc;

#[derive(Debug)]
pub enum Expr {
    Int(ErrorContext, String),
    Float(ErrorContext, String),
    Str(Str32),
    Id(ErrorContext, String),
    True,
    False,
    Null,

    Add(Box<Expr>, Box<Expr>),
    Sub(ErrorContext, Box<Expr>, Box<Expr>),
    Mul(ErrorContext, Box<Expr>, Box<Expr>),
    IDiv(ErrorContext, Box<Expr>, Box<Expr>),
    FDiv(ErrorContext, Box<Expr>, Box<Expr>),
    Mod(ErrorContext, Box<Expr>, Box<Expr>),
    Shl(ErrorContext, Box<Expr>, Box<Expr>),
    Shr(ErrorContext, Box<Expr>, Box<Expr>),
    BitAnd(ErrorContext, Box<Expr>, Box<Expr>),
    BitOr(ErrorContext, Box<Expr>, Box<Expr>),
    BitXor(ErrorContext, Box<Expr>, Box<Expr>),
    BitNot(ErrorContext, Box<Expr>),
    Neg(ErrorContext, Box<Expr>),
    Not(Box<Expr>),
    Eq(Box<Expr>, Box<Expr>),
    Ne(Box<Expr>, Box<Expr>),
    Lt(Box<Expr>, Box<Expr>),
    Gt(Box<Expr>, Box<Expr>),
    Le(Box<Expr>, Box<Expr>),
    Ge(Box<Expr>, Box<Expr>),
    Pow(ErrorContext, Box<Expr>, Box<Expr>),

    Call(ErrorContext, Box<Expr>, Vec<Expr>),

    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
    Assign(ErrorContext, Box<Expr>, Box<Expr>),
    OpAssign(ErrorContext, String, Box<Expr>, Box<Expr>),
}

#[derive(Debug)]
pub enum Stmt {
    Assert(ErrorContext, Expr, String),
    Expr(Expr),
    Goto(ErrorContext, String),
    Return(Expr),
    Label(ErrorContext, String),
    If(Expr, Vec<Stmt>, Vec<Stmt>),
    While(Expr, Vec<Stmt>),
    DoWhile(Expr, Vec<Stmt>),
    For(String, Expr, Vec<Stmt>),
    Print(ErrorContext, Vec<Expr>),
}

pub struct AST {
    pub file: Rc<String>,
    pub code: Vec<Stmt>,
}
