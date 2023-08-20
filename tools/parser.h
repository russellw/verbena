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
int keyword;

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
	keyword = -1;
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
			if (keywords.count(str))
				keyword = keywords.at(str);
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

void expect(char c) {
	if (!eat(c))
		err(string("expected '") + c + '\'');
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
