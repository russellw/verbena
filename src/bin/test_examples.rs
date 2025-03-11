use std::fs;
use std::path::{Path, PathBuf};
use std::process::exit;
use std::process::Command;

fn get_subdirs(dir: &str) -> Vec<String> {
    let path = Path::new(dir);
    let mut subdirs = Vec::new();
    let entries = fs::read_dir(path).expect("Failed to read directory");
    for entry in entries {
        let entry = entry.expect("Failed to access directory entry");
        let path = entry.path();
        if path.is_dir() {
            // Get the directory name as a string
            if let Some(dir_name) = path.file_name() {
                if let Some(dir_str) = dir_name.to_str() {
                    subdirs.push(dir_str.to_string());
                } else {
                    panic!("Directory name contains invalid Unicode");
                }
            } else {
                panic!("Failed to get directory name");
            }
        }
    }
    subdirs
}

fn main() {
    let dirs = get_subdirs("examples");
    let mut passed_count = 0;
    for name in dirs {
        let program_file = PathBuf::from("examples")
            .join(&name)
            .join(format!("{}.va", name))
            .to_string_lossy()
            .to_string();
        println!("{}", program_file);
        let expected_output_file = PathBuf::from("examples")
            .join(name)
            .join("output.txt")
            .to_string_lossy()
            .to_string();
        let expected_output = match fs::read_to_string(&expected_output_file) {
            Ok(content) => content,
            Err(err) => {
                eprintln!("Failed to read {}: {}", expected_output_file, err);
                exit(1);
            }
        };
        let output = match Command::new("./target/debug/verbena")
            .arg(program_file)
            .output()
        {
            Ok(output) => output,
            Err(err) => {
                eprintln!("Failed to run interpreter: {}", err);
                exit(1);
            }
        };
        let actual_output = match String::from_utf8(output.stdout) {
            Ok(string) => string,
            Err(err) => {
                eprintln!("Actual output not valid UTF-8: {}", err);
                exit(1);
            }
        };

        // Compare outputs (trimming whitespace to handle line ending differences)
        if actual_output.trim() == expected_output.trim() {
            passed_count += 1;
        } else {
            println!(
                "Output doesn't match expected.\nExpected:\n{}\nActual:\n{}",
                expected_output, actual_output
            );
        }
    }
    println!("Passed: {}", passed_count);
}
