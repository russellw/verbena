import os
import subprocess
import sys
from pathlib import Path


def get_example_files(directory: str) -> list[str]:
    """
    Get all Verbena (.va) files in the specified directory.
    Args:
        directory: The directory to search for .va files
    Returns:
        A list of file names without the .va extension
    Raises:
        OSError: If there's an error reading the directory
    """
    path = Path(directory)
    example_files = []
    try:
        for entry in os.listdir(path):
            if entry.endswith(".va"):
                # Remove the .va extension to get the base name
                example_files.append(entry[:-3])
    except OSError as e:
        raise OSError(f"Error reading directory: {e}")
    return example_files


def main():
    # Get a list of the example programs
    try:
        example_names = get_example_files("examples")
    except OSError as e:
        print(e)
        sys.exit(1)

    passed_count = 0
    skipped_count = 0

    # For each example program
    for name in example_names:
        # Check if corresponding output file exists
        expected_output_file = Path("examples_output") / f"{name}.txt"
        if not expected_output_file.exists():
            skipped_count += 1
            continue

        try:
            expected_output = expected_output_file.read_text()
        except OSError as e:
            print(f"Failed to read {expected_output_file}: {e}")
            sys.exit(1)

        # Get the program file path
        program_file = Path("examples") / f"{name}.va"

        # First, compile the Verbena source to a.js
        try:
            compile_proc = subprocess.run(
                ["./target/debug/verbena", str(program_file)],
                capture_output=True,
                text=True,
            )

            if compile_proc.returncode != 0:
                print(f"{program_file}")
                print(f"Failed to compile: {compile_proc.stderr}")
                sys.exit(1)
        except Exception as e:
            print(f"{program_file}")
            print(f"Failed to run compiler: {e}")
            sys.exit(1)

        # Now run the compiled JavaScript with node
        try:
            proc = subprocess.Popen(
                ["node", "a.js"],
                stdin=subprocess.PIPE,
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
                text=True,  # This ensures string I/O instead of bytes
            )
        except Exception as e:
            print(f"{program_file}")
            print(f"Failed to run node on compiled output: {e}")
            sys.exit(1)

        # Write empty input to stdin
        try:
            stdout, stderr = proc.communicate(input="")
        except Exception as e:
            print(f"{program_file}")
            print(f"Failed during program execution: {e}")
            sys.exit(1)

        exit_code = proc.returncode

        # Error
        if stderr:
            print(f"{program_file}")
            print(stderr, end="")
            sys.exit(1)

        if exit_code != 0:
            print(f"{program_file}")
            print(f"Exit code {exit_code}")
            sys.exit(1)

        # Compare outputs
        if stdout == expected_output:
            passed_count += 1
        else:
            print(f"{program_file}")
            print(
                f"Output doesn't match expected.\nExpected:\n{expected_output}\nActual:\n{stdout}"
            )

    print(f"Passed : {passed_count}")
    print(f"Skipped: {skipped_count}")


if __name__ == "__main__":
    main()
