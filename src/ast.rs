pub enum Expr {
    Int(ErrorContext, String),
    Float(ErrorContext, String),
    Str(ErrorContext, String),
    Id(ErrorContext, String),

    Not(Box<Expr>),
    Neg(Box<Expr>),
    BitNot(Box<Expr>),

    Pow(Box<Expr>, Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Mod(Box<Expr>, Box<Expr>),
    IDiv(Box<Expr>, Box<Expr>),
    FDiv(Box<Expr>, Box<Expr>),
    Shl(Box<Expr>, Box<Expr>),
    Shr(Box<Expr>, Box<Expr>),
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
    Eq(Box<Expr>, Box<Expr>),
    Ne(Box<Expr>, Box<Expr>),
    Lt(Box<Expr>, Box<Expr>),
    Gt(Box<Expr>, Box<Expr>),
    Le(Box<Expr>, Box<Expr>),
    Ge(Box<Expr>, Box<Expr>),

    Call(Box<Expr>, Vec<Expr>),
    List(Vec<Expr>),
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
    pub file: String,
    pub text: Vec<char>,
    pub code: Vec<Stmt>,
}
