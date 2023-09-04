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

bool htmlComment() {
	if (eq(src, "<!--")) {
		src += 4;
		while (!eq(src, "-->")) {
			if (!*src)
				err("unclosed '<!--'");
			++src;
		}
		src += 3;
		return 1;
	}
	return 0;
}

void html() {
	int depth = 0;
	while (*src) {
		switch (*src) {
		case '<':
			if (htmlComment())
				continue;
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

set<char> chars{0};

struct Page {
	string uname;
	string fname;
	vector<string> params;

	Page(string name) {
		uname = name;
		if (uname == "main")
			uname.clear();
		for (auto c: uname)
			chars.insert(c);

		fname = camelCase(name);
	}

	int ch(int i) {
		assert(i <= uname.size());
		if (i == uname.size())
			return 0;
		return uname[i];
	}
};

void dispatch(const vector<Page*>& pages, int i) {
	// if only one candidate page remains, we know where to go
	if (pages.size() == 1) {
		auto page = pages[0];
		os << '{';

		// parameters
		os << "auto s = req +" << page->uname.size() << ';';
		for (auto param: page->params)
			os << "auto " << param << "= \"\";";

		// call
		os << pages[0]->fname << '(';
		for (auto param: page->params)
			os << param << ',';
		os << "o);";

		// done
		os << "return;";
		os << '}';
		return;
	}

	// how many possibilities are there for the character at this position?
	// if there is only one, this position has no discriminating power
	unordered_set<char> possibilities;
	for (auto page: pages)
		possibilities.insert(page->ch(i));
	if (possibilities.size() == 1) {
		dispatch(pages, i + 1);
		return;
	}

	// check character at this position
	os << "switch (req[" << i << "]) {";
	for (auto c: chars) {
		// which pages match this character?
		vector<Page*> v;
		for (auto page: pages)
			if (page->ch(i) == c)
				v.push_back(page);
		if (v.size()) {
			// recur on the rest of the URL
			if (c)
				os << "case '" << c << "':";
			else
				os << "default:";
			dispatch(v, i + 1);
			os << "break;";
		}
	}
	os << '}';
}

int main(int argc, char** argv) {
	try {
		// pages.cxx
		os.open("pages.cxx");
		os << "#include <main.h>\n";

		// pages
		vector<Page*> pages;
		for (int i = 1; i < argc; ++i) {
			file = argv[i];
			auto page = new Page(path(file).stem().string());
			pages.push_back(page);

			// preprocess
			pread("cl -EP -I../src -nologo " + file);
			src = text.data();

			// parameters
			for (;;) {
				while (isspace(*src))
					++src;
				if (htmlComment())
					continue;
				break;
			}
			while (*src == '?') {
				++src;
				auto src0 = src;
				while (isalnum(*src))
					++src;
				page->params.push_back({src0, src});
				while (isspace(*src))
					++src;
			}

			// page generator function
			os << "void " << page->fname << '(';
			for (auto param: page->params)
				os << "const char*" << param << ',';
			os << "string& o) {";
			html();
			if (*src)
				err("unmatched '}'");
			flush();
			os << '}';
		}

		// dispatch function
		os << "void dispatch(const char* req, string& o) {";
		dispatch(pages, 0);
		os << '}';

		return 0;
	} catch (exception& e) {
		cout << e.what() << '\n';
		return 1;
	}
}
