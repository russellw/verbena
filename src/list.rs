use crate::val::*;
use std::fmt;
use std::ops::{Index, IndexMut, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo};

#[derive(Debug, Clone)]
pub struct List {
    v: Vec<Val>,
}

impl Default for List {
    fn default() -> Self {
        Self::new()
    }
}

impl List {
    pub fn new() -> Self {
        List { v: Vec::new() }
    }

    pub fn is_empty(&self) -> bool {
        self.v.is_empty()
    }

    pub fn len(&self) -> usize {
        self.v.len()
    }

    pub fn push(&mut self, a: Val) {
        self.v.push(a)
    }

    pub fn repeat(&self, n: usize) -> List {
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
        List { v: new_vec }
    }
}

impl From<Vec<Val>> for List {
    fn from(v: Vec<Val>) -> Self {
        List { v }
    }
}

impl fmt::Display for List {
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

impl PartialEq for List {
    fn eq(&self, other: &Self) -> bool {
        // Compare by identity rather than contents
        std::ptr::eq(self, other)
    }
}

// Implement the Index trait to enable read access with [] operator
impl Index<usize> for List {
    type Output = Val;

    fn index(&self, index: usize) -> &Self::Output {
        &self.v[index]
    }
}

// Implement the IndexMut trait to enable write access with [] operator
impl IndexMut<usize> for List {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.v[index]
    }
}

// Implement range indexing
impl Index<Range<usize>> for List {
    type Output = [Val];

    fn index(&self, index: Range<usize>) -> &Self::Output {
        &self.v[index]
    }
}

impl IndexMut<Range<usize>> for List {
    fn index_mut(&mut self, index: Range<usize>) -> &mut Self::Output {
        &mut self.v[index]
    }
}

// Implement inclusive range indexing (a..=b)
impl Index<RangeInclusive<usize>> for List {
    type Output = [Val];

    fn index(&self, index: RangeInclusive<usize>) -> &Self::Output {
        &self.v[index]
    }
}

impl IndexMut<RangeInclusive<usize>> for List {
    fn index_mut(&mut self, index: RangeInclusive<usize>) -> &mut Self::Output {
        &mut self.v[index]
    }
}

// Implement from range indexing (a..)
impl Index<RangeFrom<usize>> for List {
    type Output = [Val];

    fn index(&self, index: RangeFrom<usize>) -> &Self::Output {
        &self.v[index]
    }
}

impl IndexMut<RangeFrom<usize>> for List {
    fn index_mut(&mut self, index: RangeFrom<usize>) -> &mut Self::Output {
        &mut self.v[index]
    }
}

// Implement to range indexing (..b)
impl Index<RangeTo<usize>> for List {
    type Output = [Val];

    fn index(&self, index: RangeTo<usize>) -> &Self::Output {
        &self.v[index]
    }
}

impl IndexMut<RangeTo<usize>> for List {
    fn index_mut(&mut self, index: RangeTo<usize>) -> &mut Self::Output {
        &mut self.v[index]
    }
}

// Implement full range indexing (..)
impl Index<RangeFull> for List {
    type Output = [Val];

    fn index(&self, _index: RangeFull) -> &Self::Output {
        &self.v[..]
    }
}

impl IndexMut<RangeFull> for List {
    fn index_mut(&mut self, _index: RangeFull) -> &mut Self::Output {
        &mut self.v[..]
    }
}
