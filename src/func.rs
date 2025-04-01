use std::rc::Rc;
use std::cell::RefCell;
use crate::env::*;
use crate::code::*;

pub struct Func {
    pub fd: FuncDef,
    pub env: Option<Rc<RefCell<Env>>>,
}
