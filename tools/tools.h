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
#include <array>
#include <exception>
#include <filesystem>
#include <fstream>
#include <iomanip>
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
#define debug(a) cout << __FILE__ << ':' << __LINE__ << ": " << __func__ << ": " << #a << ": " << a << '\n'
#endif

struct Separator {
	bool subsequent = 0;

	bool operator()() {
		auto a = subsequent;
		subsequent = 1;
		return a;
	}
};

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

// SORT
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
		sprintf(buf, "\\x%02x\"\"", (unsigned char)c);
		o += buf;
	}
	return o + '"';
}
