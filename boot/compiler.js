"use strict"

import { writeFileSync } from "fs"

let txt = ""

function emit(s) {
	txt += s
}

// Top level
export function compile(file, v) {
	writeFileSync(file, txt, "utf8")
}
