use crate::ast::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::mem;
use std::rc::Rc;

struct Env {
    outer: Option<Rc<RefCell<Env>>>,
    m: HashMap<String, usize>,
}

impl Env {
    // TODO: don't need pub?
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

// Compiler is instantiated separately for each nested function
struct Compiler {
    env: Option<Rc<RefCell<Env>>>,

    globals: HashSet<String>,
    nonlocals: HashSet<String>,
    assigned: HashSet<String>,

    tmp_count: usize,
}

impl Compiler {
    fn new(env: Option<Rc<RefCell<Env>>>) -> Self {
        Compiler {
            env,
            globals: HashSet::<String>::new(),
            nonlocals: HashSet::<String>::new(),
            assigned: HashSet::<String>::new(),
            tmp_count: 0,
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
            Expr::Call(_ec, f, args) => {
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
            Expr::Subscript(_ec, a, i) => {
                self.decl_expr(a);
                self.decl_expr(i);
            }
            Expr::Slice(_ec, a, i, j) => {
                self.decl_expr(a);
                self.decl_expr(i);
                self.decl_expr(j);
            }
            Expr::Infix(_ec, _inst, a, b) => {
                self.decl_expr(a);
                self.decl_expr(b);
            }
            Expr::Prefix(_ec, _inst, a) => {
                self.decl_expr(a);
            }
            Expr::InfixAssign(_ec, _inst, a, b) => match &**a {
                Expr::Id(_ec, name) => {
                    self.assigned.insert(name.to_string());
                    self.decl_expr(b);
                }
                Expr::Subscript(_ec, a, i) => {
                    self.decl_expr(a);
                    self.decl_expr(i);
                    self.decl_expr(b);
                }
                _ => {}
            },
            Expr::Assign(_ec, a, b) => match &**a {
                Expr::Id(_ec, name) => {
                    self.assigned.insert(name.to_string());
                    self.decl_expr(b);
                }
                Expr::Subscript(_ec, a, i) => {
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
        match a {
            Stmt::Global(src, name) => {
                if self.nonlocals.contains(name) {
                    return Err(format!("{}: '{}' cannot be global and nonlocal", src, name));
                }
                self.globals.insert(name.to_string());
            }
            Stmt::Nonlocal(src, name) => {
                if self.globals.contains(name) {
                    return Err(format!("{}: '{}' cannot be global and nonlocal", src, name));
                }
                self.nonlocals.insert(name.to_string());
            }
            Stmt::If(cond, yes, no) => {
                self.decl_expr(cond);
                self.decl_block(yes)?;
                self.decl_block(no)?;
            }
            Stmt::Assert(_ec, cond, _msg) => {
                self.decl_expr(cond);
            }
            Stmt::Prin(a) => {
                self.decl_expr(a);
            }
            Stmt::Dowhile(cond, body) => {
                self.decl_block(body)?;
                self.decl_expr(cond);
            }
            Stmt::While(cond, body) => {
                self.decl_block(body)?;
                self.decl_expr(cond);
            }
            Stmt::Expr(a) => {
                self.decl_expr(a);
            }
            _ => {}
        }
        Ok(())
    }

    fn decl_block(&mut self, v: &Vec<Stmt>) -> Result<(), String> {
        for a in v {
            self.decl_stmt(a)?;
        }
        Ok(())
    }

    // Generate code
    fn add(&mut self, src: &Src, inst: Inst) {
        self.insts.push(inst);
        self.ecs.push(src.clone());
    }

    fn branch(&mut self, src: &Src, br: Inst, label: &str) {
        self.branches.push(Branch {
            src: src.clone(),
            i: self.insts.len(),
            label: label.to_string(),
        });
        self.add(src, br);
    }

    fn expr(&mut self, a: &Expr) -> Result<(), String> {
        match a {
            Expr::True => {
                self.add(&Src::blank(), Inst::Const(Val::True));
            }
            Expr::False => {
                self.add(&Src::blank(), Inst::Const(Val::False));
            }
            Expr::Null => {
                self.add(&Src::blank(), Inst::Const(Val::Null));
            }
            Expr::Inf => {
                self.add(&Src::blank(), Inst::Const(Val::Num(f64::INFINITY)));
            }
            Expr::Nan => {
                self.add(&Src::blank(), Inst::Const(Val::Num(f64::NAN)));
            }
            Expr::Pi => {
                self.add(&Src::blank(), Inst::Const(Val::Num(std::f64::consts::PI)));
            }
            Expr::Str(s) => {
                self.add(&Src::blank(), Inst::Const(Val::Str(s.clone())));
            }
            Expr::Num(a) => {
                self.add(&Src::blank(), Inst::Const(Val::Num(*a)));
            }
            Expr::Call(src, f, args) => {
                self.expr(f)?;
                for a in args {
                    self.expr(a)?;
                }
                self.add(src, Inst::Call(args.len()));
            }
            Expr::List(v) => {
                for a in v {
                    self.expr(a)?;
                }
                self.add(&Src::blank(), Inst::List(v.len()));
            }
            Expr::Object(v) => {
                for a in v {
                    self.expr(a)?;
                }
                self.add(&Src::blank(), Inst::Object(v.len()));
            }
            Expr::Subscript(src, a, i) => {
                self.expr(a)?;
                self.expr(i)?;
                self.add(src, Inst::Subscript);
            }
            Expr::Slice(src, a, i, j) => {
                self.expr(a)?;
                self.expr(i)?;
                self.expr(j)?;
                self.add(src, Inst::Slice);
            }
            Expr::Id(src, name) => {
                self.add(src, Inst::LoadGlobal(name.to_string()));
            }
            Expr::Infix(src, inst, a, b) => {
                self.expr(a)?;
                self.expr(b)?;
                self.add(src, inst.clone());
            }
            Expr::Prefix(src, inst, a) => {
                self.expr(a)?;
                self.add(src, inst.clone());
            }
            Expr::InfixAssign(src, inst, a, b) => match &**a {
                Expr::Id(_ec, name) => {
                    self.expr(a)?;
                    self.expr(b)?;
                    self.add(src, inst.clone());
                    self.add(src, Inst::StoreGlobal(name.to_string()));
                }
                Expr::Subscript(src, a, i) => {
                    self.expr(a)?;
                    self.expr(i)?;
                    self.add(src, Inst::Dup2Subscript);
                    self.expr(b)?;
                    self.add(src, inst.clone());
                    self.add(src, Inst::StoreAt);
                }
                _ => {
                    return Err(format!("{}: Expected lvalue", src.clone()));
                }
            },
            Expr::Assign(src, a, b) => match &**a {
                Expr::Id(_ec, name) => {
                    self.expr(b)?;
                    self.add(src, Inst::StoreGlobal(name.to_string()));
                }
                Expr::Subscript(src, a, i) => {
                    self.expr(a)?;
                    self.expr(i)?;
                    self.expr(b)?;
                    self.add(src, Inst::StoreAt);
                }
                _ => {
                    return Err(format!("{}: Expected lvalue", src.clone()));
                }
            },
            Expr::Or(a, b) => {
                self.expr(a)?;
                let after_label = self.tmp();
                self.branch(&Src::blank(), Inst::DupBrTrue(0), &after_label);
                self.add(&Src::blank(), Inst::Pop);
                self.expr(b)?;
                self.labels.insert(after_label, self.insts.len());
            }
            Expr::And(a, b) => {
                self.expr(a)?;
                let after_label = self.tmp();
                self.branch(&Src::blank(), Inst::DupBrFalse(0), &after_label);
                self.add(&Src::blank(), Inst::Pop);
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

    fn stmt(&mut self, a: &Stmt) {
        match a {
            Stmt::Global(_, _) | Stmt::Nonlocal(_, _) => {}
            Stmt::If(cond, yes, no) => {
                // Condition
                self.expr(cond)?;
                let else_label = self.tmp();
                self.branch(&Src::blank(), Inst::BrFalse(0), &else_label);

                // Then
                self.block(yes)?;
                let after_label = self.tmp();
                self.branch(&Src::blank(), Inst::Br(0), &after_label);

                // Else
                self.labels.insert(else_label, self.insts.len());
                self.block(no)?;

                // After
                self.labels.insert(after_label, self.insts.len());
            }
            Stmt::Assert(src, cond, msg) => {
                self.expr(cond)?;
                self.add(src, Inst::Assert(msg.to_string()));
            }
            Stmt::Prin(a) => {
                self.expr(a)?;
                self.add(&Src::blank(), Inst::Prin);
            }
            Stmt::Label(src, s) => {
                if self
                    .labels
                    .insert(s.to_string(), self.insts.len())
                    .is_some()
                {
                    return Err(format!("{}: '{}' was already defined", src.clone(), s));
                }
            }
            Stmt::Goto(src, label) => self.branch(src, Inst::Br(0), label),
            Stmt::Dowhile(cond, body) => {
                // Body
                let loop_label = self.tmp();
                self.labels.insert(loop_label.clone(), self.insts.len());
                self.block(body)?;

                // Condition
                self.expr(cond)?;
                self.branch(&Src::blank(), Inst::BrTrue(0), &loop_label);
            }
            Stmt::While(cond, body) => {
                // Bypass
                let cond_label = self.tmp();
                self.branch(&Src::blank(), Inst::Br(0), &cond_label);

                // Body
                let loop_label = self.tmp();
                self.labels.insert(loop_label.clone(), self.insts.len());
                self.block(body)?;

                // Condition
                self.labels.insert(cond_label, self.insts.len());
                self.expr(cond)?;
                self.branch(&Src::blank(), Inst::BrTrue(0), &loop_label);
            }
            Stmt::Expr(a) => {
                self.expr(a)?;
                self.add(&Src::blank(), Inst::Pop);
            }
            _ => {
                // TODO: remove this
                eprintln!("{:?}", a);
                todo!();
            }
        }
    }

    fn block(&mut self, v: &Vec<Stmt>) {
        for a in v {
            self.stmt(a)?;
        }
    }

    fn compile(&mut self, body: &Vec<Stmt>) {
        // Declare variables
        self.decl_block(body)?;

        // Generate code
        self.block(body)?;
        self.add(&Src::blank(), Inst::Const(Val::Null));
        self.add(&Src::blank(), Inst::Return);
    }
}

fn func(
    outer: Option<Rc<RefCell<Env>>>,
    params: Vec<String>,
    body: &Vec<Stmt>,
) -> Result<FuncDef, String> {
    let env = Env::new(outer);
    let env = Rc::new(RefCell::new(env));
    let env = Some(env);
    let mut compiler = Compiler::new(env);
    compiler.compile(body)
}

pub fn compile(ast: &Vec<Stmt>, file: &str) {
    let mut out = match File::create(file) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("Error creating file '{}': {}", file, e);
            process::exit(1);
            unreachable!()
        }
    };
    let mut compiler = Compiler::new(None, out);
    compiler.compile(ast)
}
