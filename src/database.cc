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

#include "verbena.h"

namespace verbena {
static void def(Field* field, string& sql) {
	// name
	sql += field->name;

	// type
	sql += ' ';
	switch (field->type) {
	case t_bigint:
		sql += "BIGINT";
		break;
	case t_date:
		sql += "DATE";
		break;
	case t_decimal:
		sql += "DECIMAL";
		break;
	case t_integer:
		sql += "INTEGER";
		break;
	case t_smallint:
		sql += "SMALLINT";
		break;
	case t_string:
		sql += "VARCHAR";
		if (field->size > 0) {
			sql += '(';
			sql += to_string(field->size);
			sql += ')';
		}
		break;
	}
	if (field->generated)
		sql += " GENERATED ALWAYS AS IDENTITY";

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

void Database::init(Table** tables, const char* dbname, bool create, bool update, const vector<char*>& args) {
	// parse args
	for (auto s: args) {
		auto t = strchr(s, '=');
		if (!t)
			throw runtime_error(string(s) + ": expected '='");
		*t++ = 0;

		if (strcmp(s, "dbname") == 0) {
			dbname = t;
			continue;
		}
		keywords.push_back(s);
		values.push_back(t);
	}

	if (create) {
		// connect to the server with no database name yet
		keywords.push_back(0);
		auto con = connect();

		// create the database
		string sql = "CREATE DATABASE ";
		sql += dbname;
		sql += " TEMPLATE template0 ENCODING 'UTF8' LOCALE 'en_US.UTF-8'";
		exec(con, sql);

		PQfinish(con);
		keywords.pop_back();
	}

	// database name is the final parameter
	keywords.push_back("dbname");
	values.push_back(dbname);

	keywords.push_back(0);

	if (create) {
		// create tables
		auto con = connect();
		exec(con, "BEGIN");

		while (auto table = *tables++) {
			string sql = "CREATE TABLE ";
			sql += table->name;
			sql += '(';
			for (auto field = table->fields; field->name; ++field) {
				if (field != table->fields)
					sql += ',';
				def(field, sql);
			}
			sql += ')';
			exec(con, sql);
		}

		exec(con, "COMMIT");
		PQfinish(con);
		return;
	}
}

PGconn* Database::connect() {
	auto con = PQconnectdbParams(keywords.data(), values.data(), 0);
	if (PQstatus(con) != CONNECTION_OK)
		throw runtime_error(PQerrorMessage(con));
	return con;
}

void exec(PGconn* con, const string& sql) {
	auto r = PQexec(con, sql.data());
	if (PQresultStatus(r) != PGRES_COMMAND_OK)
		throw runtime_error(PQresultErrorMessage(r));
}

Transaction::Transaction(Database& db) {
	con = db.connect();
	exec(con, "BEGIN");
}

Transaction::~Transaction() {
	exec(con, "COMMIT");
	PQfinish(con);
}

static void execParams(PGconn* con, const string& sql, const char* val0, const char* val1) {
	const char* vals[] = {val0, val1};
	auto r = PQexecParams(con, sql.data(), 2, 0, vals, 0, 0, 0);
	if (PQresultStatus(r) != PGRES_COMMAND_OK)
		throw runtime_error(PQresultErrorMessage(r));
}

void Transaction::insert(const Table& table, size_t field0, const char* val0, size_t field1, const char* val1) {
	string sql = "INSERT INTO ";
	sql += table.name;
	sql += '(';
	sql += table.fields[field0].name;
	sql += ',';
	sql += table.fields[field1].name;
	sql += ")VALUES($1,$2)";
	execParams(con, sql, val0, val1);
}
} // namespace verbena
