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

#include "../sqlite/sqlite3.h"

#include <algorithm>
#include <array>
#include <exception>
#include <fstream>
#include <iostream>
#include <iterator>
#include <ostream>
#include <stdexcept>
#include <string>
#include <unordered_map>
#include <unordered_set>
#include <vector>
using namespace std;

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
	// SORT
	bool key;
	string name;
	bool nonull;
	const Table* ref;
	int scale;
	int size;
	int type;
};

struct Table {
	string name;
	vector<Field> fields;
};

// database
void def(const Field& field, string& sql) {
	// name
	sql += field.name;

	// type
	if (field.type == t_integer)
		sql += " INTEGER";
	else
		sql += " TEXT";
	if (field.nonull)
		sql += " NOT NULL";

	// primary key
	if (field.key)
		sql += " PRIMARY KEY";

	// foreign key
	if (field.ref) {
		sql += " REFERENCES ";
		sql += field.ref->name;
	}
}

const char file[] = "C:\\Users\\Public\\Documents\\verbena.db";
sqlite3* db;

void exec(string sql) {
	char* msg;
	if (sqlite3_exec(db, sql.data(), 0, 0, &msg) != SQLITE_OK)
		throw runtime_error(msg);
}

sqlite3_stmt* prep(string sql) {
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
