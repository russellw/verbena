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
