/*
Copyright 2023 Russell Wallace
This file is part of Olivine.

Olivine is free software: you can redistribute it and/or modify it under the
terms of the GNU Affero General Public License as published by the Free Software
Foundation, either version 3 of the License, or (at your option) any later
version.

Olivine is distributed in the hope that it will be useful, but WITHOUT ANY
WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A
PARTICULAR PURPOSE.  See the GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License along
with Olivine.  If not, see <http:www.gnu.org/licenses/>.
*/

#ifdef _MSC_VER
#define unreachable() __assume(0)
#else
#define unreachable() __builtin_unreachable()
#endif

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
int indent(const vector<string>& v, size_t i);
string quote(const string& s);
void readBytes(const string& file, vector<unsigned char>& v);
void readCsv(const string& file, vector<vector<string>>& vs);
string readFile(const string& file);
void readLines(const string& file, vector<string>& v);
void writeFile(const string& file, const string& s);
void writeLines(const string& file, const vector<string>& v);
