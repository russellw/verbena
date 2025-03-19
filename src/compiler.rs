use crate::ast::*;
use crate::compile_error::*;
use std::collections::HashMap;

struct Compiler {
    ast: &Ast,
}

impl Compiler {
    fn new(ast: &Ast) -> Self {
        Compiler {}
    }

    fn compile(&mut self) -> Result<Program, CompileError> {
        Ok(Program {})
    }
}

pub fn compile(ast: &Ast) -> Result<Program, CompileError> {
    let mut compiler = Compiler::new(ast);
    compiler.compile()
}
