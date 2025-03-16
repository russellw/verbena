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
        result.push_str(&format!("{}:{}:\n", file, line_number));
        result.push_str(&format!("{}\n", error_line));
        result.push_str(&format!("{}^\n", " ".repeat(column)));
        result.push_str(&format!("{}\n", self.msg));
        result
    }
}
