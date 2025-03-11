use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::exit;
use std::process::Command;

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
        Err(err) => {
            println!("Error reading subdirectories: {}", err);
            std::process::exit(1);
        }
    };

    let mut passed_count = 0;

    // For each example program
    for name in dirs {
        let program_file = PathBuf::from("examples")
            .join(&name)
            .join(format!("{}.va", name))
            .into_os_string()
            .into_string()
            .expect("Path contains invalid UTF-8");
        let expected_output_file = PathBuf::from("examples")
            .join(name)
            .join("output.txt")
            .into_os_string()
            .into_string()
            .expect("Path contains invalid UTF-8");
        let expected_output = match fs::read_to_string(&expected_output_file) {
            Ok(content) => content,
            Err(err) => {
                println!("Failed to read {}: {}", expected_output_file, err);
                exit(1);
            }
        };

        // Run the program
        let output = match Command::new("./target/debug/verbena")
            .arg(&program_file)
            .output()
        {
            Ok(output) => output,
            Err(err) => {
                println!("{}", program_file);
                println!("Failed to run interpreter: {}", err);
                exit(1);
            }
        };
        let actual_output = match String::from_utf8(output.stdout) {
            Ok(string) => string,
            Err(err) => {
                println!("{}", program_file);
                println!("Actual output not valid UTF-8: {}", err);
                exit(1);
            }
        };
        let stderr_output = match String::from_utf8(output.stderr) {
            Ok(string) => string,
            Err(err) => {
                println!("{}", program_file);
                println!("stderr output not valid UTF-8: {}", err);
                exit(1);
            }
        };

        //Error
        if !stderr_output.is_empty() {
            println!("{}", program_file);
            print!("{}", stderr_output);
            exit(1);
        }

        // Compare outputs
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
    println!("Passed: {}", passed_count);
}
