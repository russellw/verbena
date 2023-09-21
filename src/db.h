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

extern sqlite3* db;

// prepared statements
sqlite3_stmt* prep(const char* sql, int len);

inline sqlite3_stmt* prep(const char* sql) {
	return prep(sql, strlen(sql) + 1);
}

sqlite3_stmt* prep(const string& sql);
void bind(sqlite3_stmt* S, int i, const char* val);

// commands
void exec(sqlite3_stmt* S);
void execInsert(string& sql, const vector<char*>& vals);

// queries
bool step(sqlite3_stmt* S);
const char* get(sqlite3_stmt* S, int i);
