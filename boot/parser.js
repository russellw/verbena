"use strict"

import { readFileSync } from "fs"

const eof=' '

let file
let txt
let ti=0
let line=1
let tok

function lex(){
	while(ti<txt.length){
		const c=txt[ti]
		switch(c){
			case'\n':
			line++
			ti++
			continue
		}
		if  (/\s/.test(c)){
			ti++
			continue
		}
		tok=c
		ti++
		return
	}
	tok=eof
}

export function parse(file1) {
	file = file1
	txt = readFileSync(file, "utf8") + "\n"
	lex()
}
