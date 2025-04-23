"use strict"

import assert from "assert"
import fs from "fs"

function make(op, ...v) {
	return { op, v }
}

// End of file is represented as a token of positive length
// so that inspecting its first character has predictable results
// that is distinct from any actual token
let eof = " "

let file
let txt
let pos = 0
let line = 1
let tok

function err(msg) {
	throw `${file}:${line}: '${tok}': ${msg}`
}

// Tokenizer
function lex() {
	while (pos < txt.length) {
		let i = pos
		let c = txt[pos]
		switch (c) {
			case '"':
			case "'":
				pos++
				while (txt[pos] !== c) {
					switch (txt[pos]) {
						case "\n":
							err("Unterminated string")
							break
						case "\\":
							// Backslash can escape many things
							// but most of them can be left to the JavaScript compiler to interpret
							// The only things we need to worry about here are:
							// Escaping a closing quote
							// Escaping a backslash that might otherwise escape a closing quote
							if (txt[pos] === c || txt[pos] === "\\") {
								pos++
							}
							break
					}
					pos++
				}
				pos++
				tok = txt.slice(i, pos)
				return
			case "#":
			case "\n":
				// End of line is a token
				// but, to simplify the parser, blank lines are not tokens
				while (pos < txt.length) {
					let c = txt[pos]
					if (/\s/.test(c)) {
						if (c === "\n") {
							line++
						}
						pos++
					} else if (c === "#") {
						while (txt[pos] !== "\n") {
							pos++
						}
					} else {
						break
					}
				}
				tok = "\n"
				return
		}

		// Space
		if (/\s/.test(c)) {
			pos++
			continue
		}

		// Word
		let s = txt.slice(pos)
		let m = s.match(/^[a-zA-Z_$][a-zA-Z0-9_$]*/)
		if (m) {
			tok = m[0]
			pos += tok.length
			return
		}

		// Number
		m = s.match(/^(0[box]\w+|\d+\.?\d*(e[+-]?\d+)?)/i)
		if (m) {
			tok = m[0]
			pos += tok.length
			return
		}

		// Punctuation
		let punct = [
			">>>=",

			">>=",
			"<<=",
			"**=",
			">>>",

			"<<",
			">>",
			"<=",
			">=",
			"==",
			"!=",
			"**",
			"+=",
			"-=",
			"/=",
			"&=",
			"|=",
			"^=",
			"*=",
			"%=",
			"&&",
			"||",
		]
		for (let s of punct) {
			if (s === txt.slice(pos, pos + s.length)) {
				tok = s
				pos += tok.length
				return true
			}
		}

		tok = c
		pos++
		return
	}
	tok = eof
}

function lex1() {
	let s = tok
	lex()
	return s
}

function eat(s) {
	if (tok === s) {
		lex()
		return true
	}
}

function expect(s) {
	if (!eat(s)) {
		err(`Expected '${s}'`)
	}
}

// Expressions
function primary() {
	let a
	switch (tok[0]) {
		case '"':
		case "'":
			return lex1()
		case "(":
			lex()
			a = expr()
			expect(")")
			return a
		case "{":
			a = make(lex1())
			if (tok !== "}") {
				do {
					let key = expr()
					expect(":")
					let val = expr()
					a.v.push([key, val])
				} while (eat(","))
			}
			expect("}")
			return a
		case "[":
			a = make(lex1())
			a.v = commaSeparated("]")
			return a
	}
	if (/\w/.test(tok[0])) {
		return lex1()
	}
	err("Expected expression")
}

function postfix() {
	let a = primary()
	for (;;) {
		switch (tok) {
			case ".":
				lex()
				let s = `"${lex1()}"`
				a = make("]", a, s)
				break
			case "[":
				lex()
				a = make("_get", a)

				// First subscript
				a.v.push(tok === ":" ? "undefined" : expr())

				// Second subscript?
				if (eat(":")) {
					a.op = "[:]"
					a.v.push(expr())
				}

				expect("]")
				break
			case "(":
				lex()
				a = make(a, ...commaSeparated(")"))
				break
			default:
				return a
		}
	}
}

