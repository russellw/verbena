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
#include <schema.hxx>

int main(int argc, char** argv) {
	try {
		if (sqlite3_open_v2(file, &db, SQLITE_OPEN_READWRITE, 0) != SQLITE_OK)
			throw runtime_error(string(file) + ": " + sqlite3_errmsg(db));
		exec("PRAGMA foreign_keys=ON");

		// get existing tables
		auto S = prep("SELECT name FROM sqlite_master WHERE type='table'");
		unordered_set<string> dbtables;
		while (step(S))
			dbtables.insert(get(S, 0));

		// compare schema with database
		for (auto table: tables)
			if (dbtables.count(table->name)) {
				// existing fields
				auto S = prep("PRAGMA table_info(" + table->name + ')');
				unordered_set<string> dbfields;
				while (step(S))
					dbfields.insert(get(S, 1));

				// new fields
				for (auto field: table->fields)
					if (!dbfields.count(field.name)) {
						auto sql = "ALTER TABLE " + table->name + " ADD COLUMN ";
						def(field, sql);
						println(sql);
						exec(sql);
					}
			} else {
				// new table
				auto sql = "CREATE TABLE " + table->name + '(';
				Separator separator;
				for (auto field: table->fields) {
					if (separator())
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
