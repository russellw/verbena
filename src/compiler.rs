use crate::ast::*;
use crate::code::*;
use crate::error_context::*;
use crate::val::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::mem;
use std::rc::Rc;

// Compile time lexical scope environment mirrors run time
struct Env {
    outer: Option<Rc<RefCell<Env>>>,
    m: HashMap<String, usize>,
}

impl Env {
    pub fn new(outer: Option<Rc<RefCell<Env>>>) -> Self {
        Env {
            outer,
            m: HashMap::new(),
        }
    }

    pub fn get(&self, k: &str) -> Option<usize> {
        match self.m.get(k) {
            Some(x) => Some(x.clone()),
            None => match &self.outer {
                Some(outer) => outer.borrow().get(k),
                None => None,
            },
        }
    }

    pub fn set(&mut self, k: String, x: usize) {
        self.m.insert(k, x);
    }
}

// Branches can go forward as well as back
// so can only be resolved at the end, when all labels have been seen
// so need to keep track until then
struct Branch {
    // A branch might find no corresponding label
    // and need to report error
    ec: ErrorContext,

    // Index in the code vectors
    i: usize,

    // Label referred to
    // This may be a label explicitly written by the programmer
    // or automatically generated by the compiler
    label: String,
}

struct Compiler {
    globals: HashSet<String>,
    nonlocals: HashSet<String>,
    assigned: HashSet<String>,

    tmp_count: usize,

    branches: Vec<Branch>,
    labels: HashMap<String, usize>,

    insts: Vec<Inst>,
    ecs: Vec<ErrorContext>,
}

impl Compiler {
    fn new() -> Self {
        Compiler {
            globals: HashSet::<String>::new(),
            nonlocals: HashSet::<String>::new(),
            assigned: HashSet::<String>::new(),
            tmp_count: 0,
            labels: HashMap::<String, usize>::new(),
            branches: Vec::<Branch>::new(),
            insts: Vec::<Inst>::new(),
            ecs: Vec::<ErrorContext>::new(),
        }
    }

    fn tmp(&mut self) -> String {
        self.tmp_count += 1;

        // Temporary names cannot clash with any valid identifier
        format!(" {}", self.tmp_count)
    }

    // Declare variables
    fn decl_expr(&mut self, a: &Expr) {
        match a {
            Expr::Call(ec, f, args) => {
                self.decl_expr(f);
                for a in args {
                    self.decl_expr(a);
                }
            }
            Expr::List(v) => {
                for a in v {
                    self.decl_expr(a);
                }
            }
            Expr::Object(v) => {
                for a in v {
                    self.decl_expr(a);
                }
            }
            Expr::Subscript(ec, a, i) => {
                self.decl_expr(a);
                self.decl_expr(i);
            }
            Expr::Slice(ec, a, i, j) => {
                self.decl_expr(a);
                self.decl_expr(i);
                self.decl_expr(j);
            }
            Expr::Infix(ec, inst, a, b) => {
                self.decl_expr(a);
                self.decl_expr(b);
            }
            Expr::Prefix(ec, inst, a) => {
                self.decl_expr(a);
            }
            Expr::InfixAssign(ec, inst, a, b) => match &**a {
                Expr::Id(_ec, name) => {
                    self.assigned.insert(name.to_string());
                    self.decl_expr(b);
                }
                Expr::Subscript(ec, a, i) => {
                    self.decl_expr(a);
                    self.decl_expr(i);
                    self.decl_expr(b);
                }
                _ => {}
            },
            Expr::Assign(ec, a, b) => match &**a {
                Expr::Id(_ec, name) => {
                    self.assigned.insert(name.to_string());
                    self.decl_expr(b);
                }
                Expr::Subscript(ec, a, i) => {
                    self.decl_expr(a);
                    self.decl_expr(i);
                    self.decl_expr(b);
                }
                _ => {}
            },
            Expr::Or(a, b) => {
                self.decl_expr(a);
                self.decl_expr(b);
            }
            Expr::And(a, b) => {
                self.decl_expr(a);
                self.decl_expr(b);
            }
            _ => {}
        }
    }

    fn decl_stmt(&mut self, a: &Stmt) -> Result<(), String> {
        Ok(())
    }

    // Generate code
    fn add(&mut self, ec: &ErrorContext, inst: Inst) {
        self.insts.push(inst);
        self.ecs.push(ec.clone());
    }

