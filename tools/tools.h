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
using std::runtime_error;

#include <string>
using std::string;
using std::to_string;

#include <vector>
using std::vector;

// library headers
#include <libpq-fe.h>

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

// SORT
int indent(const vector<string>& v, size_t i) {
	// end of file is end of scope, so semantically a dedent
	if (i == v.size())
		return -1;

	auto& s = v[i];

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

void readBytes(const string& file, vector<unsigned char>& v) {
	auto f = open(file.data(), O_RDONLY | O_BINARY);
	struct stat st;
	if (f < 0 || fstat(f, &st))
		throw runtime_error(file + ": " + strerror(errno));
	auto n = st.st_size;

	v.resize(n);
	read(f, v.data(), n);

	close(f);
}

string readFile(const string& file) {
	auto f = open(file.data(), O_RDONLY | O_BINARY);
	struct stat st;
	if (f < 0 || fstat(f, &st))
		throw runtime_error(file + ": " + strerror(errno));
	auto n = st.st_size;

	string s;
	s.resize(n);
	read(f, s.data(), n);

	close(f);

	// make sure input ends with a newline, to simplify parser code
	if (s.empty() || s.back() != '\n')
		s += '\n';
	return s;
}

void readLines(const string& file, vector<string>& v) {
	auto text = readFile(file);
	auto s = text.data();
	v.clear();
	while (*s) {
		auto u = strchr(s, '\n');
		auto t = u;
		while (s < t && (t[-1] == ' ' || t[-1] == '\t' || t[-1] == '\r'))
			--t;
		v.push_back(string(s, t));
		s = u + 1;
	}
}

void writeFile(const string& file, const string& s) {
	auto f = open(file.data(), O_CREAT | O_WRONLY | O_TRUNC | O_BINARY, 0644);
	if (f < 0)
		throw runtime_error(file + ": " + strerror(errno));
	write(f, s.data(), s.size());
	close(f);
}

void writeLines(const string& file, const vector<string>& v) {
	auto f = fopen(file.data(), "wb");
	if (!f)
		throw runtime_error(file + ": " + strerror(errno));
	for (auto& s: v) {
		fwrite(s.data(), 1, s.size(), f);
		fputc('\n', f);
	}
	fclose(f);
}
