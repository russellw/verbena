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

#include "../all.h"

// POSIX headers
#include <fcntl.h>
#include <sys/stat.h>

#ifdef _WIN32
#include <io.h>
#else
#include <unistd.h>
#define O_BINARY 0
#endif

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

// output
FILE* outf;

FILE* xfopen(const char* mode) {
	auto f = fopen(file.data(), mode);
	if (!f)
		throw runtime_error(file + ": " + strerror(errno));
	return f;
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
		out("\n");
	}
	fclose(outf);
}

// SORT
string esc(const string& s) {
	bool q = 0;
	string o;
	for (auto c: s) {
		if (isprint((unsigned char)c)) {
			if (!q) {
				o += '"';
				q = 1;
			}
			switch (c) {
			case '"':
				o += '\\';
				break;
			case '\n':
				o += "\\n";
				continue;
			}
			o += c;
			continue;
		}
		if (q) {
			o += '"';
			q = 0;
		}
		o += "\"\\x";
		char buf[3];
		sprintf(buf, "%02x", (unsigned char)c);
		o += buf;
		o += '"';
	}
	if (q)
		o += '"';
	return o;
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

// parser
enum {
	k_and = 0x100,
	k_eq,
	k_ge,
	k_le,
	k_quote,
	k_number,
	k_ne,
	k_or,
	k_word,
	end_k
};

// position in source text
char* src;
int line;

// current token
int tok;
string str;

[[noreturn]] void err(string msg) {
	string s;
	if (' ' < tok && tok < 127)
		s = '\'' + string(1, tok) + '\'';
	else if (tok == k_word)
		s = '\'' + str + '\'';
	else
		s = to_string(tok);
	throw runtime_error(file + ':' + to_string(line) + ": " + s + ": " + msg);
}

void lexQuote() {
	auto s = src;
	auto q = *s++;
	str.clear();
	while (*s != q) {
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
		case '!':
			if (s[1] == '=') {
				src = s + 2;
				tok = k_ne;
				return;
			}
			break;
		case '"':
		case '\'':
			tok = k_quote;
			lexQuote();
			return;
		case '#':
			// #line
			errno = 0;
			line = strtol(src + 6, &s, 10) - 1;
			if (errno)
				throw runtime_error(strerror(errno));
			if (s[1] != '"')
				throw runtime_error("bad #line");
			src = s + 1;
			lexQuote();
			file = str;
			continue;
		case '&':
			if (s[1] == '&') {
				src = s + 2;
				tok = k_and;
				return;
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
			do
				++s;
			while (isalnum(*s) || *s == '_');
			if (*s == '.')
				do
					++s;
				while (isalnum(*s) || *s == '_');
			str.assign(src, s);
			src = s;
			tok = k_number;
			return;
		case '<':
			if (s[1] == '=') {
				src = s + 2;
				tok = k_le;
				return;
			}
			break;
		case '=':
			if (s[1] == '=') {
				src = s + 2;
				tok = k_eq;
				return;
			}
			break;
		case '>':
			if (s[1] == '=') {
				src = s + 2;
				tok = k_ge;
				return;
			}
			break;
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
		case '|':
			if (s[1] == '|') {
				src = s + 2;
				tok = k_or;
				return;
			}
			break;
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

string atom() {
	switch (tok) {
	case k_number:
	case k_quote:
	case k_word:
		auto s = str;
		lex();
		return s;
	}
	err("expected atom");
}
