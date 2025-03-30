use crate::val::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Object {
    m: HashMap<String, Val>,
}

impl Default for Object {
    fn default() -> Self {
        Self::new()
    }
}

impl Object {
    pub fn new() -> Self {
        Object { m: HashMap::new() }
    }

    pub fn is_empty(&self) -> bool {
        self.m.is_empty()
    }

    pub fn len(&self) -> usize {
        self.m.len()
    }
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        // Compare by identity rather than contents
        std::ptr::eq(self, other)
    }
}
