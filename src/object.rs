use crate::val::*;
use std::fmt;

#[derive(Debug)]
pub struct Object {
    pub v: Vec<Val>,
}

impl Object {
    pub fn new(n: usize) -> Self {
        Object {
            v: vec![Val::Null; n],
        }
    }

    pub fn repeat(&self, n: usize) -> Object {
        // Calculate the new capacity needed
        let new_capacity = self.v.len() * n;

        // Create a new vector with the calculated capacity
        let mut new_vec = Vec::with_capacity(new_capacity);

        // Repeat the elements n times
        for _ in 0..n {
            // Extend the new vector with clones of the original elements
            new_vec.extend(self.v.iter().cloned());
        }

        // Return the new list
        Object { v: new_vec }
    }
}

impl From<Vec<Val>> for Object {
    fn from(v: Vec<Val>) -> Self {
        Object { v }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for (i, a) in self.v.iter().enumerate() {
            if 0 < i {
                write!(f, ", ")?;
            }
            write!(f, "{}", a)?;
        }
        write!(f, "]")
    }
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        // Compare by identity rather than contents
        std::ptr::eq(self, other)
    }
}
