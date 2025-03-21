use crate::ErrorContext;
use std::rc::Rc;

pub enum Expr {
    Int(ErrorContext, String),
    Float(ErrorContext, String),
    Str(String),
    Id(ErrorContext, String),

    Call(ErrorContext, Box<Expr>, Vec<Expr>),

    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
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
    DoWhile(Expr, Vec<Stmt>),
    For(String, Expr, Expr, Expr, Vec<Stmt>),
    Print(Vec<Expr>),
}

pub struct AST {
    pub file: Rc<String>,
    pub text: Vec<char>,
    pub code: Vec<Stmt>,
}
