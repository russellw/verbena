#!/usr/bin/env python3
import re
import sys


def main():
    if len(sys.argv) == 2:
        filename = sys.argv[1]
    else:
        filename = "src/stdlib.rs"

    try:
        with open(filename, "r") as file:
            content = file.read()
    except Exception as e:
        print(f"Error reading file: {e}")
        sys.exit(1)

    # Find all function definitions
    function_pattern = r"fn\s+([a-zA-Z0-9_]+)\s*\(\s*_?vm\s*:\s*&mut\s+VM(?:\s*,\s*([^)]+))?\)\s*->\s*Result<Val,\s*String>"
    functions = re.findall(function_pattern, content)

    # Analyze each function to determine its arity
    function_registrations = []
    for func_name, params in functions:
        # Strip trailing underscores for the quoted name but keep the original name for the function reference
        quoted_name = func_name.rstrip("_")

        # Convert leading "is_" to trailing "?" in the quoted name
        if quoted_name.startswith("is_"):
            quoted_name = quoted_name[3:] + "?"

        # Count the parameters (excluding _vm)
        if not params.strip():
            arity = 0
        elif "Vec<Val>" in params:
            # Special case for variadic functions
            registration = f'    vm.registerv("{quoted_name}", {func_name});'
            function_registrations.append(registration)
            continue
        else:
            arity = len(params.split(","))

        # Add to registrations
        registration = f'    vm.register{arity}("{quoted_name}", {func_name});'
        function_registrations.append(registration)

    # Sort registrations alphabetically by function name (the quoted name without underscores)
    function_registrations.sort(key=lambda x: re.search(r'"([^"]+)"', x).group(1))

    # Generate the new register_all function
    new_register_all = "pub fn register_all(vm: &mut VM) {\n"
    new_register_all += "\n".join(function_registrations)
    new_register_all += "\n}"

    # Replace the existing register_all function in the content
    register_all_pattern = r"pub fn register_all\(vm: &mut VM\) \{[^}]*\}"
    updated_content = re.sub(register_all_pattern, new_register_all, content)

    # Write the updated content back to the file
    try:
        with open(filename, "w", newline="\n") as file:
            file.write(updated_content)
        print(f"Successfully updated {filename}")
    except Exception as e:
        print(f"Error writing to file: {e}")
        sys.exit(1)


if __name__ == "__main__":
    main()
