"use strict"

import { writeFileSync } from "fs"

let txt = ""

function emit(s) {
	txt += s
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
	writeFileSync(file, txt, "utf8")
}
