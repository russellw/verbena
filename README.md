# BASIC Interpreter

A parser, compiler, and virtual machine for a BASIC-like programming language implemented in Rust.

## Features

- Full-featured BASIC-like language with support for:
  - Arbitrary precision integers (via `num_bigint`)
  - Floating-point numbers
  - Strings with escape sequences
  - Lists and arrays
  - Comprehensive math functions
  - Control structures (IF/THEN/ELSE, FOR/NEXT, WHILE/WEND)
  - Subroutines (GOSUB/RETURN)
  - Labels and line numbers

- Rich standard library:
  - Trigonometric functions (SIN, COS, TAN, etc.)
  - Logarithmic functions (LN, LOG10, LOG2)
  - Statistical functions (MAX, MIN, etc.)
  - String manipulation (LEFT$, RIGHT$, MID$, INSTR, etc.)
  - Type conversions (TOINT, TOFLOAT, TOSTR)
  - Bitwise operations

- Detailed error reporting with source position information

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
verbena = "0.1.1"
```

## Usage

### Basic Example

```rust
use verbena::{prep, parse, Process};

fn main() {
    // Prepare the source code
    let source = r#"
    PRINT "Hello, world!"
    FOR i = 1 TO 5
        PRINT i
    NEXT i
    "#;
    let chars = prep(source);

    // Parse the program
    let program = parse(&chars).unwrap();

    // Execute the program
    let mut process = Process::new(program);
    process.run();
}
```

### Reading from a File

```rust
use verbena::{prep, parse, Process};
use std::fs;

fn main() {
    // Read source from file
    let source = fs::read_to_string("examples/hello.bas")?;
    let chars = prep(&source);

    // Parse and execute
    let program = parse(&chars).unwrap();
    let mut process = Process::new(program);
    process.run();
}
```

## Language Reference

### Variables and Assignment

```basic
LET x = 42
y = 3.14159
name$ = "John"
```

### Control Flow

```basic
' If statement
IF x > 10 THEN
    PRINT "x is greater than 10"
ELSE
    PRINT "x is not greater than 10"
ENDIF

' For loop
FOR i = 1 TO 10 STEP 2
    PRINT i
NEXT i

' While loop
WHILE x > 0
    PRINT x
    x = x - 1
WEND
```

### Arrays and Lists

```basic
' Declare an array
DIM arr 10

' Set values
arr[0] = 42
arr[1] = 123

' List literal
list = [1, 2, 3, 4]
```

### Functions

```basic
' Math functions
PRINT SQRT(16)         ' 4
PRINT SIN(3.14159/2)   ' 1
PRINT ABS(-42)         ' 42

' String functions
PRINT LEN("Hello")     ' 5
PRINT LEFT$("Hello", 2) ' "He"
PRINT INSTR("Hello", "ll") ' 3 (1-based indexing)
```

## License

This project is licensed under either of:

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
