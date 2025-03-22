use crate::ast::*;
use crate::compile_error::*;
use crate::program::*;
use std::collections::HashMap;
use std::mem;

// GOTO and GOSUB can go forward as well as back
// That means they can only be resolved at the end, when all labels have been seen
// so need to keep track of them until then
struct LabelRef {
    // Index in the code vector
    i: usize,

    // Line number or label referred to
    label: String,
}

struct Compiler<'a> {
    ast: &'a AST,

    // Counter for generating temporary names
    tmp_count: usize,

    labels: HashMap<String, usize>,
    label_refs: Vec<LabelRef>,

    code: Vec<Inst>,
}

impl Compiler<'_> {
    fn new(ast: &AST) -> Self {
        Compiler {
            ast,
            tmp_count: 0,
            labels: HashMap::<String, usize>::new(),
            label_refs: Vec::<LabelRef>::new(),
            code: Vec::<Inst>::new(),
        }
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
