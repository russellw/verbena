"use strict"

import { readFileSync } from "fs"

const eof = " "

let file
let txt
let pos = 0
let line = 1
let tok

function lex() {
	while (pos < txt.length) {
		const c = txt[pos]
		switch (c) {
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
		if (/\s/.test(c)) {
			pos++
			continue
		}
		tok = c
		pos++
		return
	}
	tok = eof
}

export function parse(file1) {
	file = file1
	txt = readFileSync(file, "utf8") + "\n"
	lex()
}
