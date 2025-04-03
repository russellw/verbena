use crate::ast::*;
use std::collections::HashSet;
use std::fs::File;
use std::io::Write;

// Compiler is instantiated separately for each nested function
struct Compiler {
    outers: HashSet<String>,
    assigned: HashSet<String>,

    tmp_count: usize,
    out: &mut File,
}

impl Compiler {
    fn new(out: &mut File) -> Self {
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

    fn emit(&mut self, s: &str) {
        self.out.write(s.as_bytes());
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

    fn decl_stmt(&mut self, a: &Stmt) -> Result<(), String> {
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
        Ok(())
    }

    fn decl_block(&mut self, v: &Vec<Stmt>) -> Result<(), String> {
        for a in v {
            self.decl_stmt(a);
        }
        Ok(())
    }

    // Generate code
    fn expr(&mut self, a: &Expr) {
        match a {
            Expr::Atom(s) => {
                self.emit(s);
            }
            Expr::Call(f, args) => {
                self.expr(f);
                self.emit("(");
                for (i, a) in args.iter().enumerate() {
                    if 0 < i {
                        self.emit(",");
                    }
                    self.expr(a);
                }
                self.emit(")");
            }
            Expr::List(v) => {
                self.emit("[");
                for (i, a) in v.iter().enumerate() {
                    if 0 < i {
                        self.emit(",");
                    }
                    self.expr(a);
                }
                self.emit("]");
            }
            Expr::Object(v) => {
                self.emit("new Map([");
                for i in (0..v.len()).step_by(2) {
                    if 0 < i {
                        self.emit(",");
                    }
                    self.emit("[");
                    self.expr(&v[i]);
                    self.emit(",");
                    self.expr(&v[i + 1]);
                    self.emit("]");
                }
                self.emit("new Map])");
            }
            Expr::Subscript(a, i) => {
                self.emit("_get(");
                self.expr(a);
                self.emit(",");
                self.expr(i);
                self.emit(")");
            }
            Expr::Slice(a, i, j) => {
                self.emit("_slice(");
                self.expr(a);
                self.emit(",");
                self.expr(i);
                self.emit(",");
                self.expr(j);
                self.emit(")");
            }
            Expr::Infix(s, a, b) => {
                self.expr(a);
                self.emit(s);
                self.expr(b);
            }
            Expr::Prefix(s, a) => {
                self.emit(s);
                self.expr(a);
            }
            Expr::Assign(s, a, b) => match &**a {
                Expr::Subscript(a, i) => {
                    // TODO: a[i] += b
                    self.emit("_set(");
                    self.expr(&a);
                    self.emit(",");
                    self.expr(&i);
                    self.emit(",");
                    self.expr(b);
                    self.emit(")");
                }
                _ => {
                    self.expr(a);
                    self.emit(s);
                    self.expr(b);
                }
            },
        }
    }

    fn stmt(&mut self, a: &Stmt) {
        match a {
            Stmt::If(_src, cond, yes, no) => {
                self.emit("if (");
                self.expr(cond);
                self.emit(") {\n");
                self.block(yes);
                self.emit("} else {\n");
                self.block(no);
                self.emit("}\n");
            }
            Stmt::Assert(_src, cond, msg) => {
                self.emit("assert(");
                self.expr(cond);
                if !msg.is_empty() {
                    self.emit(",");
                    self.emit(msg);
                }
                self.emit(");\n");
            }
            Stmt::Prin(_src, a) => {
                self.emit("process.stdout.write(");
                self.expr(a);
                self.emit(");\n");
            }
            Stmt::Label(_src, s) => {
                self.emit(s);
                self.emit(":\n");
            }
            Stmt::Dowhile(_src, cond, body) => {
                self.emit("do {");
                self.block(body);
                self.emit("} while (");
                self.expr(cond);
                self.emit(");\n");
            }
            Stmt::While(_src, cond, body) => {
                self.emit("while (");
                self.expr(cond);
                self.emit(") {\n");
                self.block(body);
                self.emit("}\n");
            }
            Stmt::Expr(_src, a) => {
                self.expr(a);
                self.emit(";\n");
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

fn func(name: String, params: Vec<String>, body: &Vec<Stmt>, out: &mut File) {
    out.write_all(b"function ");
    out.write_all(name.as_bytes());
    out.write_all(b"(");
    out.write_all(params.join(",").as_bytes());
    out.write_all(b") {\n");
    let mut compiler = Compiler::new(out);
    compiler.compile(body);
    out.write_all(b"return null\n");
    out.write_all(b"}\n");
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
    out.write_all(b"#!/usr/bin/env node\n");
    out.write_all(b"'use strict';\n");
    out.write_all(b"const assert = require('assert');\n");
    let mut compiler = Compiler::new(&mut out);
    compiler.compile(ast)
}
