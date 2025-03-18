use std::fmt;

#[derive(Debug)]
pub struct CompileError {
    pub file: String,
    pub line: usize,
    pub message: String,
}

impl CompileError {
    fn new(file: &str, text: &Vec<char>, start: usize, message: String) -> Self {
        // Calculate line number by counting newlines up to start position
        let line = text[..start].iter().filter(|&&c| c == '\n').count() + 1;
        CompileError {
            file: file.to_string(),
            line,
            message,
        }
    }
}

impl fmt::Display for CompileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}: {}", self.file, self.line, self.message)
    }
}
