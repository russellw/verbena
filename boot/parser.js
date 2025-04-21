"use strict"

import { readFileSync } from "fs"

function isIdStart(c) {
	return /[a-zA-Z_$]/.test(c)
}

function isIdPart(c) {
	return /[a-zA-Z0-9_$]/.test(c)
}

function make(op,v){
	return[op,v]
}

const eof = " "

let file
let txt
let pos = 0
let line = 1
let tok

function err(msg) {
	throw `${file}:${line}: ${msg}`
}

// Tokenizer
function lex() {
	while (pos < txt.length) {
		const i = pos
		const c = txt[pos]
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
					const c = txt[pos]
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
		if (isIdStart(c)) {
			do {
				pos++
			} while (isIdPart(txt[pos]))
			tok = txt.slice(i, pos)
			return
		}

		// Punctuation
		const punct = [
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
		for (const s of punct) {
			const n = s.length
			if (s === txt.slice(pos, pos + n)) {
				tok = s
				pos += n
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
	const s = tok
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
	switch (tok[0]) {
		case '"':
		case "'":
			return lex1()
		case "(":
			lex()
			const a = expr()
			expect(")")
			return a
	}
	if (isIdPart(tok[0])) {
		return lex1()
	}
	err("Expected expression")
}

function postfix() {
	let a = primary()
	for (;;) {
		switch (tok) {
			case "(":
				lex()
				let v = commaSeparated(")")
				expect(")")
				break
			default:
				return a
		}
	}
}

function expr() {
	return postfix()
}

function commaSeparated(end) {
	const v = []
	if (tok !== end) {
		do {
			v.push(expr())
		} while (eat(","))
	}
	return v
}

// Top level
export function parse(file1) {
	file = file1
	txt = readFileSync(file, "utf8") + "\n"
	lex()
	console.log(tok)
}
