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

enum {
	// SORT
	t_date,
	t_decimal,
	t_integer,
	t_text,
};

struct Table;

struct Field {
	const char* name;
	int type;
	uint16_t size;
	bool generated;
	bool key;
	Table* ref;
};

struct Table {
	const char* name;
	Field* fields;
};

void exec(const string& sql);

struct Transaction {
	Transaction();
	~Transaction();

	void insert(const Table& table, size_t field0, const char* val0, size_t field1, const char* val1);
};
