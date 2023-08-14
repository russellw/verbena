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
#include <errno.h>
#include <limits.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// C++ headers
#include <array>
using std::array;

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
#include "../sqlite/sqlite3.h"

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

// schema
enum {
	// SORT
	t_date,
	t_decimal,
	t_integer,
	t_text,
};

struct Table;

struct Field {
	// descending order of size for efficient layout in memory
	const char* name;
	const char* ref;

	// SORT
	uint16_t size;
	// SORT
	uint8_t scale;
	uint8_t type;
	// SORT
	bool key;
	bool nonull;
};

struct Table {
	const char* name;
	Field* fields;
};

// database
void def(const Field* field, string& sql) {
	// name
	sql += field->name;

	// type
	if (field->type == t_integer)
		sql += " INTEGER";
	else
		sql += " TEXT";
	if (field->nonull)
		sql += " NOT NULL";

	// primary key
	if (field->key)
		sql += " PRIMARY KEY";

	// foreign key
	if (field->ref) {
		sql += " REFERENCES ";
		sql += field->ref;
	}
}

char file[] = "C:\\Users\\Public\\Documents\\verbena.db";
sqlite3* db;

void exec(const string& sql) {
	char* msg;
	if (sqlite3_exec(db, sql.data(), 0, 0, &msg) != SQLITE_OK)
		throw runtime_error(msg);
}

sqlite3_stmt* prep(const string& sql) {
	sqlite3_stmt* S;
	if (sqlite3_prepare_v2(db, sql.data(), sql.size() + 1, &S, 0) != SQLITE_OK)
		throw runtime_error(sql + ": " + sqlite3_errmsg(db));
	return S;
}

void bind(sqlite3_stmt* S, int i, const char* val) {
	if (sqlite3_bind_text(S, i, val, -1, SQLITE_STATIC) != SQLITE_OK)
		throw runtime_error(sqlite3_errmsg(db));
}

void finish(sqlite3_stmt* S) {
	switch (sqlite3_step(S)) {
	case SQLITE_DONE:
		sqlite3_finalize(S);
		return;
	case SQLITE_ROW:
		throw runtime_error("finish: sqlite3_step returned data");
	}
	throw runtime_error(sqlite3_errmsg(db));
}

bool step(sqlite3_stmt* S) {
	switch (sqlite3_step(S)) {
	case SQLITE_DONE:
		sqlite3_finalize(S);
		return 0;
	case SQLITE_ROW:
		return 1;
	}
	throw runtime_error(sqlite3_errmsg(db));
}

const char* get(sqlite3_stmt* S, int i) {
	return (const char*)sqlite3_column_text(S, i);
}
