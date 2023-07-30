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
#include <ctype.h>
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
using std::stoi;
using std::string;
using std::to_string;

#include <unordered_map>
using std::unordered_map;

#include <unordered_set>
using std::unordered_set;

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

void pread(string cmd) {
	auto f = _popen(cmd.data(), "r");
	if (!f)
		throw runtime_error(cmd + ": " + strerror(errno));
	text.clear();
	for (;;) {
		auto c = fgetc(f);
		if (c < 0) {
			_pclose(f);
			return;
		}
		text += c;
	}
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
		if (isprint((unsigned char)c)) {
			if (!q) {
				r += '"';
				q = 1;
			}
			switch (c) {
			case '"':
				r += '\\';
				break;
			case '\n':
				r += "\\n";
				continue;
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

int indent(int i) {
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
	k_quote = 0x100,
	k_word,
};

// position in source text
char* src;
int line;

// current token
int tok;
string str;

void err(const string& msg) {
	throw runtime_error(file + ':' + to_string(line) + ": " + msg);
}

void lexQuote() {
	auto s = src;
	assert(*s == '"');
	++s;
	str.clear();
	while (*s != '"') {
		str += *s;
		switch (*s) {
		case '\\':
			s += 2;
			continue;
		case '\n':
		case 0:
			err("unclosed quote");
		}
		++s;
	}
	src = s + 1;
}

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
			tok = k_quote;
			lexQuote();
			return;
		case '#':
			// #line
			errno = 0;
			line = strtol(src + 6, &s, 10);
			if (errno)
				throw runtime_error(strerror(errno));
			if (s[1] != '"')
				throw runtime_error("bad #line");
			src = s + 1;
			lexQuote();
			file = str;
			continue;
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
		case '_':
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
			while (isalnum(*s) || *s == '_');
			str.assign(src, s);
			src = s;
			tok = k_word;
			return;
		case '\n':
			src = s + 1;
			++line;
			continue;
		case 0:
			tok = 0;
			return;
		}
		src = s + 1;
		tok = *s;
		return;
	}
}

void preprocess() {
	pread("cl -E -nologo " + file);
	src = text.data();
	line = 1;
	lex();
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
	// SORT
	bool key = 0;
	string name;
	bool nonull = 0;
	Table* ref = 0;
	string refName;
	int scale = 2;
	int size = 0;
	string type = "text";
	//

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

template <class T> void topologicalSortRecur(const vector<T>& v, vector<T>& r, unordered_set<T>& visited, T a) {
	if (!visited.insert(a).second)
		return;
	for (auto b: a->links)
		topologicalSortRecur(v, r, visited, b);
	r.push_back(a);
}

template <class T> void topologicalSort(vector<T>& v) {
	unordered_set<T> visited;
	vector<T> r;
	for (auto a: v)
		topologicalSortRecur(v, r, visited, a);
	v = r;
}

void readSchema() {
	// parse
	preprocess();
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
					if (tok == k_word)
						field->refName = word();
					else
						field->refName = field->name;
					expect(';');
					continue;
				}

				if (eat("scale")) {
					field->scale = stoi(word());
					expect(';');
					continue;
				}

				if (eat("size")) {
					field->size = stoi(word());
					expect(';');
					continue;
				}

				if (eat("type")) {
					field->type = word();
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

	// eliminate forward references
	topologicalSort(tables);
}
