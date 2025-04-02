use std::fmt;
use std::rc::Rc;

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
