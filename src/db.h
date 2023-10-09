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

void initdb(const char* info);
void execInsert(string& sql, const vector<char*>& vals);

class Query {
	PGresult* r;
	int n;

public:
	Query(const char* sql);
	Query(const char* sql, const char* val1);

	~Query() {
		PQclear(r);
	}

	bool empty() {
		return !n;
	}

	struct Iterator {
		int i;

		Iterator(int i): i(i) {
		}

		int operator*() {
			return i;
		}

		Iterator& operator++() {
			++i;
			return *this;
		}

		bool operator!=(Iterator b) const {
			return i != b.i;
		}
	};

	Iterator begin() {
		return Iterator(0);
	}

	Iterator end() {
		return Iterator(n);
	}

	char* operator()(int i, int j) {
		return PQgetvalue(r, i, j);
	}
};
