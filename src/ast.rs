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

#[derive(Debug)]
pub enum Expr {
    Atom(String),
    Object(Vec<Expr>),
    List(Vec<Expr>),

    Call(Box<Expr>, Vec<Expr>),
    Subscript(Box<Expr>, Box<Expr>),
    Slice(Box<Expr>, Box<Expr>, Box<Expr>),

    Prefix(String, Box<Expr>),

    Infix(String, Box<Expr>, Box<Expr>),
    Assign(String, Box<Expr>, Box<Expr>),
}

#[derive(Debug)]
pub enum Stmt {
    Assert(Src, Expr, String),
    Expr(Src, Expr),
    Return(Src, Expr),
    Label(Src, String),
    If(Src, Expr, Vec<Stmt>, Vec<Stmt>),
    While(Src, Expr, Vec<Stmt>),
    Dowhile(Src, Expr, Vec<Stmt>),
    For(Src, String, Expr, Vec<Stmt>),
    Prin(Src, Expr),
    Func(Src, String, Vec<String>, Vec<String>, Vec<Stmt>),
}
