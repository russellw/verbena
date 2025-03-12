# Complex Arithmetic Expressions with Brackets

# We may reasonably assume Python is already debugged
# and use it for a reference implementation

# However, output had to be patched by hand to remove trailing .0's

# Mixed operations with brackets controlling order of operations
print((2 + 3) * 4)         # Addition in brackets, then multiplication
print(2 + (3 * 4))         # Multiplication in brackets, then addition
print(10 / (2 + 3))        # Addition in brackets, then division
print((10 / 2) + 3)        # Division in brackets, then addition

# Nested brackets
print(((2 + 3) * 4) - 6)   # Inner brackets, then multiplication, then subtraction
print(2 ** (3 + 1))        # Addition in brackets, then exponentiation
print(2 ** (3 ** 2))       # Inner exponentiation, then outer exponentiation
print((10 // 3) * (4 + 2)) # Two bracketed expressions, then multiplication

# Complex expressions with multiple operations
print(((5 + 3) * 2) / (7 - 3))      # Multiple brackets and operations
print(2 + 3 * 4 / (8 - 6) ** 2)     # Mixed operations with brackets
print(10 % (5 + (2 * 1)))           # Nested brackets with modulus
print((7 + 3) % (9 - 2 * 2))        # Brackets with mixed operations and modulus

# Expressions with bitwise operations
print((5 & 3) | (4 & 2))            # Bitwise operations in brackets with OR
print(~(5 & 7))                     # Bitwise NOT of a bracketed expression
print((5 << 2) + (3 >> 1))          # Shift operations in brackets, then addition
print(((5 | 2) & 6) ^ (~3 & 7))     # Complex nested bitwise operations
