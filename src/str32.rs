use std::rc::Rc;

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Str32 {
    v: Rc<[char]>,
}

impl Str32 {
    // Constructor for a new Str32 from a string slice
    pub fn new(s: &str) -> Self {
        let chars: Vec<char> = s.chars().collect();
        Self { v: chars.into() }
    }

    // Constructor from a String
    pub fn from_string(s: String) -> Self {
        let chars: Vec<char> = s.chars().collect();
        Self { v: chars.into() }
    }

    // Constructor from a single character
    pub fn from_char(c: char) -> Self {
        Self { v: vec![c].into() }
    }

    // Constructor from a Vec<char>
    pub fn from_vec(chars: Vec<char>) -> Self {
        Self { v: chars.into() }
    }

    // Returns the length of the string in characters
    pub fn len(&self) -> usize {
        self.v.len()
    }

    // Checks if the string is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    // Accesses a character at a specific index, returning an error if out of bounds
    pub fn at(&self, i: usize) -> Result<char, String> {
        if i < self.len() {
            Ok(self.v[i])
        } else {
            Err("Index out of range".to_string())
        }
    }

    // Converts the Str32 back to a regular String
    pub fn to_string(&self) -> String {
        self.v.iter().collect()
    }

    // Creates a substring from a range of indices
    pub fn substr(&self, i: usize, j: usize) -> Self {
        let slice = &self.v[i..j];
        let chars: Vec<char> = slice.to_vec();
        Self { v: chars.into() }
    }

    // Concatenates two Str32 instances
    pub fn add(&self, b: &Self) -> Self {
        let mut r: Vec<char> = self.v.to_vec();
        r.extend(b.v.iter());
        Self { v: r.into() }
    }

    // Returns a new Str32 with all uppercase characters
    pub fn upper(&self) -> Self {
        let r: Vec<char> = self
            .v
            .iter()
            .map(|c| c.to_uppercase().next().unwrap_or(*c))
            .collect();

        Self { v: r.into() }
    }

    // Returns a new Str32 with all lowercase characters
    pub fn lower(&self) -> Self {
        let r: Vec<char> = self
            .v
            .iter()
            .map(|c| c.to_lowercase().next().unwrap_or(*c))
            .collect();

        Self { v: r.into() }
    }

    // Creates a new Str32 that repeats the current content n times
    pub fn repeat(&self, n: usize) -> Self {
        let mut r: Vec<char> = Vec::with_capacity(self.len() * n);
        for _ in 0..n {
            r.extend(self.v.iter().cloned());
        }

        Self { v: r.into() }
    }
}

// Implementing Display trait to allow printing with {}
impl std::fmt::Display for Str32 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

// Implementing Debug trait to allow printing with {:?}
impl std::fmt::Debug for Str32 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Str32({:?})", self.to_string())
    }
}
