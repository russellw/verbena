//! A BASIC-like language interpreter and virtual machine.
//!
//! This crate provides a parser, compiler, and virtual machine for a simple
//! BASIC-like programming language. It can be used to parse and execute
//! programs written in this language.
//!
//! # Example
//!
//! ```
//! use verbena::prep;
//! use verbena::parse;
//! use verbena::Process;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Prepare the source code
//!     let source = "PRINT \"Hello, world!\"";
//!     let chars = prep(source);
//!
//!     // Parse the program
//!     let program = parse(&chars)?;
//!
//!     // Execute the program
//!     let mut process = Process::new(program);
//!     process.run()?;
//!
//!     Ok(())
//! }
//! ```

/// Re-export all error handling types and functions.
mod error;
/// Parser module for converting source code to bytecode.
mod parser;
/// Virtual machine for executing compiled programs.
mod vm;

pub use error::*;
pub use parser::*;
pub use vm::*;
