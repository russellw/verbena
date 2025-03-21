use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct ErrorContext {
    pub file: Rc<String>,
    pub line: usize,
}

impl ErrorContext {
    pub fn new(file: Rc<String>, text: &Vec<char>, start: usize) -> Self {
        // Calculate line number by counting newlines up to start position
        let line = text[..start].iter().filter(|&&c| c == '\n').count() + 1;
        ErrorContext { file, line }
    }
}
