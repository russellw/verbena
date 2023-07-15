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

#include "tools.h"

#include <chrono>
using std::chrono::days;
using std::chrono::sys_days;
using std::chrono::year_month_day;
using namespace std::literals;

#include <random>
using std::default_random_engine;
using std::uniform_int_distribution;

#include "../sqlite/sqlite3.h"

sqlite3* db;

void exec(const string& sql) {
	char* msg;
	if (sqlite3_exec(db, sql.data(), 0, 0, &msg) != SQLITE_OK)
		throw runtime_error(msg);
}

struct Init {
	Init() {
		try {
			// open database
			auto file = "C:\\Users\\Public\\Documents\\verbena.db";
			if (sqlite3_open(file, &db) != SQLITE_OK)
				throw runtime_error(string(file) + ": " + sqlite3_errmsg(db));
			exec("PRAGMA foreign_keys=ON");
		} catch (exception& e) {
			println(e.what());
			exit(1);
		}
	}

	~Init() {
		// this is needed to clean up the WAL file
		sqlite3_close(db);
	}
} init;

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

int64_t count(const string& tableName) {
	auto S = prep("SELECT COUNT(1) FROM " + tableName);
	step(S);
	auto n = sqlite3_column_int64(S, 0);
	finish(S);
	return n;
}

bool generated(Table* table, Field* field) {
	return field == table->fields[0] && field->type == "integer";
}

default_random_engine rndEngine;

int rnd(int n) {
	uniform_int_distribution<int> d(0, n - 1);
	return d(rndEngine);
}

template <class T> T rnd(const vector<T>& v) {
	return v[rnd(v.size())];
}

string rndVal(Field* field) {
	if (field->ref) {
		auto table = field->ref;
		auto sql = "SELECT " + table->fields[0]->name + " FROM " + table->name;
		auto S = prep(sql);
		vector<string> v;
		while (step(S))
			v.push_back(get(S, 0));
		return rnd(v);
	}
	if (field->type == "text") {
		string s;
		for (int i = 1; i < 20; i++) {
			if (s.size() && !rnd(5))
				s += ' ';
			s += 'a' + rnd(26);
		}
		return '\'' + s + '\'';
	}
	if (field->type == "date") {
		auto date = sys_days(2023y / 1 / 1) + days(rnd(365));
		year_month_day ymd(date);
		char s[11];
		sprintf(s, "%04d-%02d-%02d", (int)ymd.year(), (unsigned)ymd.month(), (unsigned)ymd.day());
		return s;
	}
	throw runtime_error(field->name + ' ' + field->type);
}

int main(int argc, char** argv) {
	try {
		if (argc < 2 || argv[1][0] == '-') {
			puts("test-data schema.h\n"
				 "Writes random data to the database, if empty");
			return 1;
		}

		// read schema
		file = argv[1];
		readSchema();

		// get existing tables
		auto S = prep("SELECT name FROM sqlite_master WHERE type='table'");
		unordered_set<string> dbtables;
		while (step(S))
			dbtables.insert(get(S, 0));

		// check the database matches the schema
		for (auto table: tables)
			if (!dbtables.count(table->name))
				throw runtime_error(table->name + ": not found");

		// check there is no existing data to pollute
		for (auto table: tables) {
			if (table->name == "country")
				continue;
			if (count(table->name))
				throw runtime_error(table->name + ": already has data");
		}

		// random data
		exec("BEGIN");
		for (auto table: tables) {
			if (count(table->name))
				continue;
			for (int i = 0; i < 10; ++i) {
				auto sql = "INSERT INTO " + table->name + '(';

				// supply values for fields that are not auto generated
				Separator separator;
				for (auto field: table->fields) {
					if (generated(table, field))
						continue;
					if (separator())
						sql += ',';
					sql += field->name;
				}
				sql += ") VALUES (";

				// it's okay to not use parameters here because we control the data
				// user-supplied data always needs parameters
				separator.subsequent = 0;
				for (auto field: table->fields) {
					if (generated(table, field))
						continue;
					if (separator())
						sql += ',';
					sql += rndVal(field);
				}
				sql += ')';

				if (!i)
					println(sql);
				exec(sql);
			}
		}
		exec("COMMIT");
		return 0;
	} catch (exception& e) {
		println(e.what());
		return 1;
	}
}
