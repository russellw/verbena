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

#include <filesystem>
using std::filesystem::path;

// tags
enum {
#define _(a) a_##a,
#include "tags.h"
#undef _
};

unordered_map<string, int> tags;

namespace {
struct Init {
	Init() {
#define _(a) tags.emplace(#a, a_##a);
#include "tags.h"
#undef _
	}
} init;
} // namespace

// abstract syntax tree
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

// SORT
string camelCase(const string& s) {
	string r;
	for (size_t i = 0; i < s.size();) {
		if (s[i] == '-') {
			r += toupper1(s[i + 1]);
			i += 2;
			continue;
		}
		r += s[i++];
	}
	return r;
}

bool endsWith(const string& s, const char* t) {
	auto n = strlen(t);
	if (s.size() < n)
		return 0;
	for (auto i = s.size() - n; i < s.size(); ++i)
		if (s[i] != t[i])
			return 0;
	return 1;
}

string titleCase(const string& s) {
	string r;
	for (auto c: s) {
		if (c == '-')
			c = ' ';
		r += c;
	}
	r[0] = toupper1(r[0]);
	return r;
}

//

// output
FILE* outf;

void out(const char* s) {
	fwrite(s, 1, strlen(s), outf);
}

void out(const string& s) {
	fwrite(s.data(), 1, s.size(), outf);
}

// as an optimization, when we output multiple consecutive string literals, fuse them together
vector<string> literals;

void literal(string s) {
	literals.push_back(s);
}

void code(string t) {
	if (literals.size()) {
		out("o +=");
		for (auto& s: literals) {
			out("\n");
			out(esc(s));
		}
		literals.clear();
		out(";\n");
	}
	out(t);
}

// recur on the abstract syntax tree
void compose(Element* a) {
	switch (a->tag) {
	case a_link:
		literal("<a href=\"");
		literal(a->ref);
		literal("\">");
		literal(titleCase(a->ref));
		literal("</a>");
		return;
	}
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

		// pages.cxx
		outf = xfopen("pages.cxx", "wb");
		out("// AUTO GENERATED - DO NOT EDIT\n");
		out("#include <main.h>\n");

		// pages
		for (int i = 2; i < argc; ++i) {
			// read
			file = argv[i];
			text = readFile(file);

			// parse
			src = text.data();
			lex();
			vector<Element*> v;
			while (tok)
				v.push_back(element());

			// page generator function
			auto stem = path(argv[i]).stem().string();
			auto name = camelCase(stem);
			out("void " + name + "(string& o) {\n");

			// header
			assert(literals.empty());
			literal("<html>");
			literal("<head>");
			literal("<title>");
			auto title = stem;
			if (endsWith(title, "-page"))
				title = title.substr(0, title.size() - 5);
			literal(titleCase(title));
			literal("</title>");
			literal("</head>");

			// body
			literal("<body>");
			for (auto a: v)
				compose(a);

			// flush remaining literals before closing function
			code("}\n");
		}

		// dispatch
		out("void dispatch(string& o) {\n");
		out("mainPage(o);\n");
		out("}\n");
		fclose(outf);
		return 0;
	} catch (exception& e) {
		println(e.what());
		return 1;
	}
}
