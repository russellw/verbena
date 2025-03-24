#!/usr/bin/env python3
import re
import sys


def main():
    if len(sys.argv) != 2:
        print("Usage: python script.py <rust_file>")
        sys.exit(1)

    # Get the filename from command line argument
    filename = sys.argv[1]

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
        # Skip the function if it starts with an underscore (like '_add', '_neg', etc.)
        # as these are already registered with the correct name

        # Count the parameters (excluding _vm)
        if not params.strip():
            arity = 0
        elif "Vec<Val>" in params:
            # Special case for variadic functions
            registration = f'    vm.registerv("{func_name}", {func_name});'
            function_registrations.append(registration)
            continue
        else:
            arity = len(params.split(","))

        # Add to registrations
        registration = f'    vm.register{arity}("{func_name}", {func_name});'
        function_registrations.append(registration)

    # Sort registrations alphabetically by function name
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
