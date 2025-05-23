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
	let outers = new Set()
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
				case "outer":
					outers.add(a.v[0])
					break
				case "for":
					assigned.add(a.x)
					break
				case "for/2":
					assigned.add(a.i)
					assigned.add(a.x)
					break
				case "=":
					if (typeof a.v[0] === "string") {
						assigned.add(a.v[0])
					}
					break
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
				emit("new Map([")
				for (let i = 0; i < a.v.length; i++) {
					if (i) {
						emit(",")
					}
					let [key, val] = a.v[i]
					emit("[")
					expr(key)
					emit(",")
					expr(val)
					emit("]")
				}
				emit("])")
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
			case "=":
				let x = a.v[0]
				let y = a.v[1]
				if (x.op === "_get") {
					emit("_set(")
					expr(x.v[0])
					emit(",")
					expr(x.v[1])
					emit(",")
					expr(y)
					emit(")")
					return
				}
				break
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
			case "while":
				emit("while (")
				expr(a.cond)
				emit(")")
				block(a.v)
				break
			case "dowhile":
				emit("do")
				block(a.v)
				emit("while (")
				expr(a.cond)
				emit(")")
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
				block(a.v[0], last)
				if (a.v.length > 1) {
					emit("else")
					block(a.v[1], last)
				}
				break
			case "outer":
				return
			case "import":
				emit(`import ${a.v[0]} from "./${a.v[0]}.js"`)
				break
			case "return":
				emit("return ")
				expr(a.v[0])
				emit(";")
				break
			default:
				if (last && !topLevel) {
					emit("return ")
				}
				expr(a)
				emit(";")
				break
		}
		emit("\n")
	}

	function block(v, last) {
		emit("{\n")
		for (let i = 0; i < v.length; i++) {
			stmt(v[i], last && i === v.length - 1)
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
		if (!(params.includes(a) || outers.has(a))) {
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
