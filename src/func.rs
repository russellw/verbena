use crate::code::*;
use crate::env::*;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Func {
    pub env: Option<Rc<RefCell<Env>>>,
    pub fd: Rc<FuncDef>,
}

impl Func {
    pub fn new(outer: Option<Rc<RefCell<Env>>>, fd: Rc<FuncDef>) -> Self {
        let env = Env::new(outer, fd.params);
        let env = Rc::new(RefCell::new(env));
        let env = Some(env);
        Func { env, fd }
    }
}
