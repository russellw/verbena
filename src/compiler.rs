use crate::ast::*;
use crate::compile_error::*;
use crate::program::*;
use std::collections::HashMap;

struct Compiler<'a> {
    ast: &'a Ast,

    // Counter for generating temporary names
    tmp_count: usize,

    labels: HashMap<Tok, usize>,
    label_refs: Vec<LabelRef>,

    carets: Vec<usize>,
    code: Vec<Inst>,
}

impl Compiler<'_> {
    fn new(ast: &Ast) -> Self {
        Compiler { ast }
    }

    fn compile(&mut self) -> Result<Program, CompileError> {
        Ok(Program {
            carets: mem::take(&mut self.carets),
            code: mem::take(&mut self.code),
        })
    }
}

pub fn compile(ast: &Ast) -> Result<Program, CompileError> {
    let mut compiler = Compiler::new(ast);
    compiler.compile()
}
