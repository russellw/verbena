use crate::ast::*;
use crate::compile_error::*;
use crate::program::*;
use std::collections::HashMap;

struct Compiler<'a> {
    ast: &'a AST,

    // Counter for generating temporary names
    tmp_count: usize,

    labels: HashMap<Tok, usize>,
    label_refs: Vec<LabelRef>,

    code: Vec<Inst>,
}

impl Compiler<'_> {
    fn new(ast: &AST) -> Self {
        Compiler { ast }
    }

    fn compile(&mut self) -> Result<Program, CompileError> {
        Ok(Program {
            code: mem::take(&mut self.code),
        })
    }
}

pub fn compile(ast: &AST) -> Result<Program, CompileError> {
    let mut compiler = Compiler::new(ast);
    compiler.compile()
}
