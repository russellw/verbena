use std::fmt;

#[derive(Debug)]
pub struct CompileError {
    pub file: String,
    pub line: usize,
    pub msg: String,
}

impl CompileError {
    pub fn new(file: String, text: &Vec<char>, start: usize, msg: String) -> Self {
        // Calculate line number by counting newlines up to start position
        let line = text[..start].iter().filter(|&&c| c == '\n').count() + 1;
        CompileError { file, line, msg }
    }
}

impl fmt::Display for CompileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}: {}", self.file, self.line, self.msg)
    }
}