function prefix() {
	switch (tok) {
		case "~":
		case "!":
		case "-":
			return make(lex1(), prefix())
	}
	return postfix()
}

// Operator precedence parser
let prec = 99
let ops = new Map()

function addOp(s, left = 1) {
	let o = {
		prec,
		left,
	}
	ops.set(s, o)
}

addOp("**", 0)

prec--
addOp("*")
addOp("/")
addOp("%")

prec--
addOp("+")
addOp("-")

prec--
addOp("<<")
addOp(">>")

prec--
addOp("&")

prec--
addOp("^")

prec--
addOp("|")

prec--
addOp("==")
addOp("!=")
addOp("<")
addOp("<=")
addOp(">")
addOp(">=")

prec--
addOp("&&")

prec--
addOp("||")

prec--
addOp("=", 0)

addOp("**=", 0)

addOp("*=", 0)
addOp("/=", 0)
addOp("%=", 0)

addOp("+=", 0)
addOp("-=", 0)

addOp("<<=", 0)
addOp(">>=", 0)
addOp(">>>=", 0)

addOp("&=", 0)

addOp("^=", 0)

addOp("|=", 0)

function expr(prec = 0) {
	let a = prefix()
	for (;;) {
		let o = ops.get(tok)
		if (!o || o.prec < prec) {
			return a
		}
		let op = lex1()
		let b = expr(o.prec + o.left)
		a = make(op, a, b)
	}
}

function commaSeparated(end) {
	let v = []
	if (tok !== end) {
		do {
			v.push(expr())
		} while (eat(","))
	}
	expect(end)
	return v
}

// Statements
function blockEnd() {
	switch (tok) {
		case "|":
		case "catch":
		case "else":
		case "elif":
		case "end":
			return true
	}
	return tok === eof
}

function if1() {
	assert(tok === "if" || tok === "elif")
	let a = make("if")
	lex()
	a.cond = expr()
	expect("\n")
	a.v.push(block())
	switch (tok) {
		case "elif":
			a.v.push([if1()])
			break
		case "else":
			lex()
			expect("\n")
			a.v.push(block())
			expect("end")
			break
		default:
			expect("end")
			break
	}
	return a
}

function stmt() {
	let a = make(tok)
	switch (tok) {
		case "fn":
			lex()
			a.name = lex1()
			expect("(")
			a.params = commaSeparated(")")
			expect("\n")
			a.v = block()
			expect("end")
			break
		case "for":
			lex()
			a.x = lex1()
			if (eat(",")) {
				a.op = "for/2"
				a.i = a.x
				a.x = lex1()
			}
			expect(":")
			a.xs = expr()
			expect("\n")
			a.v = block()
			expect("end")
			break
		case "case":
			lex()
			a.subject = expr()
			expect("\n")
			while (!eat("end")) {
				let patterns = []
				switch (tok) {
					case "|":
						lex()
						patterns = commaSeparated("\n")
						break
					case "else":
						lex()
						expect("\n")
						break
				}
				a.v.push([patterns, block()])
			}
			break
		case "return":
			lex()
			a.v.push(tok === "\n" ? "null" : expr())
			break
		case "dowhile":
		case "while":
			lex()
			a.cond = expr()
			expect("\n")
			a.v = block()
			expect("end")
			break
		case "if":
			a = if1()
			break
		default:
			a = expr()
			switch (tok) {
				case ":":
					a = make(lex1(), a)
					break
				case "\n":
					break
				default:
					return make(a, ...commaSeparated("\n"))
			}
	}
	expect("\n")
	return a
}

function block() {
	let v = []
	while (!blockEnd()) {
		v.push(stmt())
	}
	return v
}

// Top level
export function parse(file1) {
	file = file1
	txt = fs.readFileSync(file, "utf8") + "\n"
	lex()
	eat("\n")
	let v = block()
	if (tok !== eof) {
		err("Unmatched terminator")
	}
	return v
}
