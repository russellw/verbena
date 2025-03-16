pub fn current_line(text: &[char], caret: usize) -> (usize, usize) {
    assert!(caret <= text.len());

    let mut i = caret;
    while 0 < i && text[i - 1] != '\n' {
        i -= 1;
    }

    let mut j = caret;
    while j < text.len() && text[j] != '\n' {
        j += 1;
    }

    (i, j)
}

pub struct Error {
    pub caret: usize,
    pub msg: String,
}

impl Error {
    pub fn format_error(&self, file: &str, input_text: &[char]) -> String {
        // Calculate line number by counting newlines up to caret position
        let line_number = input_text[..self.caret]
            .iter()
            .filter(|&&c| c == '\n')
            .count()
            + 1;

        // Find the start and end indices of the error line
        let line_start = input_text[..self.caret]
            .iter()
            .rposition(|&c| c == '\n')
            .map_or(0, |pos| pos + 1);

        let line_end = input_text[self.caret..]
            .iter()
            .position(|&c| c == '\n')
            .map_or(input_text.len(), |pos| self.caret + pos);

        // Extract the line containing the error
        let error_line: String = input_text[line_start..line_end].iter().collect();

        // Calculate column position within the line
        let column = self.caret - line_start;

        // Build the multi-line error message
        let mut result = String::new();

        // First line: error message and location
        result.push_str(&format!("Error: {}\n", self.msg));

        // Second line: line number and the actual line content
        result.push_str(&format!("Line {}: {}\n", line_number, error_line));

        // Third line: caret pointing to the error position
        result.push_str(&format!("{}^\n", " ".repeat(column + 8)));

        result
    }
}
