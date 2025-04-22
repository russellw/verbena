import js from "@eslint/js"
import globals from "globals"
import { includeIgnoreFile } from "@eslint/compat"
import { fileURLToPath } from "node:url"

const gitignorePath = fileURLToPath(new URL(".gitignore", import.meta.url))

export default [
	includeIgnoreFile(gitignorePath),
	js.configs.recommended,
	{
		files: ["**/*.js"],
		languageOptions: {
			ecmaVersion: 2023,
			sourceType: "module",
			globals: {
				...globals.node,
			},
		},
		rules: {
			curly: ["error", "all"],
			"no-unused-vars": "off",
			eqeqeq: ["error", "always", { null: "ignore" }],
			"no-case-declarations": "off",
		},
	},
]