    fn branch(&mut self, ec: &ErrorContext, br: Inst, label: &str) {
        self.branches.push(Branch {
            ec: ec.clone(),
            i: self.insts.len(),
            label: label.to_string(),
        });
        self.add(ec, br);
    }

    fn expr(&mut self, a: &Expr) -> Result<(), String> {
        match a {
            Expr::True => {
                self.add(&ErrorContext::blank(), Inst::Const(Val::True));
            }
            Expr::False => {
                self.add(&ErrorContext::blank(), Inst::Const(Val::False));
            }
            Expr::Null => {
                self.add(&ErrorContext::blank(), Inst::Const(Val::Null));
            }
            Expr::Inf => {
                self.add(&ErrorContext::blank(), Inst::Const(Val::Num(f64::INFINITY)));
            }
            Expr::Nan => {
                self.add(&ErrorContext::blank(), Inst::Const(Val::Num(f64::NAN)));
            }
            Expr::Pi => {
                self.add(
                    &ErrorContext::blank(),
                    Inst::Const(Val::Num(std::f64::consts::PI)),
                );
            }
            Expr::Str(s) => {
                self.add(&ErrorContext::blank(), Inst::Const(Val::Str(s.clone())));
            }
            Expr::Num(a) => {
                self.add(&ErrorContext::blank(), Inst::Const(Val::Num(*a)));
            }
            Expr::Call(ec, f, args) => {
                self.expr(f)?;
                for a in args {
                    self.expr(a)?;
                }
                self.add(ec, Inst::Call(args.len()));
            }
            Expr::List(v) => {
                for a in v {
                    self.expr(a)?;
                }
                self.add(&ErrorContext::blank(), Inst::List(v.len()));
            }
            Expr::Object(v) => {
                for a in v {
                    self.expr(a)?;
                }
                self.add(&ErrorContext::blank(), Inst::Object(v.len()));
            }
            Expr::Subscript(ec, a, i) => {
                self.expr(a)?;
                self.expr(i)?;
                self.add(ec, Inst::Subscript);
            }
            Expr::Slice(ec, a, i, j) => {
                self.expr(a)?;
                self.expr(i)?;
                self.expr(j)?;
                self.add(ec, Inst::Slice);
            }
            Expr::Id(ec, name) => {
                self.add(ec, Inst::LoadGlobal(name.to_string()));
            }
            Expr::Infix(ec, inst, a, b) => {
                self.expr(a)?;
                self.expr(b)?;
                self.add(ec, inst.clone());
            }
            Expr::Prefix(ec, inst, a) => {
                self.expr(a)?;
                self.add(ec, inst.clone());
            }
            Expr::InfixAssign(ec, inst, a, b) => match &**a {
                Expr::Id(_ec, name) => {
                    self.expr(a)?;
                    self.expr(b)?;
                    self.add(ec, inst.clone());
                    self.add(ec, Inst::StoreGlobal(name.to_string()));
                }
                Expr::Subscript(ec, a, i) => {
                    self.expr(a)?;
                    self.expr(i)?;
                    self.add(ec, Inst::Dup2Subscript);
                    self.expr(b)?;
                    self.add(ec, inst.clone());
                    self.add(ec, Inst::StoreAt);
                }
                _ => {
                    return Err(format!("{}: Expected lvalue", ec.clone()));
                }
            },
            Expr::Assign(ec, a, b) => match &**a {
                Expr::Id(_ec, name) => {
                    self.expr(b)?;
                    self.add(ec, Inst::StoreGlobal(name.to_string()));
                }
                Expr::Subscript(ec, a, i) => {
                    self.expr(a)?;
                    self.expr(i)?;
                    self.expr(b)?;
                    self.add(ec, Inst::StoreAt);
                }
                _ => {
                    return Err(format!("{}: Expected lvalue", ec.clone()));
                }
            },
            Expr::Or(a, b) => {
                self.expr(a)?;
                let after_label = self.tmp();
                self.branch(&ErrorContext::blank(), Inst::DupBrTrue(0), &after_label);
                self.add(&ErrorContext::blank(), Inst::Pop);
                self.expr(b)?;
                self.labels.insert(after_label, self.insts.len());
            }
            Expr::And(a, b) => {
                self.expr(a)?;
                let after_label = self.tmp();
                self.branch(&ErrorContext::blank(), Inst::DupBrFalse(0), &after_label);
                self.add(&ErrorContext::blank(), Inst::Pop);
                self.expr(b)?;
                self.labels.insert(after_label, self.insts.len());
            }
            _ => {
                eprintln!("{:?}", a);
                todo!();
            }
        }
        Ok(())
    }

