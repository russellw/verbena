use std::fmt;

#[derive(Debug, Clone)]
pub struct Source {
    pub file: String,
    pub line: usize,
}

impl fmt::Display for Source {
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
    Assert(Source, Expr, String),
    Expr(Source, Expr),
    Global(Source, String),
    Nonlocal(Source, String),
    Return(Source, Expr),
    Label(Source, String),
    If(Source, Expr, Vec<Stmt>, Vec<Stmt>),
    While(Source, Expr, Vec<Stmt>),
    Dowhile(Source, Expr, Vec<Stmt>),
    For(Source, String, Expr, Vec<Stmt>),
    Prin(Source, Expr),
    Func(Source, String, Vec<String>, Vec<Stmt>),
}
