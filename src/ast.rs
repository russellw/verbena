use crate::ErrorContext;
use crate::program::*;
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

    Infix(ErrorContext, Inst, Box<Expr>, Box<Expr>),
    InfixAssign(ErrorContext, Inst, Box<Expr>, Box<Expr>),
    Prefix(ErrorContext, Inst, Box<Expr>),

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
