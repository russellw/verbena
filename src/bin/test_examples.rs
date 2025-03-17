use std::fs;
use std::io;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::exit;
use std::process::{Command, Stdio};

fn get_subdirs(dir: &str) -> Result<Vec<String>, io::Error> {
    let path = Path::new(dir);
    let mut subdirs = Vec::new();
    let entries = fs::read_dir(path)?;
    for entry_result in entries {
        let entry = entry_result?;
        let path = entry.path();
        if path.is_dir() {
            // Get the directory name as a string
            if let Some(dir_name) = path.file_name() {
                if let Some(dir_str) = dir_name.to_str() {
                    subdirs.push(dir_str.to_string());
                } else {
                    // Handle non-Unicode directory names
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("Directory name contains invalid Unicode: {:?}", dir_name),
                    ));
                }
            } else {
                // Handle paths with no file name component
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Failed to get directory name from path: {:?}", path),
                ));
            }
        }
    }
    Ok(subdirs)
}

fn main() {
    // Get a list of the example programs
    let dirs = match get_subdirs("examples") {
        Ok(dirs) => dirs,
        Err(e) => {
            println!("Error reading subdirectories: {}", e);
            std::process::exit(1);
        }
    };

    let mut passed_count = 0;
    let mut skipped_count = 0;

    // For each example program
    for name in dirs {
        // If output.txt exists, use it as the basis for comparison
        let expected_output_file = PathBuf::from("examples").join(&name).join("output.txt");
        if !expected_output_file.exists() {
            skipped_count += 1;
            continue;
        }
        let expected_output_file = expected_output_file
            .into_os_string()
            .into_string()
            .expect("Path contains invalid UTF-8");
        let expected_output = match fs::read_to_string(&expected_output_file) {
            Ok(content) => content,
            Err(e) => {
                println!("Failed to read {}: {}", expected_output_file, e);
                exit(1);
            }
        };

        // If input.txt exists, use it for input
        let input_file = PathBuf::from("examples").join(&name).join("input.txt");
        let input = if input_file.exists() {
            let input_file = input_file
                .into_os_string()
                .into_string()
                .expect("Path contains invalid UTF-8");
            match fs::read_to_string(&input_file) {
                Ok(content) => content,
                Err(e) => {
                    println!("Failed to read {}: {}", input_file, e);
                    exit(1);
                }
            }
        } else {
            "".to_string()
        };

        // Run the program
        let program_file = PathBuf::from("examples")
            .join(&name)
            .join(format!("{}.va", name))
            .into_os_string()
            .into_string()
            .expect("Path contains invalid UTF-8");

        // Create a command with piped stdin
        let mut cmd = Command::new("./target/debug/verbena");
        cmd.arg(&program_file)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        // Spawn the command
        let mut child = match cmd.spawn() {
            Ok(child) => child,
            Err(e) => {
                println!("{}", program_file);
                println!("Failed to run interpreter: {}", e);
                exit(1);
            }
        };

        // Write the input to stdin
        if !input.is_empty() {
            if let Some(stdin) = child.stdin.as_mut() {
                if let Err(e) = stdin.write_all(input.as_bytes()) {
                    println!("{}", program_file);
                    println!("Failed to write to stdin: {}", e);
                    exit(1);
                }
            }
        }

        // Wait for the command to complete and collect output
        let output = match child.wait_with_output() {
            Ok(output) => output,
            Err(e) => {
                println!("{}", program_file);
                println!("Failed to wait for interpreter: {}", e);
                exit(1);
            }
        };

        let actual_output = match String::from_utf8(output.stdout) {
            Ok(string) => string,
            Err(e) => {
                println!("{}", program_file);
                println!("Actual output not valid UTF-8: {}", e);
                exit(1);
            }
        };
        let stderr_output = match String::from_utf8(output.stderr) {
            Ok(string) => string,
            Err(e) => {
                println!("{}", program_file);
                println!("stderr output not valid UTF-8: {}", e);
                exit(1);
            }
        };
        let exit_code = output.status.code();

        // Error
        if !stderr_output.is_empty() {
            println!("{}", program_file);
            print!("{}", stderr_output);
            exit(1);
        }
        if exit_code != Some(0) {
            println!("{}", program_file);
            println!("Exit code {:?}", exit_code);
            exit(1);
        }

        // Compare outputs
        // Watch out for output.txt's with MS-DOS line endings!
        if actual_output == expected_output {
            passed_count += 1;
        } else {
            println!("{}", program_file);
            println!(
                "Output doesn't match expected.\nExpected:\n{}\nActual:\n{}",
                expected_output, actual_output
            );
        }
    }
    println!("Passed : {}", passed_count);
    println!("Skipped: {}", skipped_count);
}
