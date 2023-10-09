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
#include <schema.hxx>

int main(int argc, char** argv) {
	try {
		initdb("dbname=verbena user=postgres password=a");

		// get existing tables
		auto r = exec("SELECT tablename FROM pg_tables WHERE schemaname='public'");
		unordered_set<string> dbtables;
		for (int i = 0; i < PQntuples(r); ++i)
			dbtables.insert(PQgetvalue(r, i, 0));

		// compare schema with database
		for (auto table: tables)
			if (dbtables.count(table->name)) {
				// existing fields
				auto r = exec("SELECT column_name FROM information_schema.columns WHERE table_name='" + table->name + '\'');
				unordered_set<string> dbfields;
				for (int i = 0; i < PQntuples(r); ++i)
					dbtables.insert(PQgetvalue(r, i, 0));

				// new fields
				for (auto field: table->fields)
					if (!dbfields.count(field.name)) {
						auto sql = "ALTER TABLE " + table->name + " ADD COLUMN ";
						def(field, sql);
						cout << sql << '\n';
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
				sql += ')';
				cout << sql << '\n';
				exec(sql);
			}

		return 0;
	} catch (exception& e) {
		cout << e.what() << '\n';
		return 1;
	}
}
