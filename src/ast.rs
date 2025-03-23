use crate::ErrorContext;
use std::rc::Rc;

#[derive(Debug)]
pub enum Expr {
    Int(ErrorContext, String),
    Float(ErrorContext, String),
    Str(String),
    Id(ErrorContext, String),

    Call(ErrorContext, Box<Expr>, Vec<Expr>),

    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
    Assign(ErrorContext, Box<Expr>, Box<Expr>),
    OpAssign(ErrorContext, String, Box<Expr>, Box<Expr>),
}

pub enum Stmt {
    Assert(ErrorContext, Expr),
    Input(String, String),
    Let(String, Expr),
    Goto(ErrorContext, String),
    Return,
    Label(ErrorContext, String),
    If(Expr, Vec<Stmt>, Vec<Stmt>),
    While(Expr, Vec<Stmt>),
    Dowhile(Expr, Vec<Stmt>),
    For(String, Expr, Vec<Stmt>),
    Print(ErrorContext, Vec<Expr>),
}

pub struct AST {
    pub file: Rc<String>,
    pub code: Vec<Stmt>,
}
