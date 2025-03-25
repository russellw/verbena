use std::fmt;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct ErrorContext {
    pub file: Rc<String>,
    pub line: usize,
}

impl ErrorContext {
    pub fn new(file: Rc<String>, line: usize) -> Self {
        ErrorContext { file, line }
    }
}

impl fmt::Display for ErrorContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.file, self.line)
    }
}
