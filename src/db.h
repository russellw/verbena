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

extern sqlite3* db;

sqlite3_stmt* prep(const char* sql, int len);

inline sqlite3_stmt* prep(const char* sql) {
	return prep(sql, strlen(sql) + 1);
}

sqlite3_stmt* prep(const string& sql);
void bind(sqlite3_stmt* S, int i, const char* val);
void finish(sqlite3_stmt* S);
bool step(sqlite3_stmt* S);
const char* get(sqlite3_stmt* S, int i);

class select {
	sqlite3_stmt* S;

public:
	select(const char* sql) {
		S = prep(sql, strlen(sql));
	}

	select(const char* sql, const char* val1) {
		S = prep(sql, strlen(sql));
		bind(S, 1, val1);
	}

	~select() {
		sqlite3_finalize(S);
	}

	operator bool() {
		switch (sqlite3_step(S)) {
		case SQLITE_DONE:
			return 0;
		case SQLITE_ROW:
			return 1;
		}
		throw runtime_error(sqlite3_errmsg(db));
	}

	const char* operator[](int i) {
		return get(S, i);
	}
};

// update
struct Transaction {
	Transaction();
	~Transaction();

	void insert(const char* table, const char* field1, const char* val1, const char* field2, const char* val2);
};
