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
	a_link,
	a_row,
};

unordered_map<string, int> tags;

namespace {
struct Init {
#define _(a) tags.emplace(#a, a_##a)
	Init() {
		// SORT
		_(column);
		_(field);
		_(link);
		_(row);
	}
#undef _
} init;
} // namespace

struct Element {
	int tag;
	string name;

	// SORT
	string ref;
	//

	vector<Element*> v;

	Element(int tag): tag(tag) {
	}
};

Element* element() {
	auto s = word();
	Element* a;
	try {
		a = new Element(tags.at(s));
	} catch (out_of_range& e) {
		err(s + ": unknown tag");
	}
	if (tok == k_word)
		a->name = word();
	expect('{');
	while (!eat('}')) {
		// SORT
		if (eat("ref")) {
			eat('=');
			a->ref = word();
			expect(';');
			continue;
		}

		a->v.push_back(element());
	}
	return a;
}

int main(int argc, char** argv) {
	try {
		if (argc < 3 || argv[1][0] == '-') {
			puts("compile-pages schema.h *-page.h\n"
				 "Writes pages.cxx");
			return 1;
		}

		file = argv[1];
		readSchema();

		for (int i = 2; i < argc; ++i) {
			// read
			file = argv[i];
			text = readFile(file);

			// parse
			src = text.data();
			lex();
			while (tok)
				element();
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
