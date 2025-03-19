/// An instruction for the virtual machine.
///
/// Instructions represent operations that can be performed by the VM.
#[derive(Debug, Clone)]
pub enum Inst {
    // Stack & Memory Operations
    /// Push a constant value onto the stack
    Const(Val),
    /// Remove the top value from the stack
    Pop,
    /// Push a variable's value onto the stack
    Load(String),
    /// Store the top stack value into a variable
    Store(String),

    // Control Flow
    /// Unconditional jump to a code position
    Br(usize),
    /// Jump to a code position if the top stack value is false
    BrFalse(usize),
    /// Duplicate the top stack value and jump if it's true
    DupBrTrue(usize),
    /// Duplicate the top stack value and jump if it's false
    DupBrFalse(usize),
    /// Call a subroutine at the specified position
    Gosub(usize),
    /// Return from a subroutine
    Return,
    /// Terminate program execution
    Exit,
    /// Check if the top stack value is true, error if not
    Assert,

    // I/O Operations
    /// Read user input into a variable
    Input(String),
    /// Output the top stack value
    Print,

    // Type Conversion
    /// Convert the top stack value to an integer
    ToInt,
    /// Convert the top stack value to a float
    ToFloat,
    /// Convert the top stack value to a string
    ToStr,
    /// Convert a number to a string with a specified base
    StrBase,
    /// Parse a string as a number with a specified base
    ValBase,

    // Comparison Operations
    /// Check if two values are equal
    Eq,
    /// Check if two values are not equal
    Ne,
    /// Check if one value is less than another
    Lt,
    /// Check if one value is less than or equal to another
    Le,
    /// Check if one value is greater than another
    Gt,
    /// Check if one value is greater than or equal to another
    Ge,

    // Logical Operations
    /// Logical NOT operation
    Not,

    // Bitwise Operations
    /// Bitwise NOT operation
    BitNot,
    /// Bitwise AND operation
    BitAnd,
    /// Bitwise OR operation
    BitOr,
    /// Bitwise XOR operation
    BitXor,
    /// Bit shift left operation
    Shl,
    /// Bit shift right operation
    Shr,

    // Arithmetic Operations (Integer-specific)
    /// Integer division
    IDiv,
    /// Calculate nth root of a number
    NthRoot,
    /// Count trailing zeros in binary representation
    TrailingZeros,
    /// Get the value of a specific bit
    Bit,
    /// Set the value of a specific bit
    SetBit,
    /// Calculate greatest common divisor
    Gcd,
    /// Calculate least common multiple
    Lcm,

    // Arithmetic Operations (Float-specific)
    /// Float division
    FDiv,
    /// Round down to nearest integer
    Floor,
    /// Round up to nearest integer
    Ceil,
    /// Round to nearest integer
    Round,
    /// Round to nearest with ties to even
    RoundTiesEven,
    /// Truncate to integer
    Trunc,
    /// Get fractional part
    Fract,
    /// Multiply and add (a*b+c)
    MulAdd,
    /// Euclidean division
    DivEuclid,
    /// Euclidean remainder
    RemEuclid,
    /// Integer power
    PowI,
    /// Exponential function
    Exp,
    /// Base-2 exponential
    Exp2,
    /// Natural logarithm
    Ln,
    /// Logarithm with custom base
    Log,
    /// Base-2 logarithm
    Log2,
    /// Base-10 logarithm
    Log10,
    /// Hypotenuse calculation
    Hypot,
    /// Sine function
    Sin,
    /// Cosine function
    Cos,
    /// Tangent function
    Tan,
    /// Arc sine function
    ASin,
    /// Arc cosine function
    ACos,
    /// Arc tangent function
    ATan,
    /// Two-argument arc tangent
    ATan2,
    /// exp(x) - 1 with better precision
    ExpM1,
    /// ln(1 + x) with better precision
    Ln1P,
    /// Hyperbolic sine
    SinH,
    /// Hyperbolic cosine
    CosH,
    /// Hyperbolic tangent
    TanH,
    /// Inverse hyperbolic sine
    ASinH,
    /// Inverse hyperbolic cosine
    ACosH,
    /// Inverse hyperbolic tangent
    ATanH,
    /// Check if value is NaN
    IsNan,
    /// Check if value is finite
    IsFinite,
    /// Check if value is infinite
    IsInfinite,
    /// Check if value is subnormal
    IsSubnormal,
    /// Check if value is normal
    IsNormal,
    /// Check if sign is positive
    IsSignPositive,
    /// Check if sign is negative
    IsSignNegative,
    /// Calculate reciprocal (1/x)
    Recip,
    /// Convert radians to degrees
    ToDegrees,
    /// Convert degrees to radians
    ToRadians,

    // Arithmetic Operations (Polymorphic)
    /// Addition
    Add,
    /// Subtraction
    Sub,
    /// Multiplication
    Mul,
    /// Modulo
    Mod,
    /// Negation
    Neg,
    /// Power operation
    Pow,
    /// Square root
    Sqrt,
    /// Cube root
    Cbrt,
    /// Maximum of two values
    Max,
    /// Minimum of two values
    Min,
    /// Midpoint between two values
    Midpoint,
    /// Total comparison (for ordering floats)
    TotalCmp,
    /// Clamp value to a range
    Clamp,
    /// Absolute value
    Abs,
    /// Sign function
    Signum,
    /// Copy sign from one value to another
    CopySign,

    // String Operations
    /// String length
    Len,
    /// Left substring
    Left,
    /// Right substring
    Right,
    /// Middle substring
    Mid,
    /// ASCII code of first character
    Asc,
    /// Character from code point
    Chr,
    /// Find substring position
    Instr,
    /// Convert to uppercase
    UCase,
    /// Convert to lowercase
    LCase,

    // List Operations
    /// Create a new list
    Dim(String),
    /// Create a list from stack values
    List(usize),
    /// Get an element from a list
    Subscript,
    /// Set an element in a list
    StoreSubscript,

    /// Generate a random number
    Rnd,
    /// Get the type of a value
    Typeof,
}

/// A compiled program ready for execution.
///
/// Contains the bytecode instructions and debugging information.
pub struct Program {
    // These two vectors parallel each other
    // carets[i] is the error location in the input text
    // if an error occurs while executing code[i]
    carets: Vec<usize>,
    code: Vec<Inst>,
}

impl Program {
    /// Creates a new program from a list of instructions and their source positions.
    ///
    /// # Arguments
    ///
    /// * `carets` - Vector of source code positions (for error reporting)
    /// * `code` - Vector of instructions to execute
    ///
    /// # Returns
    ///
    /// A new Program instance
    pub fn new(carets: Vec<usize>, code: Vec<Inst>) -> Self {
        assert_eq!(carets.len(), code.len());
        Program { carets, code }
    }
}
