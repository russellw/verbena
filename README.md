# Verbena Programming Language

A lightweight, expressive scripting language that compiles to JavaScript, designed for simplicity and readability.

## Installation

```bash
# Clone the repository
git clone https://github.com/russellw/verbena
cd verbena

# Build the compiler
cargo build --release

# Add to your PATH (optional)
export PATH=$PATH:$(pwd)/target/release
```

## Basic Syntax

### Variables and Data Types

Dynamically typed. Variable declaration is implicit upon assignment.

```
# Variables are dynamically typed
name = "Verbena"
is_awesome = true

# Data structures
numbers = [1, 2, 3, 4, 5]
settings = {
    "debug": true,
    "port": 8080,
    "host": "localhost"
}
```

### Operators

Standard arithmetic, comparison, and logical operators:

```
# Arithmetic
sum = a + b
difference = a - b
product = a * b
quotient = a / b
remainder = a % b
power = a ** b

# Compound assignment
count += 1
total *= factor

# Comparison
is_equal = a == b
is_not_equal = a != b
is_greater = a > b
is_less = a < b

# Logical
result = condition1 && condition2
result = condition1 || condition2
result = !condition
```

### Control Flow

```
# If statements
if x > 0
    prin "Positive"
elif x < 0
    prin "Negative"
else
    prin "Zero"
end

# While loops
while count < 10
    count += 1
    prin count
end

# Do-while loops
dowhile condition
    # Code executed at least once
end

# For loops
for item : collection
    prin item
end

# For loops with index
for i, item : collection
    prin i, item
end

# Case statement (similar to switch)
case value
| 1, 2, 3
    prin "Small number"
| 4, 5, 6
    prin "Medium number"
else
    prin "Large number"
end
```

### Functions

```
fn greet(name)
    prin "Hello, " + name + "!"
end

fn add(a, b)
    return a + b
end

# Call functions
greet("World")
result = add(5, 3)
```

### Error Handling

```
try
    # Code that might throw an error
    result = risky_operation()
catch error
    # Handle the error
    eprin "Error: " + error
end

# Throw an error
throw "Something went wrong"

# Assertions
assert x > 0, "x must be positive"
```

## Advanced Features

### Outer Variables

Verbena allows you to explicitly declare variables from outer scopes that a function needs access to:

```
count = 0

fn increment()
    outer count
    count += 1
end
```

### Subscripts and Slices

```
# Access array elements
first = array[0]

# Slices (similar to Python)
subset = array[1:4]  # Elements 1, 2, 3
beginning = array[:5]  # First 5 elements
end = array[5:]  # Elements from index 5 to the end
```

### Type Checking

```
if typeof(value) == "string"
    prin "Value is a string: " + value
end
```

## Built-in Functions

Verbena includes several built-in functions for common operations:

- `len()` - Get the length of a collection
- `push()` - Add an element to an array
- `repr()` - Get string representation
- `string()` - Convert to string
- `range()` - Generate a sequence of numbers

## Compiling and Running

```bash
# Compile a Verbena file to JavaScript
verbena -o output.js input.va

# Run the compiled JavaScript
node output.js
```

## Command Line Options

```
Usage: Verbena [OPTIONS] <file>

Arguments:
  <file>  Source file

Options:
  -o <file>  Output file [default: a.js]
  -h, --help     Print help
  -V, --version  Print version
```

## Examples

### Hello World

```
print "Hello, Verbena!"
```

### Factorial Function

```
fn factorial(n)
    if n <= 1
        return 1
    else
        return n * factorial(n - 1)
    end
end

for i : range(1, 11)
    print i, "! =", factorial(i)
end
```

### Error Handling Example

```
fn divide(a, b)
    if b == 0
        throw "Division by zero"
    end
    return a / b
end

try
    result = divide(10, 0)
    print "Result:", result
catch error
    print "Error caught:", error
end
```

## License

This project is licensed under either of:

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
