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

function fn(params, body, topLevel) {
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
		dbg(a)
		if (/\w/.test(a.op[0])) {
			emit(a.op)
			emit("(")
			commaSeparated(a.v)
			emit(")")
			return
		}
		switch (a.op) {
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
		dbg(a)
		switch (a.op) {
			case "fn":
				emit(`function ${a.name}(`)
				commaSeparated(a.params)
				emit(")")
				block(a.v)
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
		dbg(v)
		emit("{\n")
		for (let i = 0; i < v.length; i++) {
			stmt(v[i], i === v.length - 1)
		}
		emit("}")
	}

	// Normalize parameters
	for (let a of params) {
		emit(`if (${a} === undefined) ${a} = null\n`)
	}

	// Declare variables
	decl(body)
	for (let a of assigned) {
		if (!params.includes(a)) {
			emit(`let ${a} = null;\n`)
		}
	}

	// Generate code
	block(body, true)
}

// Top level
export function compile(file, v) {
	fn([], v, true)
	fs.writeFileSync(file, txt, "utf8")
}