    fn stmt(&mut self, a: &Stmt) -> Result<(), String> {
        match a {
            Stmt::Global(_, _) | Stmt::Nonlocal(_, _) => {}
            Stmt::If(cond, yes, no) => {
                // Condition
                self.expr(cond)?;
                let else_label = self.tmp();
                self.branch(&ErrorContext::blank(), Inst::BrFalse(0), &else_label);

                // Then
                self.block(yes)?;
                let after_label = self.tmp();
                self.branch(&ErrorContext::blank(), Inst::Br(0), &after_label);

                // Else
                self.labels.insert(else_label, self.insts.len());
                self.block(no)?;

                // After
                self.labels.insert(after_label, self.insts.len());
            }
            Stmt::Assert(ec, cond, msg) => {
                self.expr(cond)?;
                self.add(ec, Inst::Assert(msg.to_string()));
            }
            Stmt::Prin(v) => {
                for a in v {
                    self.expr(a)?;
                    self.add(&ErrorContext::blank(), Inst::Prin);
                }
            }
            Stmt::Label(ec, s) => {
                if self
                    .labels
                    .insert(s.to_string(), self.insts.len())
                    .is_some()
                {
                    return Err(format!("{}: '{}' was already defined", ec.clone(), s));
                }
            }
            Stmt::Goto(ec, label) => self.branch(ec, Inst::Br(0), label),
            Stmt::Dowhile(cond, body) => {
                // Body
                let loop_label = self.tmp();
                self.labels.insert(loop_label.clone(), self.insts.len());
                self.block(body)?;

                // Condition
                self.expr(cond)?;
                self.branch(&ErrorContext::blank(), Inst::BrTrue(0), &loop_label);
            }
            Stmt::While(cond, body) => {
                // Bypass
                let cond_label = self.tmp();
                self.branch(&ErrorContext::blank(), Inst::Br(0), &cond_label);

                // Body
                let loop_label = self.tmp();
                self.labels.insert(loop_label.clone(), self.insts.len());
                self.block(body)?;

                // Condition
                self.labels.insert(cond_label, self.insts.len());
                self.expr(cond)?;
                self.branch(&ErrorContext::blank(), Inst::BrTrue(0), &loop_label);
            }
            Stmt::Expr(a) => {
                self.expr(a)?;
                self.add(&ErrorContext::blank(), Inst::Pop);
            }
            _ => {
                eprintln!("{:?}", a);
                todo!();
            }
        }
        Ok(())
    }

    fn block(&mut self, v: &Vec<Stmt>) -> Result<(), String> {
        for a in v {
            self.stmt(a)?;
        }
        Ok(())
    }

    fn compile(&mut self, ast: &Vec<Stmt>) -> Result<FuncDef, String> {
        // Generate code
        self.block(ast)?;
        self.add(&ErrorContext::blank(), Inst::Const(Val::Null));
        self.add(&ErrorContext::blank(), Inst::Return);

        // Resolve branches
        for branch in &self.branches {
            let target = match self.labels.get(&branch.label) {
                Some(i) => *i,
                None => {
                    return Err(format!(
                        "{}: '{}' not found",
                        branch.ec.clone(),
                        branch.label
                    ));
                }
            };
            let br = &self.insts[branch.i];
            self.insts[branch.i] = match br {
                Inst::Br(_) => Inst::Br(target),
                Inst::DupBrFalse(_) => Inst::DupBrFalse(target),
                Inst::BrFalse(_) => Inst::BrFalse(target),
                Inst::DupBrTrue(_) => Inst::DupBrTrue(target),
                Inst::BrTrue(_) => Inst::BrTrue(target),
                _ => {
                    eprintln!("{:?}", br);
                    todo!();
                }
            };
        }

        Ok(FuncDef {
            params: 0,
            insts: mem::take(&mut self.insts),
            ecs: mem::take(&mut self.ecs),
        })
    }
}

pub fn func(params: Vec<String>, body: &Vec<Stmt>) -> Result<FuncDef, String> {
    todo!()
}

pub fn compile(ast: &Vec<Stmt>) -> Result<FuncDef, String> {
    let mut compiler = Compiler::new();
    compiler.compile(ast)
}
