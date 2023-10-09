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

static PGconn* con;

void initdb(const char* info) {
	con = PQconnectdb(info);
	if (PQstatus(con) != CONNECTION_OK)
		throw runtime_error(PQerrorMessage(con));
}

void execInsert(string& sql, const vector<char*>& vals) {
	assert(sql.back() == ',');
	sql.pop_back();

	sql += ")VALUES(";
	for (int i = 0; i < vals.size(); ++i) {
		if (i)
			sql += ',';
		sql += '$';
		sql += '1' + i;
	}
	sql += ')';

	auto r = PQexecParams(con, sql.data(), vals.size(), 0, vals.data(), 0, 0, 0);
	if (PQresultStatus(r) != PGRES_COMMAND_OK)
		throw runtime_error(PQresultErrorMessage(r));
	PQclear(r);
}

Query::Query(const char* sql) {
	r = PQexec(con, sql);
	if (PQresultStatus(r) != PGRES_TUPLES_OK)
		throw runtime_error(PQresultErrorMessage(r));
	n = PQntuples(r);
}

Query::Query(const char* sql, const char* val1) {
	r = PQexecParams(con, sql, 1, 0, &val1, 0, 0, 0);
	if (PQresultStatus(r) != PGRES_TUPLES_OK)
		throw runtime_error(PQresultErrorMessage(r));
	n = PQntuples(r);
}
