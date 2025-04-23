"use strict"

import fs from "fs"
import util from "util"

function dbg(a) {
	const stack = new Error().stack.split("\n")[2]
	console.log(stack.trim().replace(/^at\s+/g, ""))
	// https://nodejs.org/api/util.html#utilinspectobject-options
	console.log(
		util.inspect(a, {
			showHidden: false,
			depth: null,
			colors: true,
			maxArrayLength: null,
			maxStringLength: null,
			compact: true,
		}),
	)
}

let txt = fs.readFileSync("boot/prefix.js", "utf8")

function emit(s) {
	txt += s
}

// This function is called, recursively, once per layer of lexical scope
// that is, once for the top-level program, and once per function
function scope(params, body, topLevel) {
	let assigned = new Set()

	// Declare variables
	function decl(a) {
		if (Array.isArray(a)) {
			a.forEach(decl)
			return
		}
		if (typeof a === "object") {
			if (a.op === "fn") {
				return
			}
			a.v.forEach(decl)
			switch (a.op) {
				case "for":
					assigned.add(a.x)
					break
				case "for/2":
					assigned.add(a.i)
					assigned.add(a.x)
					break
				case "<=":
				case ">=":
				case "==":
				case "!=":
					return
			}
			if (a.op.endsWith("=") && typeof a.v[0] === "string") {
				assigned.add(a.v[0])
			}
		}
	}

	// Expressions
	function expr(a) {
		if (typeof a === "string") {
			emit(a)
			return
		}
		if (/\w/.test(a.op[0])) {
			emit(a.op)
			emit("(")
			commaSeparated(a.v)
			emit(")")
			return
		}
		switch (a.op) {
			case "{":
				emit("{")
				for (let i = 0; i < a.v.length; i++) {
					if (i) {
						emit(",")
					}
					let [key, val] = a.v[i]
					expr(key)
					emit(":")
					expr(val)
				}
				emit("}")
				return
			case "[":
				emit("[")
				commaSeparated(a.v)
				emit("]")
				return
			case "[:]":
				expr(a.v[0])
				emit(".slice(")
				expr(a.v[1])
				emit(",")
				expr(a.v[2])
				emit(")")
				return
		}
		switch (a.v.length) {
			case 1:
				emit("(")
				emit(a.op)
				expr(a.v[0])
				emit(")")
				return
			case 2:
				emit("(")
				expr(a.v[0])
				emit(a.op)
				expr(a.v[1])
				emit(")")
				return
		}
		throw a
	}

	function commaSeparated(v) {
		for (let i = 0; i < v.length; i++) {
			if (i) {
				emit(",")
			}
			expr(v[i])
		}
	}

	// Statements
	function stmt(a, last) {
		switch (a.op) {
			case "case":
				emit("switch (")
				expr(a.subject)
				emit(") {\n")
				for (let [patterns, body] of a.v) {
					for (let pattern of patterns) {
						emit("case ")
						expr(pattern)
						emit(":\n")
					}
					if (!patterns.length) {
						emit("default:\n")
					}
					block(body)
					emit("break;\n")
				}
				emit("}")
				break
			case "for":
				emit(`for (${a.x} of `)
				expr(a.xs)
				emit(")")
				block(a.v)
				break
			case "for/2":
				emit(`for ([${a.i}, ${a.x}] of `)
				expr(a.xs)
				emit(".entries())")
				block(a.v)
				break
			case "fn":
				emit(`function ${a.name}(`)
				commaSeparated(a.params)
				emit(") {\n")
				scope(a.params, a.v)
				emit("}")
				break
			case "if":
				emit("if (")
				expr(a.cond)
				emit(")")
				block(a.v[0])
				if (a.v.length > 1) {
					emit("else")
					block(a.v[1])
				}
				break
			case "import":
				emit(`import ${a.v[0]} from "./${a.v[0]}.js"`)
				break
			default:
				expr(a)
				emit(";")
				break
		}
		emit("\n")
	}

	function block(v, last) {
		emit("{\n")
		for (let i = 0; i < v.length; i++) {
			stmt(v[i], i === v.length - 1)
		}
		emit("}")
	}

	// Normalize parameters
	for (let a of params) {
		emit(`if (${a} === undefined) ${a} = null;\n`)
	}

	// Declare variables
	decl(body)
	for (let a of assigned) {
		// This is why scope() needs to know about the function parameters
		// even though it is not responsible for printing them if there is a function
		// it is responsible for declaring local variables
		// and needs to avoid re-declaring parameters
		if (!params.includes(a)) {
			emit(`let ${a} = null;\n`)
		}
	}

	// Generate code
	block(body, true)
}

// Top level
export function compile(file, v) {
	scope([], v, true)
	fs.writeFileSync(file, txt, "utf8")
}
