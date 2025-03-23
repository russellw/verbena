use crate::ast::*;
use crate::compile_error::*;
use crate::program::*;
use crate::val::*;
use num_bigint::BigInt;
use num_traits::Num;
use std::collections::HashMap;
use std::mem;
use std::str::FromStr;

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

fn parse_bigint(input: &str) -> Result<BigInt, String> {
    // Convert to bytes for easier prefix checking
    let bytes = input.as_bytes();

    // Check for prefixes and determine base
    if bytes.len() >= 2 && bytes[0] == b'0' {
        match bytes[1] {
            b'x' | b'X' => {
                // Hexadecimal (0x)
                let hex_str = &input[2..]; // Skip the '0x' prefix
                return BigInt::from_str_radix(hex_str, 16)
                    .map_err(|e| format!("Invalid hexadecimal: {}", e));
            }
            b'b' | b'B' => {
                // Binary (0b)
                let bin_str = &input[2..]; // Skip the '0b' prefix
                return BigInt::from_str_radix(bin_str, 2)
                    .map_err(|e| format!("Invalid binary: {}", e));
            }
            b'o' | b'O' => {
                // Octal (0o)
                let oct_str = &input[2..]; // Skip the '0o' prefix
                return BigInt::from_str_radix(oct_str, 8)
                    .map_err(|e| format!("Invalid octal: {}", e));
            }
            _ => {}
        }
    }

    // Default to decimal
    BigInt::from_str(input).map_err(|e| format!("Invalid decimal: {}", e))
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
            Expr::Float(ec, s) => {
                let a = match s.parse::<f64>() {
                    Ok(a) => a,
                    Err(e) => return Err(CompileError::new(ec.clone(), e.to_string())),
                };
                self.code.push(Inst::Const(Val::Float(a)));
            }
            Expr::Int(ec, s) => {
                let a = match parse_bigint(s) {
                    Ok(a) => a,
                    Err(e) => return Err(CompileError::new(ec.clone(), e.to_string())),
                };
                self.code.push(Inst::Const(Val::Int(a)));
            }
            Expr::Call(ec, f, args) => {
                if let Expr::Id(_, name) = &**f {
                    for a in args {
                        self.expr(a)?;
                    }
                    self.code
                        .push(Inst::Call(ec.clone(), name.to_string(), args.len()));
                    return Ok(());
                }
                todo!();
            }
            Expr::Id(ec, name) => {
                self.code.push(Inst::Load(ec.clone(), name.to_string()));
            }
            _ => {
                eprintln!("{:?}", a);
                todo!();
            }
        }
        Ok(())
    }

    fn stmt(&mut self, a: &Stmt) -> Result<(), CompileError> {
        match a {
            Stmt::Print(ec, v) => {
                for a in v {
                    self.expr(a)?;
                    self.code
                        .push(Inst::Call(ec.clone(), "_print".to_string(), 1));
                }
            }
            Stmt::Expr(a) => {
                self.expr(a)?;
                self.code.push(Inst::Pop);
            }
            _ => {
                eprintln!("{:?}", a);
                todo!();
            }
        }
        Ok(())
    }

    fn compile(&mut self) -> Result<Program, CompileError> {
        for a in &self.ast.code {
            self.stmt(a)?;
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
