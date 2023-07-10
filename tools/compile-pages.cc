/*
Copyright 2023 Russell Wallace
This file is part of Verbena.

Verbena is free software: you can redistribute it and/or modify it under the
terms of the GNU Affero General Public License as published by the Free Software
Foundation, either version 3 of the License, or (at your option) any later
version.

Verbena is distributed in the hope that it will be useful, but WITHOUT ANY
WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A
PARTICULAR PURPOSE.  See the GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License along
with Verbena.  If not, see <http:www.gnu.org/licenses/>.
*/

#include "compile.h"

enum {
	// SORT
	a_column,
	a_field,
	a_row,
};

struct Element {
	int tag;
	vector<Element*> v;

	Element(int tag): tag(tag) {
	}
};

int main(int argc, char** argv) {
	try {
		if (argc < 3 || argv[1][0] == '-') {
			puts("compile-pages schema.h *-page.h\n"
				 "Writes pages.cxx");
			return 1;
		}

		file = argv[1];
		readSchema();

		unordered_map<string, int> tags;
		// SORT
		tags.emplace("column", a_column);
		tags.emplace("link", a_link);
		tags.emplace("row", a_row);
		//

		for (int i = 2; i < argc; ++i) {
			// read
			file = argv[1];
			text = readFile(file);

			// parse
			src = text.data();
			lex();
			while (tok) {
				lex();
			}
		}

		string o = "// AUTO GENERATED - DO NOT EDIT\n";
		o += "#include <main.h>\n";
		o += "void dispatch(string& o){\n";
		o += "o+=\"Hello, World!\";\n";
		o += "}\n";
		writeFile("pages.cxx", o);
		return 0;
	} catch (exception& e) {
		println(e.what());
		return 1;
	}
}
