use crate::ErrorContext;
use std::fmt;

#[derive(Debug)]
pub struct CompileError {
    pub ec: ErrorContext,
    pub msg: String,
}

impl CompileError {
    pub fn new(ec: ErrorContext, msg: String) -> Self {
        CompileError { ec, msg }
    }
}

impl fmt::Display for CompileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.ec, self.msg)
    }
}
