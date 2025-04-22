import fs from 'fs';
import path from 'path';
import { spawn, spawnSync } from 'child_process';
import { fileURLToPath } from 'url';
import { dirname } from 'path';

// Get the directory name equivalent to Python's __file__
const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

/**
 * Get all Verbena (.va) files in the specified directory.
 * @param {string} directory - The directory to search for .va files
 * @returns {string[]} - A list of file names without the .va extension
 * @throws {Error} - If there's an error reading the directory
 */
function getExampleFiles(directory) {
  const exampleFiles = [];
  try {
    const entries = fs.readdirSync(directory);
    for (const entry of entries) {
      if (entry.endsWith(".va")) {
        // Remove the .va extension to get the base name
        exampleFiles.push(entry.slice(0, -3));
      }
    }
  } catch (e) {
    throw new Error(`Error reading directory: ${e.message}`);
  }
  return exampleFiles;
}

async function main() {
  // Get a list of the example programs
  let exampleNames;
  try {
    exampleNames = getExampleFiles("test");
  } catch (e) {
    console.error(e.message);
    process.exit(1);
  }

  let passedCount = 0;
  const skipped = [];

  // For each example program
  for (const name of exampleNames) {
    // Check if corresponding output file exists
    const expectedOutputFile = path.join("test_output", `${name}.txt`);
    if (!fs.existsSync(expectedOutputFile)) {
      skipped.push(name);
      continue;
    }

    let expectedOutput;
    try {
      expectedOutput = fs.readFileSync(expectedOutputFile, 'utf8');
    } catch (e) {
      console.error(`Failed to read ${expectedOutputFile}: ${e.message}`);
      process.exit(1);
    }

    // Get the program file path
    const programFile = path.join("test", `${name}.va`);

    // First, compile the Verbena source to a.mjs
    try {
      const compileProc = spawnSync(
        "./target/debug/verbena", 
        [programFile],
        { encoding: 'utf8' }
      );
      
      if (compileProc.status !== 0) {
        console.error(`${programFile}`);
        console.error(`Failed to compile: ${compileProc.stderr}`);
        process.exit(1);
      }
    } catch (e) {
      console.error(`${programFile}`);
      console.error(`Failed to run compiler: ${e.message}`);
      process.exit(1);
    }

    // Now run the compiled JavaScript with node
    try {
      const proc = spawn('node', [`target/${name}.mjs`]);
      
      let stdout = '';
      let stderr = '';

      proc.stdout.on('data', (data) => {
        stdout += data.toString();
      });

      proc.stderr.on('data', (data) => {
        stderr += data.toString();
      });

      // Wait for the process to finish
      const exitCode = await new Promise((resolve) => {
        proc.on('close', (code) => {
          resolve(code);
        });
      });

      // Error
      if (stderr) {
        console.error(`${programFile}`);
        console.error(stderr);
        process.exit(1);
      }

      if (exitCode !== 0) {
        console.error(`${programFile}`);
        console.error(`Exit code ${exitCode}`);
        process.exit(1);
      }

      // Compare outputs
      if (stdout === expectedOutput) {
        passedCount++;
      } else {
        console.error(`${programFile}`);
        console.error(
          `Output doesn't match expected.\nExpected:\n${expectedOutput}\nActual:\n${stdout}`
        );
      }
    } catch (e) {
      console.error(`${programFile}`);
      console.error(`Failed during program execution: ${e.message}`);
      process.exit(1);
    }
  }

  console.log(`Passed : ${passedCount}`);
  console.log(`Skipped: ${skipped.length}`);
  for (const name of skipped) {
    console.log(name);
  }
}

// Equivalent to Python's if __name__ == "__main__"
if (import.meta.url === `file://${process.argv[1]}`) {
  main().catch(err => {
    console.error(err);
    process.exit(1);
  });
}

export { getExampleFiles };