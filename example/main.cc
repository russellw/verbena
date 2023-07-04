/*
Copyright 2023 Russell Wallace
This file is part of Olivine.

Olivine is free software: you can redistribute it and/or modify it under the
terms of the GNU Affero General Public License as published by the Free Software
Foundation, either version 3 of the License, or (at your option) any later
version.

Olivine is distributed in the hope that it will be useful, but WITHOUT ANY
WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A
PARTICULAR PURPOSE.  See the GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License along
with Olivine.  If not, see <http:www.gnu.org/licenses/>.
*/

#include "main.h"

Database db;

int main(int argc, char** argv) {
	try {
		bool create = 0;
		bool update = 0;
		vector<char*> args;
		for (int i = 1; i < argc; ++i) {
			auto s = argv[i];
			if (*s == '-') {
				do
					++s;
				while (*s == '-');
				switch (*s) {
				case 'C':
					create = 1;
					continue;
				case 'U':
					update = 1;
					continue;
				case 'V':
				case 'v':
					print("Postgres client ");
					println(PQlibVersion());
					return 0;
				case 'h':
					puts("example [options] [keyword=value...]\n"
						 "\n"
						 "-h  Show help\n"
						 "-v  Show version\n"
						 "\n"
						 "-C  Create a new database\n"
						 "-U  Update the database schema\n"
						 "\n"
						 "The currently recognized keywords are listed at:\n"
						 "https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-PARAMKEYWORDS");
					return 0;
				}
				throw runtime_error(string(argv[i]) + ": unknown option");
			}
			args.push_back(s);
		}

		db.init(tables, "example", create, update, args);
		if (create) {
			Transaction tx(db);
			for (size_t i = 0; i < sizeof countries_data / sizeof *countries_data; ++i)
				tx.insert(countries_table, countries_code, countries_data[i][1], countries_name, countries_data[i][0]);
		}

		server();
		return 0;
	} catch (exception& e) {
		println(e.what());
		return 1;
	}
}
