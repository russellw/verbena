use crate::ast::*;
use crate::compile_error::*;
use crate::program::*;
use std::collections::HashMap;

struct Compiler<'a> {
    ast: &'a Ast,
}

impl Compiler<'_> {
    fn new(ast: &Ast) -> Self {
        Compiler { ast }
    }

    fn compile(&mut self) -> Result<Program, CompileError> {
        Ok(Program {})
    }
}

pub fn compile(ast: &Ast) -> Result<Program, CompileError> {
    let mut compiler = Compiler::new(ast);
    compiler.compile()
}
