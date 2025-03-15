pub struct Error {
    pub caret: usize,
    pub msg: String,
}

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
