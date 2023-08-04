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
	Table* ref;

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

sqlite3_stmt* prep(const char* sql, int len);

sqlite3_stmt* prep(const char* sql) {
	return prep(sql, strlen(sql) + 1);
}

sqlite3_stmt* prep(const string& sql);
void bind(sqlite3_stmt* S, int i, const char* val);
void finish(sqlite3_stmt* S);
bool step(sqlite3_stmt* S);
const char* get(sqlite3_stmt* S, int i);

// query with at most one result
struct Result {
	sqlite3_stmt* S;

	Result(sqlite3_stmt* S): S(S) {
		switch (sqlite3_step(S)) {
		case SQLITE_DONE:
			sqlite3_finalize(S);
			S = 0;
			return;
		case SQLITE_ROW:
			return;
		}
		throw runtime_error(sqlite3_errmsg(db));
	}

	~Result() {
		sqlite3_finalize(S);
	}

	operator bool() {
		return S;
	}
};

Result query(const char* sql, int len, const char* val1);

Result query(const char* sql, const char* val1) {
	return query(sql, strlen(sql) + 1, val1);
}

// query with many results
struct Results {
	sqlite3_stmt* S;

	Results(sqlite3_stmt* S): S(S) {
	}

	~Results() {
		sqlite3_finalize(S);
	}
};

Results querys(const char* sql, int len, const char* val1);

Results querys(const char* sql, const char* val1) {
	return querys(sql, strlen(sql) + 1, val1);
}

// update
struct Transaction {
	Transaction();
	~Transaction();

	void insert(const Table& table, int field1, const char* val1, int field2, const char* val2);
};
