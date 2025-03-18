/// An error that occurred during parsing or processing.
///
/// Contains the position where the error occurred (`caret`) and a descriptive message (`msg`).
#[derive(Debug)]
pub struct VError {
    /// The character position where the error occurred in the input.
    pub caret: usize,
    /// A human-readable error message describing the problem.
    pub msg: String,
}

impl VError {
    /// Formats the error into a user-friendly multi-line string with context.
    ///
    /// # Arguments
    ///
    /// * `file` - The name of the file where the error occurred
    /// * `text` - The input text being processed when the error occurred
    ///
    /// # Returns
    ///
    /// A formatted error message that includes:
    /// - The file and line number where the error occurred
    /// - The line of text containing the error
    /// - A caret (^) pointing to the exact position of the error
    /// - The error message
    pub fn format_error(&self, file: &str, text: &str) -> String {
        let chars: Vec<char> = text.chars().collect();

        // Calculate line number by counting newlines up to caret position
        let line_number = chars[..self.caret].iter().filter(|&&c| c == '\n').count() + 1;

        // Find the start and end indices of the error line
        let line_start = chars[..self.caret]
            .iter()
            .rposition(|&c| c == '\n')
            .map_or(0, |pos| pos + 1);

        let line_end = chars[self.caret..]
            .iter()
            .position(|&c| c == '\n')
            .map_or(chars.len(), |pos| self.caret + pos);

        // Extract the line containing the error
        let error_line: String = chars[line_start..line_end].iter().collect();

        // Calculate column position within the line
        let column = self.caret - line_start;

        // Build the multi-line error message
        let mut result = String::new();
        result.push_str(&format!("{}:{}:\n", file, line_number));
        result.push_str(&format!("{}\n", error_line));
        result.push_str(&format!("{}^\n", " ".repeat(column)));
        result.push_str(&self.msg.to_string());
        result
    }
}
