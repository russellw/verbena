"use strict"

import { parse } from "./parser.js"

const args = process.argv.slice(2)
const file = args[0]
const v = parse(file)
console.log(v)
