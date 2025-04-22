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
		emit(")")
		return
	}
}

// Statements
function stmt(a) {
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

// Top level
export function compile(file, v) {
	writeFileSync(file, txt, "utf8")
}
