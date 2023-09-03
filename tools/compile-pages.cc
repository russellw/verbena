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

char* src;

[[noreturn]] void err(string msg) {
	print(src);
	throw runtime_error(file + ": " + msg);
}

bool ccomment() {
	switch (src[1]) {
	case '*':
		src += 2;
		while (!eq(src, "*/")) {
			if (!*src)
				err("unclosed block comment");
			++src;
		}
		src += 2;
		return 1;
	case '/':
		src = strchr(src, '\n') + 1;
		return 1;
	}
	return 0;
}

string quote() {
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
	return string(src0, src);
}

string buf;
ofstream os;

void flush() {
	if (buf.size()) {
		os << "o +=" << esc(buf) << ';';
		buf.clear();
	}
}

void html();

void sql() {
	for (;;) {
		switch (*src) {
		case '-':
			if (src[1] == '-') {
				src = strchr(src, '\n') + 1;
				continue;
			}
			break;
		case '\'':
			os << quote();
			continue;
		case '\n':
			++src;
			os << ' ';
			continue;
		case '}':
			return;
		case 0:
			err("unclosed '$'");
		}
		os << *src++;
	}
}

void cxxExpr() {
	int depth = 0;
	for (;;) {
		switch (*src) {
		case '(':
		case '[':
			++depth;
			break;
		case ')':
		case ']':
			--depth;
			if (!depth) {
				// in this case, the closing bracket
				// is actually part of the expression to be copied
				os << *src++;
				return;
			}
			break;
		case 0:
			err("unclosed '@' in HTML");
		}
		os << *src++;
	}
}

void cxxBlock() {
	int depth = 0;
	for (;;) {
		switch (*src) {
		case '"':
		case '\'':
			os << quote();
			continue;
		case '$':
			// $
			++src;
			while (isspace(*src))
				++src;

			// [variable]
			if (isalnum(*src) || *src == '_') {
				os << "Query ";
				while (isalnum(*src) || *src == '_')
					os << *src++;
				while (isspace(*src))
					++src;
			} else
				os << "exec";

			// {
			if (*src != '{')
				err("'$' without '{'");
			++src;
			os << "(\"";

			// SQL
			sql();

			// }
			assert(*src == '}');
			++src;
			os << "\");";
			continue;
		case '/':
			if (ccomment())
				continue;
			break;
		case '@':
			// @{
			if (src[1] != '{')
				err("stray '@' in C++");
			src += 2;
			os << '{';

			// HTML
			html();

			// }
			if (*src != '}')
				err("unclosed '@{' in C++");
			++src;
			flush();
			os << '}';
			continue;
		case '{':
			++depth;
			break;
		case '}':
			--depth;
			if (depth < 0)
				return;
			break;
		case 0:
			err("unclosed '@{' in HTML");
		}
		os << *src++;
	}
}

void html() {
	int depth = 0;
	while (*src) {
		switch (*src) {
		case '<':
			if (eq(src, "<!--")) {
				src += 4;
				while (!eq(src, "-->")) {
					if (!*src)
						err("unclosed '<!--'");
					++src;
				}
				src += 3;
				continue;
			}
			if (eq(src, "<script>")) {
				// JavaScript
				while (!eq(src, "</script>")) {
					switch (*src) {
					case '"':
					case '\'':
						buf += quote();
						continue;
					case '/':
						if (ccomment())
							continue;
						break;
					case 0:
						err("unclosed <script>");
					}
					buf += *src++;
				}
				src += 9;
				buf += "</script>";
				continue;
			}
			break;
		case '@':
			switch (src[1]) {
			case '@':
				++src;
				break;
			case '{':
				// @{
				src += 2;
				flush();
				os << '{';

				// C++
				cxxBlock();

				// }
				++src;
				os << '}';
				continue;
			default:
				// @
				++src;
				flush();
				os << "o +=";

				// C++
				cxxExpr();

				os << ';';
				continue;
			}
			break;
		case '{':
			++depth;
			break;
		case '}':
			--depth;
			if (depth < 0)
				return;
			break;
		}
		buf += *src++;
	}
}

int main(int argc, char** argv) {
	try {
		if (argc < 2 || argv[1][0] == '-') {
			puts("compile-pages *-page.h\n"
				 "Writes pages.cxx");
			return 1;
		}

		// pages.cxx
		os.open("pages.cxx");
		os << "#include <main.h>\n";

		// pages
		vector<string> pages;
		for (int i = 1; i < argc; ++i) {
			file = argv[i];
			auto name = path(file).stem().string();
			pages.push_back(name);

			// preprocess
			pread("cl -EP -I../src -nologo " + file);
			src = text.data();

			// page generator function
			os << "void " << camelCase(name) << "(string& o) {\n";
			html();
			if (*src)
				err("unmatched '}'");
			flush();
			os << "}\n";
		}

		// dispatch function
		os << "void dispatch(const char* req, string& o) {\n";
		for (auto name: pages) {
			auto s = name;
			if (s == "main")
				s.clear();
			os << "if (eq(req, \"" << s << " \")) {\n";
			os << camelCase(name) << "(o);\n";
			os << "return;\n";
			os << "}\n";
		}
		os << "}\n";

		return 0;
	} catch (exception& e) {
		println(e.what());
		return 1;
	}
}
