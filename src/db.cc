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

#include "all.h"

sqlite3* db;

namespace {
void exec(const string& sql) {
	char* msg;
	if (sqlite3_exec(db, sql.data(), 0, 0, &msg) != SQLITE_OK)
		throw runtime_error(msg);
}
} // namespace

struct Init {
	Init() {
		try {
			auto file = "C:\\Users\\Public\\Documents\\verbena.db";
			if (sqlite3_open(file, &db) != SQLITE_OK)
				throw runtime_error(string(file) + ": " + sqlite3_errmsg(db));
			exec("PRAGMA foreign_keys=ON");
		} catch (exception& e) {
			cout << e.what() << '\n';
			exit(1);
		}
	}

	~Init() {
		// this is needed to clean up the WAL file
		sqlite3_close(db);
	}
} init;

sqlite3_stmt* prep(const char* sql, int len) {
	sqlite3_stmt* S;
	if (sqlite3_prepare_v2(db, sql, len, &S, 0) != SQLITE_OK)
		throw runtime_error(string(sql) + ": " + sqlite3_errmsg(db));
	return S;
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
