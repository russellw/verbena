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

#include "main.h"

#include "../sqlite/sqlite3.h"

static void def(Field* field, string& sql) {
	// name
	sql += field->name;

	// type
	if (field->type == t_integer)
		sql += " INTEGER";
	else
		sql += " TEXT";

	// primary key
	if (field->key)
		sql += " PRIMARY KEY";

	// foreign key
	if (field->ref) {
		sql += " REFERENCES ";
		sql += field->ref->name;
		sql += '(';
		sql += field->ref->fields->name;
		sql += ')';
	}
}

sqlite3* db;

void exec(const string& sql) {
	char* msg;
	if (sqlite3_exec(db, sql.data(), 0, 0, &msg) != SQLITE_OK)
		throw runtime_error(msg);
}

struct Init {
	Init() {
		auto file = "C:\\Users\\Public\\Documents\\verbena.db";
		if (sqlite3_open(file, &db) != SQLITE_OK)
			throw runtime_error(string(file) + ": " + sqlite3_errmsg(db));
		exec("PRAGMA foreign_keys=ON");

		if (1) {
			// create tables
			exec("BEGIN");
			auto tp = tables;
			while (auto table = *tp++) {
				string sql = "CREATE TABLE ";
				sql += table->name;
				sql += '(';
				for (auto field = table->fields; field->name; ++field) {
					if (field != table->fields)
						sql += ',';
					def(field, sql);
				}
				sql += ")STRICT";
				exec(sql);
			}

			exec("COMMIT");

			Transaction tx;
			for (size_t i = 0; i < sizeof countries_data / sizeof *countries_data; ++i)
				tx.insert(countries_table, countries_code, countries_data[i][1], countries_name, countries_data[i][0]);
			return;
		}
	}

	~Init() {
		if (sqlite3_close(db) != SQLITE_OK)
			puts(sqlite3_errmsg(db));
	}
} _;

Transaction::Transaction() {
	exec("BEGIN");
}

Transaction::~Transaction() {
	exec("COMMIT");
}

static void execParams(const string& sql, const char* val0, const char* val1) {
	const char* vals[]{val0, val1};
}

void Transaction::insert(const Table& table, size_t field0, const char* val0, size_t field1, const char* val1) {
	string sql = "INSERT INTO ";
	sql += table.name;
	sql += '(';
	sql += table.fields[field0].name;
	sql += ',';
	sql += table.fields[field1].name;
	sql += ")VALUES($1,$2)";
	execParams(sql, val0, val1);
}
