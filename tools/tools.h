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
