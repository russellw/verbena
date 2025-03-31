use crate::ErrorContext;
use crate::program::*;
use std::rc::Rc;

#[derive(Debug)]
pub enum Expr {
    Num(f64),
    Str(String),
    Id(ErrorContext, String),
    True,
    False,
    Null,
    Inf,
    Nan,
    Pi,

    Infix(ErrorContext, Inst, Box<Expr>, Box<Expr>),
    InfixAssign(ErrorContext, Inst, Box<Expr>, Box<Expr>),
    Prefix(ErrorContext, Inst, Box<Expr>),

    Object(Vec<Expr>),
    List(Vec<Expr>),
    Call(ErrorContext, Box<Expr>, Vec<Expr>),
    Subscript(ErrorContext, Box<Expr>, Box<Expr>),
    Slice(ErrorContext, Box<Expr>, Box<Expr>, Box<Expr>),

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
    Dowhile(Expr, Vec<Stmt>),
    For(String, Expr, Vec<Stmt>),
    Prin(Vec<Expr>),
    Func(String, Vec<String>, Vec<Stmt>),
}

pub struct AST {
    pub file: Rc<String>,
    pub code: Vec<Stmt>,
}
