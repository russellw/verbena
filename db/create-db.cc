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

#include "db.h"
#include <country.hxx>
#include <schema.hxx>

int main(int argc, char** argv) {
	try {
		if (sqlite3_open_v2(file, &db, SQLITE_OPEN_READONLY, 0) == SQLITE_OK)
			throw runtime_error(string(file) + ": already exists");
		if (sqlite3_open(file, &db) != SQLITE_OK)
			throw runtime_error(string(file) + ": " + sqlite3_errmsg(db));
		exec("PRAGMA foreign_keys=ON");
		for (auto table: tables) {
			string sql = "CREATE TABLE ";
			sql += table->name;
			sql += '(';
			for (auto field = table->fields; field->name; ++field) {
				if (field != table->fields)
					sql += ',';
				def(field, sql);
			}
			sql += ") STRICT";
			println(sql);
			exec(sql);
		}
		sqlite3_close(db);
		return 0;
	} catch (exception& e) {
		sqlite3_close(db);
		println(e.what());
		return 1;
	}
}
