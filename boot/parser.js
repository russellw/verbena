"use strict"

import { readFileSync } from "fs"

const eof = " "

let file
let txt
let pos = 0
let line = 1
let tok

export function parse(file1) {
	file = file1
	txt = readFileSync(file, "utf8") + "\n"
	lex()
}

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

		// Space
		if (/\s/.test(c)) {
			pos++
			continue
		}

		// Punctuation
		const punct = [">>>=", 
		
		">>=", "<<=", "**=", ">>>",
		
		'<<','>>','<=','>=','==','!=','**','+=','-=','/=','*=','%=',
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
