use crate::ast::*;
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
    fn new(outers: HashSet<String>, out: &'a mut File) -> Self {
        Compiler {
            outers,
            assigned: HashSet::<String>::new(),
            out,
        }
    }

    fn emit(&mut self, s: &str) {
        emit(self.out, s.as_bytes());
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
            Expr::List(v) | Expr::Object(v) => {
                for a in v {
                    self.decl_expr(a);
                }
            }
            Expr::Slice(a, i, j) => {
                self.decl_expr(a);
                self.decl_expr(i);
                self.decl_expr(j);
            }
            Expr::Subscript(a, b) | Expr::Infix(_, a, b) => {
                self.decl_expr(a);
                self.decl_expr(b);
            }
            Expr::Prefix(_, a) | Expr::Typeof(a) => {
                self.decl_expr(a);
            }
            Expr::Assign(a, b) => {
                if let Expr::Atom(name) = &**a {
                    self.assigned.insert(name.to_string());
                }
                self.decl_expr(a);
                self.decl_expr(b);
            }
            Expr::Atom(_) => {}
        }
    }

    fn decl_stmt(&mut self, a: &Stmt) {
        match a {
            Stmt::If(_, cond, yes, no) => {
                self.decl_expr(cond);
                self.decl_block(yes);
                self.decl_block(no);
            }
            Stmt::Try(_, normal, _, fallback) => {
                self.decl_block(normal);
                self.decl_block(fallback);
            }
            Stmt::Prin(_, a)
            | Stmt::EPrin(_, a)
            | Stmt::Throw(_, a)
            | Stmt::Expr(_, a)
            | Stmt::Assert(_, a, _)
            | Stmt::Return(_, a) => {
                self.decl_expr(a);
            }
            Stmt::Dowhile(_, cond, body) | Stmt::While(_, cond, body) => {
                self.decl_expr(cond);
                self.decl_block(body);
            }
            Stmt::For(_, item, collection, body) => {
                self.assigned.insert(item.to_string());
                self.decl_expr(collection);
                self.decl_block(body);
            }
            Stmt::For2(_, idx, item, collection, body) => {
                self.assigned.insert(idx.to_string());
                self.assigned.insert(item.to_string());
                self.decl_expr(collection);
                self.decl_block(body);
            }
            Stmt::Label(_, _) | Stmt::Func(_, _, _, _, _) => {}
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
                self.expr(a);
                self.emit(".slice(");
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
            Expr::Prefix(s, a) => {
                self.emit("(");
                self.emit(s);
                self.expr(a);
                self.emit(")");
            }
            Expr::Typeof(a) => {
                self.emit("_typeof(");
                self.expr(a);
                self.emit(")");
            }
            Expr::Assign(a, b) => match &**a {
                Expr::Subscript(a, i) => {
                    self.emit("_set(");
                    self.expr(a);
                    self.emit(",");
                    self.expr(i);
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

    fn stmt(&mut self, a: &Stmt, last: bool) {
        match a {
            Stmt::If(_src, cond, yes, no) => {
                self.emit("if (");
                self.expr(cond);
                self.emit(") {\n");
                self.block(yes, last);
                self.emit("} else {\n");
                self.block(no, last);
                self.emit("}\n");
            }
            Stmt::Try(_src, normal, name, fallback) => {
                self.emit("try {\n");
                self.block(normal, last);
                self.emit("} catch (");
                self.emit(name);
                self.emit(") {\n");
                self.block(fallback, last);
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
            Stmt::EPrin(_src, a) => {
                self.emit("_eprin(");
                self.expr(a);
                self.emit(");\n");
            }
            Stmt::Label(_src, s) => {
                self.emit(s);
                self.emit(":\n");
            }
            Stmt::Dowhile(_src, cond, body) => {
                self.emit("do {");
                self.block(body, false);
                self.emit("} while (");
                self.expr(cond);
                self.emit(");\n");
            }
            Stmt::While(_src, cond, body) => {
                self.emit("while (");
                self.expr(cond);
                self.emit(") {\n");
                self.block(body, false);
                self.emit("}\n");
            }
            Stmt::Expr(_src, a) => {
                if last {
                    self.emit("return ");
                }
                self.expr(a);
                self.emit(";\n");
            }
            Stmt::For(_src, item, collection, body) => {
                self.emit("for (");
                self.emit(item);
                self.emit(" of ");
                self.expr(collection);
                self.emit(") {\n");
                self.block(body, false);
                self.emit("}\n");
            }
            Stmt::For2(_src, idx, item, collection, body) => {
                self.emit("for ([");
                self.emit(idx);
                self.emit(",");
                self.emit(item);
                self.emit("] of ");
                self.expr(collection);
                self.emit(".entries()) {\n");
                self.block(body, false);
                self.emit("}\n");
            }
            Stmt::Func(_src, name, params, outers, body) => {
                self.emit("function ");
                self.emit(name);
                self.emit("(");
                self.emit(&params.join(","));
                self.emit(") {\n");

                // TODO: outers
                let mut compiler = Compiler::new(outers.clone(), self.out);
                compiler.compile(body);
                self.emit("return null\n");

                self.emit("}\n");
            }
            Stmt::Return(_src, a) => {
                self.emit("return ");
                self.expr(a);
                self.emit(";\n");
            }
            Stmt::Throw(_src, a) => {
                self.emit("throw ");
                self.expr(a);
                self.emit(";\n");
            }
        }
    }

    fn block(&mut self, v: &[Stmt], last: bool) {
        for (i, a) in v.iter().enumerate() {
            self.stmt(a, last && i == v.len() - 1);
        }
    }

    fn compile(&mut self, body: &Vec<Stmt>) {
        // Declare variables
        self.decl_block(body);
        for a in self.assigned.clone() {
            if !self.outers.contains(&a) {
                self.emit("var ");
                self.emit(&a);
                self.emit("= null;\n");
            }
        }

        // Generate code
        self.block(body, true);
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
    let mut compiler = Compiler::new(HashSet::<String>::new(), &mut out);
    compiler.compile(ast)
}
