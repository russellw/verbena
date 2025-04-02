use crate::code::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ErrorContext {
    pub file: String,
    pub line: usize,
}

impl fmt::Display for ErrorContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.file, self.line)
    }
}

#[derive(Debug)]
pub enum Expr {
    Atom(String),
    Object(Vec<Expr>),
    List(Vec<Expr>),

    Call(Box<Expr>, Vec<Expr>),
    Subscript( Box<Expr>, Box<Expr>),
    Slice( Box<Expr>, Box<Expr>, Box<Expr>),

    Prefix( String, Box<Expr>),

    Infix( String, Box<Expr>, Box<Expr>),
    Assign( String, Box<Expr>, Box<Expr>),
}

#[derive(Debug)]
pub enum Stmt {
    Assert(ErrorContext, Expr, String),
    Expr(ErrorContext, Expr),
    Global(ErrorContext, String),
    Nonlocal(ErrorContext, String),
    Return(ErrorContext, Expr),
    Label(ErrorContext, String),
    If(ErrorContext, Expr, Vec<Stmt>, Vec<Stmt>),
    While(ErrorContext, Expr, Vec<Stmt>),
    Dowhile(ErrorContext, Expr, Vec<Stmt>),
    For(ErrorContext, String, Expr, Vec<Stmt>),
    Prin(ErrorContext, Expr),
    Func(ErrorContext, String, Vec<String>, Vec<Stmt>),
}
