"use strict"

import fs from "fs"

let txt = fs.readFileSync("boot/prefix.js", "utf8")

function emit(s) {
	txt += s
}

function fn(params, body, topLevel) {
	// Expressions
	function expr(a) {
		// Atom
		if (typeof a === "string") {
			emit(a)
			return
		}

		// Function call
		if (/\w/.test(a.op[0])) {
			emit(a.op)
			emit("(")
			commaSeparated(a.v)
			emit(")")
			return
		}

		// Prefix
		if (a.v.length === 1) {
			emit("(")
			emit(a.op)
			expr(a.v[0])
			emit(")")
			return
		}

		// Infix
		if (a.v.length === 2) {
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
			case "import":
				emit(`import ${a.v[0]} from "./${a.v[0]}.js"`)
				break
			default:
				expr(a)
				break
		}
		emit("\n")
	}

	function block(v, last) {
		for (let i = 0; i < v.length; i++) {
			stmt(v[i], i === v.length - 1)
		}
	}

	// Normalize parameters
	for (const a of params) {
		emit(`if (${a} === undefined) ${a} = null\n`)
	}

	// Generate code
	block(body, true)
}

// Top level
export function compile(file, v) {
	fn([], v, true)
	fs.writeFileSync(file, txt, "utf8")
}
