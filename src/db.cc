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
