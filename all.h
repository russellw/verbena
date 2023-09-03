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

// C headers
#include <assert.h>
#include <ctype.h>
#include <errno.h>
#include <limits.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// C++ headers
#include <algorithm>
using std::max;
using std::min;
using std::swap;

#include <array>
using std::array;

#include <exception>
using std::exception;

#include <iostream>
using std::cout;

#include <ostream>
using std::ostream;

#include <stdexcept>
using std::runtime_error;

#include <string>
using std::string;
using std::to_string;

#include <unordered_map>
using std::unordered_map;

#include <unordered_set>
using std::unordered_set;

#include <vector>
using std::vector;

// SORT
*/

/*

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

// debug
#ifdef NDEBUG
#define debug(a)
#else
#define debug(a) cout << __FILE__ << ':' << __LINE__ << ": " << __func__ << ": " << #a << ": " << a << '\n'
#endif

// a lot of output syntax uses comma separators
struct Separator {
	bool subsequent = 0;

	bool operator()() {
		auto a = subsequent;
		subsequent = 1;
		return a;
	}
};

// SORT
inline bool eq(const char* s, const char* t) {
	for (auto i = strlen(t); i--;)
		if (*s++ != *t++)
			return 0;
	return 1;
}
