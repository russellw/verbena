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

char* src;
int line;
string tok;

[[noreturn]] void err(string msg) {
	throw runtime_error(file + ':' + to_string(line) + ": " + msg);
}

void quote() {
	auto src0 = src;
	auto q = *src++;
	while (*src != q) {
		switch (*src) {
		case '\\':
			src += 2;
			continue;
		case '\n':
			err("unclosed quote");
		}
		++src;
	}
	++src;
	tok.assign(src0, src);
}

void lineDirective() {
	assert(eq(src, "#line "));
	errno = 0;
	line = strtol(src + 6, &src, 10) - 1;
	if (errno)
		throw runtime_error(strerror(errno));
	if (!eq(src, " \""))
		throw runtime_error("bad #line");
	++src;
	quote();
	file = tok.substr(1, tok.size() - 2);
}

string buf;

namespace cxx {
void out(string s) {
	if (buf.size()) {
		::out("o +=" + esc(buf) + ";\n");
		buf.clear();
	}
	::out(s);
}
} // namespace cxx

namespace js {
void out(string s) {
	buf += s;
}

void lex() {
	for (;;) {
		auto src0 = src;
		switch (*src) {
		case ' ':
		case '\f':
		case '\r':
		case '\t':
			++src;
			continue;
		case '"':
		case '\'':
			quote();
			return;
		case '#':
			if (eq(src + 1, "line ")) {
				lineDirective();
				continue;
			}
			break;
		case '/':
			switch (src[1]) {
			case '/':
				src = strchr(src, '\n');
				continue;
			case '*':
				++src;
				do {
					++src;
					if (!*src)
						err("unclosed block comment");
					if (*src == '\n')
						++line;
				} while (!eq(src, "*/"));
				src += 2;
				continue;
			}
			break;
		case '\n':
			++src;
			++line;
			continue;
		case 0:
			tok.clear();
			return;
		}
		tok = *src++;
		return;
	}
}

void parse() {
	lex();
	while (!(tok == "<" && eq(src, "/script>"))) {
		if (tok.empty())
			err("unclosed <script>");
		out(tok);
		lex();
	}
	src += 8;
	out("</script>");
}
} // namespace js

namespace html {
void out(string s) {
	buf += s;
}

void word(string s) {
	if (buf.size() && buf.back() != '>')
		buf += ' ';
	buf += s;
}

void parse() {
	for (;;) {
		auto src0 = src;
		switch (*src) {
		case ' ':
		case '\f':
		case '\r':
		case '\t':
			++src;
			continue;
		case '#':
			if (eq(src + 1, "line ")) {
				lineDirective();
				continue;
			}
			break;
		case '<': {
			if (eq(src, "<!--")) {
				src += 3;
				do {
					++src;
					if (!*src)
						err("unclosed comment");
					if (*src == '\n')
						++line;
				} while (!eq(src, "-->"));
				src += 3;
				continue;
			}
			do {
				++src;
				if (*src == '\n')
					err("unclosed '<'");
			} while (*src != '>');
			++src;
			string tag(src0, src);
			out(tag);
			if (tag == "<script>")
				js::parse();
			continue;
		}
		case '\n':
			++src;
			++line;
			continue;
		case 0:
			return;
		}
		do
			++src;
		while (!(isspace(*src) || *src == '<'));
		word(string(src0, src));
	}
}
} // namespace html

string camelCase(string s) {
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
			html::parse();

			// flush output buffer
			// and close generator function
			cxx::out("}\n");
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
