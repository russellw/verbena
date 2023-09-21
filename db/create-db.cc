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
#include <csv.hxx>
#include <schema.hxx>

int main(int argc, char** argv) {
	try {
		// make sure the database doesn't already exist
		if (sqlite3_open_v2(file, &db, SQLITE_OPEN_READONLY, 0) == SQLITE_OK)
			throw runtime_error(string(file) + ": already exists");

		if (sqlite3_open(file, &db) != SQLITE_OK)
			throw runtime_error(string(file) + ": " + sqlite3_errmsg(db));
		exec("PRAGMA foreign_keys=ON");

		// create tables
		for (auto table: tables) {
			auto sql = "CREATE TABLE " + table->name + '(';
			Separator separator;
			for (auto field: table->fields) {
				if (separator())
					sql += ',';
				def(field, sql);
			}
			sql += ") STRICT";
			cout << sql << '\n';
			exec(sql);
		}

		// initial data
		exec("BEGIN");
		for (auto r: countryData) {
			auto S =
				prep("INSERT INTO country(" + countryTable.fields[0].name + ',' + countryTable.fields[1].name + ") VALUES($1,$2)");
			bind(S, 1, r.Code);
			bind(S, 2, r.Name);
			exec(S);
		}
		exec("COMMIT");

		sqlite3_close(db);
		return 0;
	} catch (exception& e) {
		sqlite3_close(db);
		cout << e.what() << '\n';
		return 1;
	}
}
