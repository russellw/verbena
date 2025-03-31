use crate::val::*;
use std::cell::RefCell;
use std::rc::Rc;

// Environment implements lexical scoping
// TODO: Check if a boxed array would be slightly faster
// TODO: Check if iteration would be slightly faster than recursion
pub struct Env {
    v: Vec<Val>,                     // Current scope's bindings
    outer: Option<Rc<RefCell<Env>>>, // Parent (outer) environment
}

impl Env {
    pub fn new(outer: Option<Rc<RefCell<Env>>>, n: usize) -> Self {
        Env {
            v: vec![Val::Null; n],
            outer,
        }
    }

    pub fn get(&self, level: usize, k: usize) -> Val {
        if level == 0 {
            self.v[k].clone()
        } else {
            self.outer.as_ref().unwrap().borrow().get(level - 1, k)
        }
    }

    pub fn set(&mut self, level: usize, k: usize, a: Val) {
        if level == 0 {
            self.v[k] = a;
        } else {
            self.outer
                .as_mut()
                .unwrap()
                .borrow_mut()
                .set(level - 1, k, a);
        }
    }
}
