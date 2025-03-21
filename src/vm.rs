use crate::error_context::*;
use crate::program::*;
use crate::val::*;
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::Signed;
use num_traits::Zero;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;
use std::cell::RefCell;
use std::collections::HashMap;
use std::io;
use std::io::Write;
use std::rc::Rc;

/// The execution context for running a program.
///
/// Maintains the execution state including stack, variables, and control flow.
pub struct VM {
    program: Program,
    pub rng: ChaCha20Rng,
    pc: usize,
    val_stack: Vec<Val>,
    gosub_stack: Vec<usize>,
    pub vars: HashMap<String, Val>,
}

impl VM {
    /// Creates a new execution process for the given program.
    ///
    /// # Arguments
    ///
    /// * `program` - The compiled program to execute
    ///
    /// # Returns
    ///
    /// A new Process instance ready to run the program
    pub fn new(program: Program) -> Self {
        VM {
            program,
            rng: ChaCha20Rng::seed_from_u64(0),
            pc: 0,
            val_stack: Vec::new(),
            gosub_stack: Vec::new(),
            vars: HashMap::new(),
        }
    }

    fn push(&mut self, val: Val) {
        self.val_stack.push(val);
    }

    fn pop(&mut self) -> Val {
        self.val_stack.pop().unwrap()
    }

    fn top(&mut self) -> Val {
        self.val_stack.last().unwrap().clone()
    }

    fn err<S: AsRef<str>>(&self, ec: &ErrorContext, msg: S) -> String {
        format!(f, "{}:{}: {}", ec.file,ec.line, 
            msg: msg.as_ref().to_string())
    }

    /// Executes the program.
    ///
    /// Runs the program instruction by instruction until it completes or encounters an error.
    ///
    /// # Returns
    ///
    /// * `Ok(Val)` - The result of the program execution
    /// * `Err(Error)` - An error that occurred during execution
    pub fn run(&mut self) -> Result<Val, String> {
        while self.pc < self.program.code.len() {
            let inst = self.program.code[self.pc].clone();
            match inst {
                Inst::Const(a) => {
                    self.push(a.clone());
                }
                Inst::Pop => {
                    self.pop();
                }
                Inst::BrFalse(target) => {
                    let a = self.pop();
                    if !a.truth() {
                        self.pc = target;
                        continue;
                    }
                }
                Inst::DupBrFalse(target) => {
                    let a = self.top();
                    if !a.truth() {
                        self.pc = target;
                        continue;
                    }
                }
                Inst::DupBrTrue(target) => {
                    let a = self.top();
                    if a.truth() {
                        self.pc = target;
                        continue;
                    }
                }
                Inst::Load(ec, name) => {
                    let a = match self.vars.get(&name) {
                        Some(a) => a,
                        None => {
                            return Err(self.err(ec, format!("'{}' is not defined", name)));
                        }
                    };
                    self.push(a.clone());
                }
                Inst::Store(name) => {
                    let a = self.pop();
                    self.vars.insert(name.clone(), a);
                }
                Inst::Br(target) => {
                    self.pc = target;
                    continue;
                }
                Inst::Gosub(target) => {
                    self.gosub_stack.push(self.pc);
                    self.pc = target;
                    continue;
                }
                Inst::Return => {
                    self.pc = match self.gosub_stack.pop() {
                        Some(a) => a,
                        None => {
                            return Ok(Val::Int(BigInt::zero()));
                        }
                    };
                }
                Inst::Exit => {
                    let a = self.pop();
                    return Ok(a);
                }
            }
            self.pc += 1;
        }
        Ok(Val::Int(BigInt::zero()))
    }
}
