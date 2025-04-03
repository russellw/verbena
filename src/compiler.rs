use crate::ast::*;
use std::collections::HashSet;
use std::fs::File;
use std::io::Write;
use std::process;

fn emit(out: &mut File, s: &str) {
    match out.write_all(s.as_bytes()) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    }
}

// Compiler is instantiated separately for each nested function
struct Compiler<'a> {
    outers: HashSet<String>,
    assigned: HashSet<String>,

    tmp_count: usize,
    out: &'a mut File,
}

impl<'a> Compiler<'a> {
    fn new(out: &'a mut File) -> Self {
        Compiler {
            outers: HashSet::<String>::new(),
            assigned: HashSet::<String>::new(),
            tmp_count: 0,
            out,
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
            Expr::Call(f, args) => {
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
            Expr::Subscript(a, i) => {
                self.decl_expr(a);
                self.decl_expr(i);
            }
            Expr::Slice(a, i, j) => {
                self.decl_expr(a);
                self.decl_expr(i);
                self.decl_expr(j);
            }
            Expr::Infix(_s, a, b) => {
                self.decl_expr(a);
                self.decl_expr(b);
            }
            Expr::Prefix(_s, a) => {
                self.decl_expr(a);
            }
            Expr::Assign(_s, a, b) => {
                match &**a {
                    Expr::Atom(name) => {
                        self.assigned.insert(name.to_string());
                    }
                    _ => {}
                }
                self.decl_expr(a);
                self.decl_expr(b);
            }
            _ => {}
        }
    }

    fn decl_stmt(&mut self, a: &Stmt) {
        match a {
            Stmt::If(_src, cond, yes, no) => {
                self.decl_expr(cond);
                self.decl_block(yes);
                self.decl_block(no);
            }
            Stmt::Assert(_src, cond, _msg) => {
                self.decl_expr(cond);
            }
            Stmt::Prin(_src, a) => {
                self.decl_expr(a);
            }
            Stmt::Dowhile(_src, cond, body) => {
                self.decl_block(body);
                self.decl_expr(cond);
            }
            Stmt::While(_src, cond, body) => {
                self.decl_block(body);
                self.decl_expr(cond);
            }
            Stmt::Expr(_src, a) => {
                self.decl_expr(a);
            }
            _ => {}
        }
    }

    fn decl_block(&mut self, v: &Vec<Stmt>) {
        for a in v {
            self.decl_stmt(a);
        }
    }

    // Generate code
    fn expr(&mut self, a: &Expr) {
        match a {
            Expr::Atom(s) => {
                emit(self.out, s);
            }
            Expr::Call(f, args) => {
                self.expr(f);
                emit(self.out, "(");
                for (i, a) in args.iter().enumerate() {
                    if 0 < i {
                        emit(self.out, ",");
                    }
                    self.expr(a);
                }
                emit(self.out, ")");
            }
            Expr::List(v) => {
                emit(self.out, "[");
                for (i, a) in v.iter().enumerate() {
                    if 0 < i {
                        emit(self.out, ",");
                    }
                    self.expr(a);
                }
                emit(self.out, "]");
            }
            Expr::Object(v) => {
                emit(self.out, "new Map([");
                for i in (0..v.len()).step_by(2) {
                    if 0 < i {
                        emit(self.out, ",");
                    }
                    emit(self.out, "[");
                    self.expr(&v[i]);
                    emit(self.out, ",");
                    self.expr(&v[i + 1]);
                    emit(self.out, "]");
                }
                emit(self.out, "new Map])");
            }
            Expr::Subscript(a, i) => {
                emit(self.out, "_get(");
                self.expr(a);
                emit(self.out, ",");
                self.expr(i);
                emit(self.out, ")");
            }
            Expr::Slice(a, i, j) => {
                emit(self.out, "_slice(");
                self.expr(a);
                emit(self.out, ",");
                self.expr(i);
                emit(self.out, ",");
                self.expr(j);
                emit(self.out, ")");
            }
            Expr::Infix(s, a, b) => {
                self.expr(a);
                emit(self.out, s);
                self.expr(b);
            }
            Expr::Prefix(s, a) => {
                emit(self.out, s);
                self.expr(a);
            }
            Expr::Assign(s, a, b) => match &**a {
                Expr::Subscript(a, i) => {
                    // TODO: a[i] += b
                    emit(self.out, "_set(");
                    self.expr(&a);
                    emit(self.out, ",");
                    self.expr(&i);
                    emit(self.out, ",");
                    self.expr(b);
                    emit(self.out, ")");
                }
                _ => {
                    self.expr(a);
                    emit(self.out, s);
                    self.expr(b);
                }
            },
        }
    }

    fn stmt(&mut self, a: &Stmt) {
        match a {
            Stmt::If(_src, cond, yes, no) => {
                emit(self.out, "if (");
                self.expr(cond);
                emit(self.out, ") {\n");
                self.block(yes);
                emit(self.out, "} else {\n");
                self.block(no);
                emit(self.out, "}\n");
            }
            Stmt::Assert(_src, cond, msg) => {
                emit(self.out, "assert(");
                self.expr(cond);
                if !msg.is_empty() {
                    emit(self.out, ",");
                    emit(self.out, msg);
                }
                emit(self.out, ");\n");
            }
            Stmt::Prin(_src, a) => {
                emit(self.out, "process.stdout.write(");
                self.expr(a);
                emit(self.out, ");\n");
            }
            Stmt::Label(_src, s) => {
                emit(self.out, s);
                emit(self.out, ":\n");
            }
            Stmt::Dowhile(_src, cond, body) => {
                emit(self.out, "do {");
                self.block(body);
                emit(self.out, "} while (");
                self.expr(cond);
                emit(self.out, ");\n");
            }
            Stmt::While(_src, cond, body) => {
                emit(self.out, "while (");
                self.expr(cond);
                emit(self.out, ") {\n");
                self.block(body);
                emit(self.out, "}\n");
            }
            Stmt::Expr(_src, a) => {
                self.expr(a);
                emit(self.out, ";\n");
            }
            Stmt::Func(_src, name, params, outers, body) => {
                func(name, params, outers, body, self.out);
            }
            Stmt::Return(_src, a) => {
                emit(self.out, "return ");
                self.expr(a);
                emit(self.out, ";\n");
            }
        }
    }

    fn block(&mut self, v: &Vec<Stmt>) {
        for a in v {
            self.stmt(a);
        }
    }

    fn compile(&mut self, body: &Vec<Stmt>) {
        // Declare variables
        self.decl_block(body);

        // Generate code
        self.block(body);
    }
}

fn func(
    name: &str,
    params: &Vec<String>,
    outers: &HashSet<String>,
    body: &Vec<Stmt>,
    out: &mut File,
) {
    emit(out, "function ");
    emit(out, &name);
    emit(out, "(");
    emit(out, &params.join(","));
    emit(out, ") {\n");

    // TODO: outers
    let mut compiler = Compiler::new(out);
    compiler.compile(body);
    emit(out, "return null\n");

    emit(out, "}\n");
}

pub fn compile(ast: &Vec<Stmt>, file: &str) {
    let mut out = match File::create(file) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("Error creating file '{}': {}", file, e);
            process::exit(1);
        }
    };
    emit(&mut out, "#!/usr/bin/env node\n");
    emit(&mut out, "'use strict';\n");
    emit(&mut out, "const assert = require('assert');\n");
    let mut compiler = Compiler::new(&mut out);
    compiler.compile(ast)
}
