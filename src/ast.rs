use crate::ErrorContext;
use std::rc::Rc;

pub enum Expr {
    Int(ErrorContext, String),
    Float(ErrorContext, String),
    Str(ErrorContext, String),
    Id(ErrorContext, String),

    Call(ErrorContext, Box<Expr>, Vec<Expr>),

    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
}

pub enum PrintTerminator {
    Newline,
    Semi,
    Comma,
}

pub enum Stmt {
    Assert(ErrorContext, Expr),
    Dim(String, Expr),
    Input(String, String),
    Let(String, Expr),
    Gosub(ErrorContext, Expr),
    Goto(ErrorContext, Expr),
    Return,
    Label(ErrorContext, Expr),
    If(Expr, Vec<Stmt>, Vec<Stmt>),
    While(Expr, Vec<Stmt>),
    For(String, Expr, Expr, Expr, Vec<Stmt>),
    Print(Vec<(Expr, PrintTerminator)>),
}

pub struct AST {
    pub file: Rc<String>,
    pub text: Vec<char>,
    pub code: Vec<Stmt>,
}
