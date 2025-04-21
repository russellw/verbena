"use strict"

import { readFileSync } from "fs"

let file
let txt

export function parse(file1) {
	file = file1
	txt = readFileSync(file, "utf8") + "\n"
	console.log(txt)
}
