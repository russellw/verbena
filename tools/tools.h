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

#include <fstream>
using std::ifstream;

#include <iterator>
using std::istreambuf_iterator;

// input
string file;
string text;
vector<string> V;

void readText() {
	ifstream is(file, std::ios::in);
	text = {istreambuf_iterator<char>(is), istreambuf_iterator<char>()};

	// make sure input ends with a newline, to simplify parser code
	if (text.empty() || text.back() != '\n')
		text += '\n';
}

void readLines() {
	readText();
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
			auto r = _pclose(f);
			if (r)
				throw runtime_error(cmd + ": " + to_string(r));
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

void out(string s) {
	fwrite(s.data(), 1, s.size(), outf);
}

void writeLines() {
	outf = xfopen("wb");
	for (auto s: V) {
		out(s);
		out("\n");
	}
	fclose(outf);
}

// SORT
bool endsWith(string s, const char* t) {
	auto n = strlen(t);
	if (s.size() < n)
		return 0;
	return memcmp(s.data() + s.size() - n, t, n) == 0;
}

string esc(string s) {
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

	auto s = V[i];

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

bool startsWith(string s, const char* t) {
	auto n = strlen(t);
	if (s.size() < n)
		return 0;
	return memcmp(s.data(), t, n) == 0;
}
