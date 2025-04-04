use crate::ast::*;
use crate::parser::*;
use std::collections::HashSet;
use std::fs::File;
use std::io::Write;
use std::process;

const PREFIX_JS_BYTES: &[u8] = include_bytes!("prefix.js");

fn emit(out: &mut File, b: &[u8]) {
    match out.write_all(b) {
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
    out: &'a mut File,
}

impl<'a> Compiler<'a> {
    fn new(out: &'a mut File) -> Self {
        Compiler {
            outers: HashSet::<String>::new(),
            assigned: HashSet::<String>::new(),
            out,
        }
    }

    fn emit(&mut self, s: &str) {
        emit(&mut self.out, s.as_bytes());
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
            Expr::Assign(a, b) => {
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
                self.emit("])");
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
                self.emit("(");
                self.expr(a);
                self.emit(s);
                self.expr(b);
                self.emit(")");
            }
            Expr::Input(a) => {
                self.emit("input(");
                self.expr(a);
                self.emit(")");
            }
            Expr::Prefix(s, a) => {
                self.emit("(");
                self.emit(s);
                self.expr(a);
                self.emit(")");
            }
            Expr::Assign(a, b) => match &**a {
                Expr::Subscript(a, i) => {
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
                    self.emit("=");
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
                self.emit("_prin(");
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
            Stmt::For(_src, name, collection, body) => {
                self.emit("for (");
                self.emit(name);
                self.emit(" of ");
                self.expr(collection);
                self.emit(") {\n");
                self.block(body);
                self.emit("}\n");
            }
            Stmt::Func(_src, name, params, outers, body) => {
                self.emit("function ");
                self.emit(&name);
                self.emit("(");
                self.emit(&params.join(","));
                self.emit(") {\n");

                // TODO: outers
                let mut compiler = Compiler::new(self.out);
                compiler.compile(body);
                self.emit("return null\n");

                self.emit("}\n");
            }
            Stmt::Return(_src, a) => {
                self.emit("return ");
                self.expr(a);
                self.emit(";\n");
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
        for a in self.assigned.clone() {
            self.emit("var ");
            self.emit(&a);
            self.emit("= null;\n");
        }

        // Generate code
        self.block(body);
    }
}

pub fn compile(ast: &Vec<Stmt>, file: &str) {
    let mut out = match File::create(file) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("{}: {}", file, e);
            process::exit(1);
        }
    };
    emit(&mut out, PREFIX_JS_BYTES);
    if INPUT.with(|flag| flag.get()) {
        emit(&mut out, b"const input = require('readline-sync');\n");
    }
    let mut compiler = Compiler::new(&mut out);
    compiler.compile(ast)
}
