use std::fmt;

#[derive(Debug)]
pub struct ErrorContext {
    pub file: String,
    pub line: usize,
}

impl ErrorContext {
    pub fn new(file: String, text: &Vec<char>, start: usize) -> Self {
        // Calculate line number by counting newlines up to start position
        let line = text[..start].iter().filter(|&&c| c == '\n').count() + 1;
        ErrorContext { file, line }
    }
}
