use crate::ast::*;
use crate::compile_error::*;
use crate::program::*;
use crate::val::*;
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

impl<'a> Compiler<'a> {
    fn new(ast: &'a AST) -> Self {
        Compiler {
            ast,
            tmp_count: 0,
            labels: HashMap::<String, usize>::new(),
            label_refs: Vec::<LabelRef>::new(),
            code: Vec::<Inst>::new(),
        }
    }

    fn expr(&mut self, a: &Expr) -> Result<(), CompileError> {
        match a {
            Expr::Str(s) => {
                self.code.push(Inst::Const(Val::Str(s.clone().into())));
            }
            _ => todo!(),
        }
        Ok(())
    }

    fn stmt(&mut self, a: &Stmt) -> Result<(), CompileError> {
        match a {
            Stmt::Print(ec, v) => {
                for a in v {
                    self.expr(a);
                    self.code
                        .push(Inst::Call(ec.clone(), "_print".to_string(), 1));
                }
            }
            _ => todo!(),
        }
        Ok(())
    }

    fn compile(&mut self) -> Result<Program, CompileError> {
        for a in &self.ast.code {
            self.stmt(&a)?;
        }
        Ok(Program {
            code: mem::take(&mut self.code),
        })
    }
}

pub fn compile(ast: &AST) -> Result<Program, CompileError> {
    let mut compiler = Compiler::new(ast);
    compiler.compile()
}
