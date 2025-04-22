"use strict"

import fs from "fs"

let txt = ""

function emit(s) {
	txt += s
}

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
	for (let i = 0; i < v.length - 1; i++) {
		stmt(v[i])
	}
	stmt(v.at(-1), true)
}

// Top level
export function compile(file, v) {
	block(v)
	fs.writeFileSync(file, txt, "utf8")
}
