"use strict"

import path from "path"

import { parse } from "./parser.js"
import { compile } from "./compiler.js"

const args = process.argv.slice(2)
const file = args[0]
const v = parse(file)
console.log(v)
const name = path.basename(file, path.extname(file))
compile(`build/${name}.js`, v)
