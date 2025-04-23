"use strict"

Object.getOwnPropertyNames(Math).forEach((name) => {
	global[name] = Math[name]
})

let exit = process.exit

import assert from "assert"

function push(v, a) {
	v.push(a)
}

function _norm(a) {
	if (a === undefined) {
		return null
	}
	return a
}

function _get(a, k) {
	if (a instanceof Map) {
		return _norm(a.get(k))
	}
	return a[k]
}

function _set(a, k, x) {
	if (a instanceof Map) {
		a.set(k, x)
	} else {
		a[k] = x
	}
	return x
}

function len(a) {
	if (a instanceof Map) {
		return a.size
	}
	return a.length
}

function repr(a) {
	return JSON.stringify(a)
}

function _prin(stream, a) {
	if (typeof a !== "string") {
		a = repr(a)
	}
	stream.write(a)
}

function prin() {
	for (let a of arguments) {
		_prin(process.stdout, a)
	}
}

function eprin() {
	for (let a of arguments) {
		_prin(process.stderr, a)
	}
}

function print() {
	for (let a of arguments) {
		_prin(process.stdout, a)
	}
	_prin(process.stdout, "\n")
}

function eprint() {
	for (let a of arguments) {
		_prin(process.stderr, a)
	}
	_prin(process.stderr, "\n")
}

function string(a) {
	return a.toString()
}

function eq(a, b) {
	// Check if both values are the same reference
	if (a === b) {
		return true
	}

	// If either is null/undefined or not an object, they're not equal
	if (a == null || b == null || typeof a !== "object" || typeof b !== "object") {
		return false
	}

	// Check if they are arrays
	let aIsArray = Array.isArray(a)
	let bIsArray = Array.isArray(b)

	// Both should be arrays or both should be objects
	if (aIsArray !== bIsArray) {
		return false
	}

	if (aIsArray) {
		// Check array length
		if (a.length !== b.length) {
			return false
		}

		// Compare each element
		for (let i = 0; i < a.length; i++) {
			if (!eq(a[i], b[i])) {
				return false
			}
		}
		return true
	} else {
		// For objects (including plain objects, Maps, Sets, etc.)
		let keysA = Object.keys(a)
		let keysB = Object.keys(b)

		// Check if they have the same number of properties
		if (keysA.length !== keysB.length) {
			return false
		}

		// Check if all properties in a exist in b with the same values
		for (let key of keysA) {
			if (!Object.prototype.hasOwnProperty.call(b, key) || !eq(a[key], b[key])) {
				return false
			}
		}
		return true
	}
}

/**
 * Returns a more precise type than the native typeof operator
 * Correctly identifies:
 * - null (instead of "object")
 * - arrays (instead of "object")
 * - Maps (instead of "object")
 * - Sets (instead of "object")
 * - other special object types
 *
 * @param {any} value - The value to check
 * @return {string} The precise type of the value
 */
function _typeof(value) {
	// Handle null specially (typeof null returns "object" in JS)
	if (value === null) {
		return "null"
	}

	// Get basic type using native typeof
	let basicType = typeof value

	// If not an object, return the basic type
	if (basicType !== "object") {
		return basicType
	}

	// For objects, use Object.prototype.toString to get a more specific type
	let objectType = Object.prototype.toString.call(value)
	// Extract the type name from "[object TypeName]"
	let match = objectType.match(/^\[object\s(.*)\]$/)

	if (match) {
		let typeName = match[1]
		// Return lowercase type name for consistency with typeof
		return typeName.toLowerCase()
	}

	// Fallback to basic object if something unexpected happens
	return "object"
}

/**
 * Creates a range of numbers, similar to Python's range function.
 * @param {number} start - The start value (included if provided as first arg, otherwise 0)
 * @param {number} stop - The end value (excluded)
 * @param {number} step - The step value (default 1)
 * @returns {Array} - Array containing the range of numbers
 */
function range() {
	let start, stop, step

	// Parse arguments similar to Python's range
	if (arguments.length === 1) {
		start = 0
		stop = arguments[0]
		step = 1
	} else if (arguments.length === 2) {
		start = arguments[0]
		stop = arguments[1]
		step = 1
	} else if (arguments.length === 3) {
		start = arguments[0]
		stop = arguments[1]
		step = arguments[2]
	} else {
		throw new Error("range requires at least one argument")
	}

	// Validate inputs
	if (step === 0) {
		throw new Error("range() step argument must not be zero")
	}

	let result = []

	// Handle positive and negative steps
	if (step > 0) {
		for (let i = start; i < stop; i += step) {
			result.push(i)
		}
	} else {
		for (let i = start; i > stop; i += step) {
			result.push(i)
		}
	}

	return result
}
