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
		// obviously the production version will need proper password configuration
		initdb("user=postgres password=a");

		// create database
		exec("CREATE DATABASE verbena TEMPLATE template0 ENCODING 'UTF8' LOCALE 'en_US.UTF-8'");

		// reconnect to our database
		PQfinish(con);
		initdb("dbname=verbena user=postgres password=a");

		// create tables
		for (auto table: tables) {
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

		// initial data
		for (auto r: countryData)
			exec("INSERT INTO country(" + countryTable.fields[0].name + ',' + countryTable.fields[1].name + ") VALUES($1,$2)",
				 r.Code,
				 r.Name);
		return 0;
	} catch (exception& e) {
		cout << e.what() << '\n';
		return 1;
	}
}
