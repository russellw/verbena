import os
import subprocess
import sys
from pathlib import Path


def get_subdirs(directory: str) -> list[str]:
    """
    Get all subdirectories in the specified directory.

    Args:
        directory: The directory to search for subdirectories

    Returns:
        A list of subdirectory names (not full paths)

    Raises:
        OSError: If there's an error reading the directory
    """
    path = Path(directory)
    subdirs = []

    try:
        entries = os.listdir(path)
        for entry in entries:
            entry_path = path / entry
            if entry_path.is_dir():
                subdirs.append(entry)
    except OSError as e:
        raise OSError(f"Error reading subdirectories: {e}")

    return subdirs


def main():
    # Get a list of the example programs
    try:
        dirs = get_subdirs("examples")
    except OSError as e:
        print(e)
        sys.exit(1)

    passed_count = 0
    skipped_count = 0

    # For each example program
    for name in dirs:
        # If output.txt exists, use it as the basis for comparison
        expected_output_file = Path("examples") / name / "output.txt"
        if not expected_output_file.exists():
            skipped_count += 1
            continue

        try:
            expected_output = expected_output_file.read_text()
        except OSError as e:
            print(f"Failed to read {expected_output_file}: {e}")
            sys.exit(1)

        # If input.txt exists, use it for input
        input_file = Path("examples") / name / "input.txt"
        if input_file.exists():
            try:
                input_data = input_file.read_text()
            except OSError as e:
                print(f"Failed to read {input_file}: {e}")
                sys.exit(1)
        else:
            input_data = ""

        # Run the program
        program_file = Path("examples") / name / f"{name}.va"

        # Create a subprocess with piped stdin/stdout/stderr
        try:
            proc = subprocess.Popen(
                ["./target/debug/verbena", str(program_file)],
                stdin=subprocess.PIPE,
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
                text=True,  # This ensures string I/O instead of bytes
            )
        except Exception as e:
            print(f"{program_file}")
            print(f"Failed to run interpreter: {e}")
            sys.exit(1)

        # Write the input to stdin
        try:
            stdout, stderr = proc.communicate(input=input_data)
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
