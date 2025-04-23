import fs from "fs"
import path from "path"
import { spawn, spawnSync } from "child_process"
import { fileURLToPath } from "url"
import { dirname } from "path"

// Get the directory name equivalent to Python's __file__
let __filename = fileURLToPath(import.meta.url)
let __dirname = dirname(__filename)

/**
 * Get all Verbena (.va) files in the specified directory.
 * @param {string} directory - The directory to search for .va files
 * @returns {string[]} - A list of file names without the .va extension
 * @throws {Error} - If there's an error reading the directory
 */
function getExampleFiles(directory) {
	let exampleFiles = []
	try {
		let entries = fs.readdirSync(directory)
		for (let entry of entries) {
			if (entry.endsWith(".va")) {
				// Remove the .va extension to get the base name
				exampleFiles.push(entry.slice(0, -3))
			}
		}
	} catch (e) {
		throw new Error(`Error reading directory: ${e.message}`)
	}
	return exampleFiles
}

// Get a list of the example programs
let exampleNames
try {
	exampleNames = getExampleFiles("test")
} catch (e) {
	console.error(e.message)
	process.exit(1)
}

let passedCount = 0
let skipped = []

// For each example program
for (let name of exampleNames) {
	// Check if corresponding output file exists
	let expectedOutputFile = path.join("test-output", `${name}.txt`)
	if (!fs.existsSync(expectedOutputFile)) {
		skipped.push(name)
		continue
	}

	let expectedOutput
	try {
		expectedOutput = fs.readFileSync(expectedOutputFile, "utf8")
	} catch (e) {
		console.error(`Failed to read ${expectedOutputFile}: ${e.message}`)
		process.exit(1)
	}

	// Get the program file path
	let programFile = path.join("test", `${name}.va`)

	// First, compile the Verbena source
	try {
		let compileProc = spawnSync("node", ["boot/main.js", programFile], { encoding: "utf8" })

		if (compileProc.status !== 0) {
			console.error(`${programFile}`)
			console.error(`Failed to compile: ${compileProc.stderr}`)
			process.exit(1)
		}
	} catch (e) {
		console.error(`${programFile}`)
		console.error(`Failed to run compiler: ${e.message}`)
		process.exit(1)
	}

	// Now run the compiled JavaScript with node
	try {
		let proc = spawn("node", [`build/${name}.js`])

		let stdout = ""
		let stderr = ""

		proc.stdout.on("data", (data) => {
			stdout += data.toString()
		})

		proc.stderr.on("data", (data) => {
			stderr += data.toString()
		})

		// Wait for the process to finish
		let exitCode = await new Promise((resolve) => {
			proc.on("close", (code) => {
				resolve(code)
			})
		})

		// Error
		if (stderr) {
			console.error(`${programFile}`)
			console.error(stderr)
			process.exit(1)
		}

		if (exitCode !== 0) {
			console.error(`${programFile}`)
			console.error(`Exit code ${exitCode}`)
			process.exit(1)
		}

		// Compare outputs
		if (stdout === expectedOutput) {
			passedCount++
		} else {
			console.error(`${programFile}`)
			console.error(`Output doesn't match expected.\nExpected:\n${expectedOutput}\nActual:\n${stdout}`)
		}
	} catch (e) {
		console.error(`${programFile}`)
		console.error(`Failed during program execution: ${e.message}`)
		process.exit(1)
	}
}

console.log(`Passed : ${passedCount}`)
console.log(`Skipped: ${skipped.length}`)
for (let name of skipped) {
	console.log(name)
}
