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
with Verbena.  If not, see <https://www.gnu.org/licenses/>.
*/

#include "tools.h"

#include <filesystem>
using std::filesystem::path;

string outs;

void outcc(string s) {
}

void put(string s) {
}

void outWord(string s) {
}

char* src;
int line;
string tok;

[[noreturn]] void err(string msg) {
	throw runtime_error(file + ':' + to_string(line) + ": " + msg);
}

void quote() {
	auto s = src;
	auto q = *s++;
	while (*s != q) {
		switch (*s) {
		case '\\':
			s += 2;
			continue;
		case '\n':
			err("unclosed quote");
		}
		++s;
	}
	++s;
	tok.assign(src, s);
	src = s;
}

void lineDirective() {
	assert(eq(src, "#line "));
	char* s;
	errno = 0;
	line = strtol(src + 6, &s, 10) - 1;
	if (errno)
		throw runtime_error(strerror(errno));
	if (!eq(s, " \""))
		throw runtime_error("bad #line");
	src = s + 1;
	quote();
	file = tok.substr(1, tok.size() - 2);
}

namespace js {
void lex() {
	for (;;) {
		auto s = src;
		switch (*s) {
		case ' ':
		case '\f':
		case '\r':
		case '\t':
			src = s + 1;
			continue;
		case '"':
		case '\'':
			quote();
			return;
		case '#':
			if (eq(s + 1, "line ")) {
				lineDirective();
				continue;
			}
			break;
		case '/':
			switch (s[1]) {
			case '/':
				src = strchr(s, '\n');
				continue;
			case '*':
				++s;
				do {
					++s;
					if (!*s)
						err("unclosed block comment");
					if (*s == '\n')
						++line;
				} while (!eq(s, "*/"));
				src = s + 2;
				continue;
			}
			break;
		case '\n':
			src = s + 1;
			++line;
			continue;
		case 0:
			tok.clear();
			return;
		}
		src = s + 1;
		tok = *s;
		return;
	}
}

void parse() {
	lex();
	while (!(tok == "<" && eq(src, "/script>"))) {
		if (tok.empty())
			err("unclosed <script>");
		put(tok);
		lex();
	}
	src += 8;
	put("</script>");
}
} // namespace js

void html() {
	for (;;) {
		auto s = src;
		switch (*s) {
		case ' ':
		case '\f':
		case '\r':
		case '\t':
			src = s + 1;
			continue;
		case '#':
			if (eq(s + 1, "line ")) {
				lineDirective();
				continue;
			}
			break;
		case '<': {
			if (eq(s, "<!--")) {
				s += 3;
				do {
					++s;
					if (!*s)
						err("unclosed comment");
					if (*s == '\n')
						++line;
				} while (!eq(s, "-->"));
				src = s + 3;
				continue;
			}
			do {
				++s;
				if (*s == '\n')
					err("unclosed '<'");
			} while (*s != '>');
			++s;
			string tag(src, s);
			put(tag);
			src = s;
			if (tag == "<script>")
				js::parse() continue;
		}
		case '\n':
			src = s + 1;
			++line;
			continue;
		case 0:
			return;
		}
		do
			++s;
		while (!(isspace(*s) || *s == '<'));
		outWord(string(src, s));
		src = s;
	}
}

string camelCase(const string& s) {
	string o;
	for (int i = 0; i < s.size();) {
		if (s[i] == '-') {
			o += toupper(s[i + 1]);
			i += 2;
			continue;
		}
		o += s[i++];
	}
	return o;
}

int main(int argc, char** argv) {
	try {
		if (argc < 2 || argv[1][0] == '-') {
			puts("compile-pages *-page.h\n"
				 "Writes pages.cxx");
			return 1;
		}

		// pages.cxx
		file = "pages.cxx";
		outf = xfopen("wb");
		out("// AUTO GENERATED - DO NOT EDIT\n");
		out("#include <main.h>\n");

		// pages
		vector<string> pages;
		for (int i = 1; i < argc; ++i) {
			file = argv[i];
			auto name = path(file).stem().string();
			pages.push_back(name);

			// preprocess
			pread("cl -E -I../src -nologo " + file);

			// page generator function
			out("void " + camelCase(name) + "(string& o) {\n");

			// parse
			src = text.data();
			line = 1;
			html();

			out("}\n");
		}

		// dispatch function
		out("void dispatch(const char* req, string& o) {\n");
		for (auto& name: pages) {
			auto s = name;
			if (s == "main")
				s.clear();
			out("if (eq(req, \"" + s + " \")) {\n");
			out(camelCase(name) + "(o);\n");
			out("return;\n");
			out("}\n");
		}
		out("}\n");

		fclose(outf);
		return 0;
	} catch (exception& e) {
		println(e.what());
		return 1;
	}
}
