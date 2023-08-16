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

#include <chrono>
using std::chrono::days;
using std::chrono::sys_days;
using std::chrono::year_month_day;
using namespace std::literals;

#include <random>
using std::default_random_engine;
using std::uniform_int_distribution;

// number to words
string oneWords[]{
	"zero", "one",	  "two",	"three",	"four",		"five",	   "six",	  "seven",	   "eight",	   "nine",
	"ten",	"eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen", "eighteen", "nineteen",
};

string tenWords[]{
	"",
	"",
	"twenty",
	"thirty",
	"forty",
	"fifty",
	"sixty",
	"seventy",
	"eighty",
	"ninety",
};

string thousandWords[]{
	"",
	"thousand",
	"million",
	"billion",
	"trillion",
	"quadrillion",
	"quintillion",
};

string words2(int n) {
	assert(n < 100);
	if (n < 20)
		return oneWords[n];
	auto s = tenWords[n / 10];
	if (n % 10)
		s += '-' + oneWords[n % 10];
	return s;
}

string words3(int n) {
	assert(n < 2000);
	string s;
	if (n / 100) {
		s = oneWords[n / 100] + " hundred";
		if (n % 100)
			s += " and ";
	}
	if (n % 100)
		s += words2(n % 100);
	return s;
}

string words(uint64_t n) {
	int i = 0;
	string s;
	do {
		if (n % 1000 || !n) {
			auto t = words3(n % 1000);
			if (i)
				t += ' ' + thousandWords[i];
			else if (n / 1000 && n % 1000 < 100)
				t = "and " + t;

			if (s.size())
				s = ", " + s;
			s = t + s;
		}
		++i;
		n /= 1000;
	} while (n);
	return s;
}

// database
int64_t count(const string& tableName) {
	auto S = prep("SELECT COUNT(1) FROM " + tableName);
	step(S);
	auto n = sqlite3_column_int64(S, 0);
	finish(S);
	return n;
}

bool generated(const Table* table, const Field& field) {
	return field.type == t_integer && field.key;
}

// random
default_random_engine rndEngine;

int rnd(int n) {
	uniform_int_distribution<int> d(0, n - 1);
	return d(rndEngine);
}

template <class T> T rnd(const vector<T>& v) {
	return v[rnd(v.size())];
}

// the value for a particular field may be random or deterministic
// depending on type and whether it is a primary or foreign key
string makeVal(const Table* table, int i, const Field& field) {
	if (field.key) {
		assert(field.type == t_text);
		string s(1, toupper(table->name[0]));
		s += to_string(i);
		return '\'' + s + '\'';
	}
	if (field.ref) {
		auto sql = "SELECT " + field.ref->fields[0].name + " FROM " + field.ref->name;
		auto S = prep(sql);
		vector<string> v;
		while (step(S))
			v.push_back(get(S, 0));
		auto s = rnd(v);
		if (field.type == t_text)
			return '\'' + s + '\'';
		return s;
	}
	switch (field.type) {
	case t_date: {
		auto date = sys_days(2023y / 1 / 1) + days(rnd(365));
		year_month_day ymd(date);
		char s[11];
		sprintf(s, "%04d-%02d-%02d", (int)ymd.year(), (unsigned)ymd.month(), (unsigned)ymd.day());
		return s;
	}
	case t_decimal: {
		auto s = to_string(rnd(10));
		if (field.scale) {
			s += '.';
			for (auto i = field.scale; i--;)
				s += '0' + rnd(10);
		}
		return s;
	}
	case t_integer:
		return to_string(rnd(100));
	case t_text:
		return '\'' + table->name + ' ' + field.name + ' ' + words(i) + '\'';
	}
	throw runtime_error(table->name + '.' + field.name + ": " + to_string(field.type));
}

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

		// check the database matches the schema
		for (auto table: tables)
			if (!dbtables.count(table->name))
				throw runtime_error(table->name + ": not found");

		// check there is no existing data to pollute
		for (auto table: tables) {
			if (table->name == "country")
				continue;
			if (count(table->name))
				throw runtime_error(table->name + ": already has data");
		}

		// make data
		unordered_map<const Table*, int> tableSize;
		exec("BEGIN");
		for (auto table: tables) {
			if (count(table->name))
				continue;

			// detail tables should have more records
			int n = 1;
			for (auto& field: table->fields)
				if (field.ref)
					n = max(n, tableSize.at(field.ref));
			n *= 10;
			tableSize[table] = n;

			// make the records
			for (int i = 0; i < n; ++i) {
				auto sql = "INSERT INTO " + table->name + '(';

				// supply values for fields that are not auto generated
				Separator separator;
				for (auto& field: table->fields) {
					if (generated(table, field))
						continue;
					if (separator())
						sql += ',';
					sql += field.name;
				}
				sql += ") VALUES (";

				// it's okay to not use parameters here because we control the data
				// user-supplied data always needs parameters
				separator.subsequent = 0;
				for (auto field: table->fields) {
					if (generated(table, field))
						continue;
					if (separator())
						sql += ',';
					sql += makeVal(table, i + 1, field);
				}
				sql += ')';

				if (!i)
					println(sql);
				exec(sql);
			}
		}
		exec("COMMIT");
		return 0;
	} catch (exception& e) {
		println(e.what());
		return 1;
	}
}
