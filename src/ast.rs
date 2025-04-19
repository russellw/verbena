use std::collections::HashSet;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Src {
    pub file: String,
    pub line: usize,
}

impl fmt::Display for Src {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.file, self.line)
    }
}

#[derive(Debug, Clone)]
pub enum Expr {
    Assign(Box<Expr>, Box<Expr>),
    Atom(String),
    Call(Box<Expr>, Vec<Expr>),
    Infix(String, Box<Expr>, Box<Expr>),
    List(Vec<Expr>),
    Object(Vec<Expr>),
    Prefix(String, Box<Expr>),
    Slice(Box<Expr>, Box<Expr>, Box<Expr>),
    Subscript(Box<Expr>, Box<Expr>),
    Typeof(Box<Expr>),
}

#[derive(Debug)]
pub enum Stmt {
    Assert(Src, Expr, String),
    Case(Src, Expr, Vec<(Vec<Expr>, Vec<Stmt>)>),
    Dowhile(Src, Expr, Vec<Stmt>),
    Expr(Src, Expr),
    For(Src, String, Expr, Vec<Stmt>),
    For2(Src, String, String, Expr, Vec<Stmt>),
    Func(Src, String, Vec<String>, HashSet<String>, Vec<Stmt>),
    If(Src, Expr, Vec<Stmt>, Vec<Stmt>),
    Label(Src, String),
    Return(Src, Expr),
    Throw(Src, Expr),
    Try(Src, Vec<Stmt>, String, Vec<Stmt>),
    While(Src, Expr, Vec<Stmt>),
}
