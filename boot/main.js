"use strict"

import path from "path"
import util from "util"

import { parse } from "./parser.js"
import { compile } from "./compiler.js"

// Command line
const file = process.argv[2]

// Parse
const v = parse(file)
console.log(
	util.inspect(v, {
		showHidden: false,
		depth: null,
		colors: true,
		maxArrayLength: null,
		maxStringLength: null,
		compact: true,
	}),
)

// Compile
const name = path.basename(file, path.extname(file))
compile(`build/${name}.js`, v)
