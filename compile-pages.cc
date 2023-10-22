#include <assert.h>
#include <ctype.h>
#include <errno.h>
#include <limits.h>
#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <algorithm>
#include <exception>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <iterator>
#include <ostream>
#include <set>
#include <stdexcept>
#include <string>
#include <unordered_map>
#include <unordered_set>
#include <vector>
using namespace std;
using filesystem::path;

#ifdef NDEBUG
#define debug(a)
#else
#define debug(a) cout << __FILE__ << ':' << __LINE__ << ": " << __func__ << ": " << #a << ": " << (a) << '\n'
#endif

// input
string file;
string text;

// SORT
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

bool eq(const char* s, const char* t) {
	for (auto i = strlen(t); i--;)
		if (*s++ != *t++)
			return 0;
	return 1;
}

string esc(string s) {
	string o = "\"";
	for (auto c: s) {
		if (isprint((unsigned char)c)) {
			if (c == '"')
				o += '\\';
			o += c;
			continue;
		}
		char buf[7];
		sprintf_s(buf, sizeof buf, "\\x%02x\"\"", (unsigned char)c);
		o += buf;
	}
	return o + '"';
}

void pread(string cmd) {
	auto f = _popen(cmd.data(), "r");
	if (!f)
		throw runtime_error(cmd + ": " + strerror(errno));
	text.clear();
	for (;;) {
		auto c = fgetc(f);
		if (c < 0) {
			auto r = _pclose(f);
			if (r)
				throw runtime_error(cmd + ": " + to_string(r));
			return;
		}
		text += c;
	}
}

// parser
char* src;

[[noreturn]] void err(string msg) {
	throw runtime_error(file + ": " + msg);
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

void sql() {
	int depth = 0;
	while (*src) {
		switch (*src) {
		case '(':
			++depth;
			break;
		case ')':
			--depth;
			if (depth < 0)
				return;
			break;
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
		}
		os << *src++;
	}
}

void cxxExpr() {
	while (isalnum(*src))
		os << *src++;
	switch (*src) {
	case '(':
	case '[':
		break;
	default:
		return;
	}

	int depth = 0;
	while (*src) {
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
		}
		os << *src++;
	}
}

void cxx();

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
			++src;
			switch (*src) {
			case '@':
				break;
			case '{':
				++src;
				flush();
				os << '{';

				cxx();
				if (*src != '}')
					err("unclosed '@{' in HTML");

				++src;
				os << '}';
				continue;
			default:
				flush();
				os << "o +=";

				cxxExpr();
				if (!*src)
					err("unclosed '@' in HTML");

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

void cxx() {
	int depth = 0;
	while (*src) {
		switch (*src) {
		case '"':
		case '\'':
			os << quote();
			continue;
		case '@':
			++src;
			switch (*src) {
			case '(':
				++src;
				os << '"';

				sql();
				if (*src != ')')
					err("unclosed '@(' in C++");

				++src;
				os << '"';
				continue;
			case '{':
				++src;
				os << '{';

				html();
				if (*src != '}')
					err("unclosed '@{' in C++");

				++src;
				flush();
				os << '}';
				continue;
			}
			if (eq(src, "POST") || eq(src, "PUT"))
				return;
			err("stray '@' in C++");
		case '{':
			++depth;
			break;
		case '}':
			--depth;
			if (depth < 0)
				return;
			break;
		}
		os << *src++;
	}
}

struct Page {
	string uname;
	string fname;
	vector<string> params;
	bool post = 0;
	bool put = 0;

	Page(string name) {
		if (name == "main") {
			fname = "main1";
			return;
		}
		uname = name;
		fname = camelCase(name);
	}

	int ch(int i) {
		if (i < uname.size())
			return uname[i];
		return 256;
	}
};

// generate top-level dispatch functions
bool postMode;

void dispatch(const vector<Page*>& pages, int i) {
	// if only one candidate page remains, we know where to go
	if (pages.size() == 1) {
		auto page = pages[0];
		os << page->fname;
		if (postMode)
			os << "POST(s";
		else {
			os << '(';
			if (page->params.size())
				os << "s +" << page->uname.size() << ',';
			os << 'o';
		}
		os << ");";
		os << "return;";
		return;
	}

	// how many possibilities are there for the character at this position?
	// if there is only one, this position has no discriminating power
	set<int> possibilities;
	for (auto page: pages)
		possibilities.insert(page->ch(i));
	if (possibilities.size() == 1) {
		dispatch(pages, i + 1);
		return;
	}

	// check character at this position
	os << "switch (s[" << i << "]) {";
	for (auto c: possibilities) {
		// which pages match this character?
		vector<Page*> v;
		for (auto page: pages)
			if (page->ch(i) == c)
				v.push_back(page);
		if (v.size()) {
			// recur on the rest of the URL
			if (c == *possibilities.rbegin())
				os << "default:";
			else
				os << "case '" << (char)c << "':";
			dispatch(v, i + 1);
		}
	}
	os << '}';
}

int main(int argc, char** argv) {
	try {
		// pages.cxx
		os.open("pages.cxx");
		os << "#include <all.h>\n";

		// pages
		vector<Page*> pages;
		for (int i = 1; i < argc; ++i) {
			file = argv[i];
			auto page = new Page(path(file).stem().string());
			pages.push_back(page);

			// preprocess
			pread("cl -EP -nologo " + file);
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

			// parameters
			switch (page->params.size()) {
			case 0:
				os << "string& o) {";
				break;
			case 1: {
				os << "char* s, string& o) {";
				auto param = page->params[0];
				os << "auto " << param << "= \"\";";
				os << "if (*s == '?') {";
				os << "s +=" << param.size() + 2 << ';';
				os << param << "= s;";
				os << "while (*s > ' ')";
				os << "++s;";
				os << "*s = 0;";
				os << '}';
				break;
			}
			default:
				err("multiple parameters not yet implemented");
			}

			// body
			cxx();
			flush();
			os << '}';

			// POST/PUT function
			if (!*src)
				continue;
			if (eq(src, "POST"))
				page->post = 1;
			else
				page->put = 1;

			os << "void " << page->fname;
			while (isupper(*src))
				os << *src++;
			os << "(char* s) {";
			os << src;
			os << '}';
		}

		// dispatch GET
		os << "void dispatch(char* s, string& o) {";
		dispatch(pages, 0);
		os << '}';

		// dispatch POST
		vector<Page*> v;
		copy_if(pages.begin(), pages.end(), back_inserter(v), [](Page* page) { return page->post; });
		postMode = 1;
		os << "void dispatchPOST(char* s) {";
		dispatch(v, 0);
		os << '}';

		// dispatch PUT
		v.clear();
		copy_if(pages.begin(), pages.end(), back_inserter(v), [](Page* page) { return page->put; });
		os << "void dispatchPUT(char* s) {";
		dispatch(v, 0);
		os << '}';

		return 0;
	} catch (exception& e) {
		cerr << e.what() << '\n';
		return 1;
	}
}
