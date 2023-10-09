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

#include <libpq-fe.h>

#include <algorithm>
#include <array>
#include <exception>
#include <iostream>
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
struct Table;

struct Field {
	// SORT
	bool key;
	string name;
	bool nonull;
	const Table* ref;
	const char* type;
};

struct Table {
	string name;
	vector<Field> fields;
};

// database
bool isint(const Field& field) {
	return field.type == "bigint" || field.type == "integer" || field.type == "smallint";
}

void def(const Field& field, string& sql) {
	sql += field.name + ' ' + field.type;
	if (field.nonull)
		sql += " NOT NULL";
	if (field.key) {
		sql += " PRIMARY KEY";
		if (isint(field))
			sql += " GENERATED ALWAYS AS IDENTITY";
	}
	if (field.ref) {
		sql += " REFERENCES ";
		sql += field.ref->name;
	}
}

PGconn* con;

void initdb(const char* info) {
	con = PQconnectdb(info);
	if (PQstatus(con) != CONNECTION_OK)
		throw runtime_error(PQerrorMessage(con));
}

PGresult* exec(string sql) {
	auto r = PQexec(con, sql.data());
	switch (PQresultStatus(r)) {
	case PGRES_COMMAND_OK:
	case PGRES_TUPLES_OK:
		break;
	default:
		throw runtime_error(PQresultErrorMessage(r));
	}
	return r;
}

void exec(string sql, const char* val0, const char* val1) {
	const char* vals[]{val0, val1};
	auto r = PQexecParams(con, sql.data(), 2, 0, vals, 0, 0, 0);
	if (PQresultStatus(r) != PGRES_COMMAND_OK)
		throw runtime_error(PQresultErrorMessage(r));
}
