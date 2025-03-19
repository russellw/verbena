//! A BASIC-like language interpreter and virtual machine.
//!
//! This crate provides a parser, compiler, and virtual machine for a simple
//! BASIC-like programming language. It can be used to parse and execute
//! programs written in this language.
//!
//! # Example
//!
//! ```
//! use verbena::parse;
//! use verbena::Process;
//!
//! fn main() {
//!     // Prepare the source code
//!     let source = "PRINT \"Hello, world!\"";
//!
//!     // Parse the program
//!     let program = parse(&source).unwrap();
//!
//!     // Execute the program
//!     let mut process = Process::new(program);
//!     process.run();
//! }
//! ```

pub mod aparser;
pub mod ast;
pub mod compile_error;
pub mod compiler;
/// Re-export all error handling types and functions.
pub mod error;
/// Parser module for converting source code to bytecode.
pub mod parser;
pub mod val;
/// Virtual machine for executing compiled programs.
pub mod vm;

pub use aparser::*;
pub use ast::*;
pub use compile_error::*;
pub use compiler::*;
pub use error::*;
pub use parser::*;
pub use val::*;
pub use vm::*;
