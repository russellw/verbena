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

// C headers
#include <assert.h>
#include <errno.h>
#include <limits.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// POSIX headers
#include <fcntl.h>
#include <sys/stat.h>

#ifdef _WIN32
#include <io.h>
#else
#include <unistd.h>
#define O_BINARY 0
#endif

// C++ headers
#include <exception>
using std::exception;

#include <stdexcept>
using std::out_of_range;
using std::runtime_error;

#include <string>
using std::string;
using std::to_string;

#include <unordered_map>
using std::unordered_map;

#include <vector>
using std::vector;

// debug
#ifdef NDEBUG
#define debug(a)
#else
#define debug(a) \
	do { \
		printf("%s:%d: %s: %s: ", __FILE__, __LINE__, __func__, #a); \
		println(a); \
	} while (0)
#endif

// defining our own isxxxxx in preference to the ones in ctype
// avoids surprising behavior related to locale
// and undefined behavior if an input has the high bit set
// and you forget to cast to unsigned char
inline bool isdigit1(int c) {
	return '0' <= c && c <= '9';
}

inline bool islower1(int c) {
	return 'a' <= c && c <= 'z';
}

inline bool isupper1(int c) {
	return 'A' <= c && c <= 'Z';
}

inline bool isalpha1(int c) {
	return islower1(c) || isupper1(c);
}

inline bool isalnum1(int c) {
	return isalpha1(c) || isdigit1(c);
}

inline bool isid(int c) {
	return isalnum1(c) || c == '_';
}

inline bool isprint1(int c) {
	return ' ' <= c && c <= 126;
}

inline int tolower1(int c) {
	return isupper1(c) ? c + 32 : c;
}

inline int toupper1(int c) {
	return islower1(c) ? c - 32 : c;
}

// SORT
inline void print(char c) {
	putchar(c);
}

inline void print(const char* s) {
	fwrite(s, 1, strlen(s), stdout);
}

inline void print(const string& s) {
	fwrite(s.data(), 1, s.size(), stdout);
}

inline void print(const void* p) {
	printf("%p", p);
}

inline void print(int32_t n) {
	print(to_string(n));
}

inline void print(int64_t n) {
	print(to_string(n));
}

inline void print(uint32_t n) {
	print(to_string(n));
}

inline void print(uint64_t n) {
	print(to_string(n));
}

template <class T> void print(const vector<T>& v) {
	putchar('[');
	bool more = 0;
	for (auto& a: v) {
		if (more)
			print(", ");
		more = 1;
		print(a);
	}
	putchar(']');
}

inline void println(const char* s) {
	auto n = strlen(s);
	fwrite(s, 1, n, stdout);
	if (n && s[n - 1] != '\n')
		putchar('\n');
}

template <class T> void println(const T& a) {
	print(a);
	putchar('\n');
}

// a lot of output syntax uses comma separators
struct Separator {
	bool subsequent = 0;

	bool operator()() {
		auto r = subsequent;
		subsequent = 1;
		return r;
	}
};

// input
string file;
string text;
vector<string> V;

void readFile() {
	auto f = open(file.data(), O_RDONLY | O_BINARY);
	struct stat st;
	if (f < 0 || fstat(f, &st))
		throw runtime_error(file + ": " + strerror(errno));
	auto n = st.st_size;

	text.resize(n);
	read(f, text.data(), n);

	close(f);

	// make sure input ends with a newline, to simplify parser code
	if (text.empty() || text.back() != '\n')
		text += '\n';
}

void readLines() {
	readFile();
	auto s = text.data();
	V.clear();
	while (*s) {
		auto u = strchr(s, '\n');
		auto t = u;
		while (s < t && (t[-1] == ' ' || t[-1] == '\t' || t[-1] == '\r'))
			--t;
		V.push_back(string(s, t));
		s = u + 1;
	}
}

// output
FILE* outf;

FILE* xfopen(const char* mode) {
	auto f = fopen(file.data(), mode);
	if (!f)
		throw runtime_error(file + ": " + strerror(errno));
	return f;
}

void out(char c) {
	fputc(c, outf);
}

void out(const char* s) {
	fwrite(s, 1, strlen(s), outf);
}

void out(const string& s) {
	fwrite(s.data(), 1, s.size(), outf);
}

void writeLines() {
	outf = xfopen("wb");
	for (auto& s: V) {
		out(s);
		out('\n');
	}
	fclose(outf);
}

// SORT
string esc(const string& s) {
	bool q = 0;
	string r;
	for (auto c: s) {
		if (isprint1(c)) {
			if (!q) {
				r += '"';
				q = 1;
			}
			switch (c) {
			case '\n':
				r += "\\n";
				continue;
			case '"':
				r += '\\';
				break;
			}
			r += c;
			continue;
		}
		if (q) {
			r += '"';
			q = 0;
		}
		r += "\"\\x";
		char buf[3];
		sprintf(buf, "%02x", (unsigned char)c);
		r += buf;
		r += '"';
	}
	if (q)
		r += '"';
	return r;
}

int indent(size_t i) {
	// end of file is end of scope, so semantically a dedent
	if (i == V.size())
		return -1;

	auto& s = V[i];

	// blank line does not meaningfully have an indent level
	if (s.empty())
		return INT_MAX;

	// in C++, nor does a preprocessor directive
	if (s[0] == '#')
		return INT_MAX;

	// assuming each file uses either tabs or spaces consistently
	int j = 0;
	while (s[j] == '\t' || s[j] == ' ')
		++j;
	return j;
}

string quote(const string& s) {
	return '"' + s + '"';
}

// parser
enum {
	k_word = 0x100,
};

char* tokBegin;
char* src;

int tok;
string str;

void err(const string& msg) {
	size_t line = 1;
	for (auto s = text.data(); s < tokBegin; ++s)
		if (*s == '\n')
			++line;
	throw runtime_error(file + ':' + to_string(line) + ": " + msg);
}

void lex() {
	for (;;) {
		auto s = tokBegin = src;
		switch (*s) {
		case ' ':
		case '\f':
		case '\n':
		case '\r':
		case '\t':
			src = s + 1;
			continue;
		case '/':
			if (s[1] == '/') {
				src = strchr(s, '\n');
				continue;
			}
			if (s[1] == '*') {
				++s;
				do {
					++s;
					if (!*s)
						err("unclosed block comment");
				} while (!(s[0] == '*' && s[1] == '/'));
				src = s + 2;
				continue;
			}
			break;
		case '0':
		case '1':
		case '2':
		case '3':
		case '4':
		case '5':
		case '6':
		case '7':
		case '8':
		case '9':
		case 'A':
		case 'B':
		case 'C':
		case 'D':
		case 'E':
		case 'F':
		case 'G':
		case 'H':
		case 'I':
		case 'J':
		case 'K':
		case 'L':
		case 'M':
		case 'N':
		case 'O':
		case 'P':
		case 'Q':
		case 'R':
		case 'S':
		case 'T':
		case 'U':
		case 'V':
		case 'W':
		case 'X':
		case 'Y':
		case 'Z':
		case 'a':
		case 'b':
		case 'c':
		case 'd':
		case 'e':
		case 'f':
		case 'g':
		case 'h':
		case 'i':
		case 'j':
		case 'k':
		case 'l':
		case 'm':
		case 'n':
		case 'o':
		case 'p':
		case 'q':
		case 'r':
		case 's':
		case 't':
		case 'u':
		case 'v':
		case 'w':
		case 'x':
		case 'y':
		case 'z':
			do
				++s;
			while (isid(*s));
			str.assign(src, s);
			src = s;
			tok = k_word;
			return;
		case 0:
			tok = 0;
			return;
		}
		src = s + 1;
		tok = *s;
		return;
	}
}

bool eat(int k) {
	if (tok == k) {
		lex();
		return 1;
	}
	return 0;
}

bool eat(const char* s) {
	if (tok == k_word && str == s) {
		lex();
		return 1;
	}
	return 0;
}

void expect(char c) {
	if (!eat(c))
		err(string("expected '") + c + '\'');
}

void expect(const char* s) {
	if (!eat(s))
		err(string("expected '") + s + '\'');
}

string word() {
	if (tok != k_word)
		err("expected word");
	auto s = str;
	lex();
	return s;
}

// schema
struct Table;

struct Field {
	string name;
	string type = "text";
	string size = "0";
	bool nonull = 0;
	bool key = 0;
	string refName;
	Table* ref = 0;

	Field(const string& name): name(name) {
	}
};

struct Table {
	string name;
	vector<Field*> fields;
	vector<Table*> links;

	Table(const string& name): name(name) {
	}
};

vector<Table*> tables;

void readSchema() {
	// read
	readFile();

	// parse
	src = text.data();
	lex();
	while (tok) {
		expect("table");
		auto table = new Table(word());
		expect('{');
		do {
			expect("field");
			auto field = new Field(word());
			expect('{');
			while (!eat('}')) {
				// SORT
				if (eat("key")) {
					field->key = 1;
					expect(';');
					continue;
				}

				if (eat("nonull")) {
					field->nonull = 1;
					expect(';');
					continue;
				}

				if (eat("ref")) {
					field->refName = eat('=') ? word() : field->name;
					expect(';');
					continue;
				}

				if (eat("type")) {
					expect('=');
					field->type = word();
					if (eat('(')) {
						field->size = word();
						expect(')');
					}
					expect(';');
					continue;
				}

				err("expected attribute");
			}
			table->fields.push_back(field);
		} while (!eat('}'));
		tables.push_back(table);
	}

	// link table references
	unordered_map<string, Table*> tableMap;
	for (auto table: tables)
		tableMap[table->name] = table;
	for (auto table: tables)
		for (auto field: table->fields)
			if (field->refName.size()) {
				field->ref = tableMap.at(field->refName);
				auto key = field->ref->fields[0];
				field->type = key->type;
				field->size = key->size;
				table->links.push_back(field->ref);
			}
}
