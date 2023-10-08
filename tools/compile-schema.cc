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

char* src;
string tok;

[[noreturn]] void err(string msg) {
	int line = 1;
	for (auto s = text.data(); s < src; ++s)
		if (*s == '\n')
			++line;
	throw runtime_error(file + ':' + to_string(line) + ": " + msg);
}

void lex() {
	for (;;) {
		auto src0 = src;
		switch (*src) {
		case ' ':
		case '\n':
		case '\t':
			++src;
			continue;
		case '/':
			switch (src[1]) {
			case '/':
				src = strchr(src, '\n');
				continue;
			case '*':
				src += 2;
				while (!eq(src, "*/")) {
					if (!*src)
						err("unclosed block comment");
					++src;
				}
				src += 2;
				continue;
			}
			break;
		case '0':
		case '1':
		case '2':
		case '3':
		case '4':
		case '5':
		case '6':
		case '7':
		case '8':
		case '9':
		case 'A':
		case 'B':
		case 'C':
		case 'D':
		case 'E':
		case 'F':
		case 'G':
		case 'H':
		case 'I':
		case 'J':
		case 'K':
		case 'L':
		case 'M':
		case 'N':
		case 'O':
		case 'P':
		case 'Q':
		case 'R':
		case 'S':
		case 'T':
		case 'U':
		case 'V':
		case 'W':
		case 'X':
		case 'Y':
		case 'Z':
		case '_':
		case 'a':
		case 'b':
		case 'c':
		case 'd':
		case 'e':
		case 'f':
		case 'g':
		case 'h':
		case 'i':
		case 'j':
		case 'k':
		case 'l':
		case 'm':
		case 'n':
		case 'o':
		case 'p':
		case 'q':
		case 'r':
		case 's':
		case 't':
		case 'u':
		case 'v':
		case 'w':
		case 'x':
		case 'y':
		case 'z':
			do
				++src;
			while (isalnum(*src) || *src == '_');
			tok.assign(src0, src);
			return;
		case '\'':
			++src;
			while (*src != '\'') {
				switch (*src) {
				case '\\':
					src += 2;
					continue;
				case '\n':
					err("unclosed quote");
				}
				++src;
			}
			++src;
			tok.assign(src0, src);
			return;
		case 0:
			tok.clear();
			return;
		}
		tok = *src++;
		return;
	}
}

bool eat(const char* s) {
	if (tok == s) {
		lex();
		return 1;
	}
	return 0;
}

void expect(const char* s) {
	if (!eat(s))
		err(string("expected '") + s + '\'');
}

string word() {
	if (!(tok.size() && isalnum(tok[0])))
		err("expected word");
	auto s = tok;
	lex();
	return s;
}

struct Table;

struct Field {
	// SORT
	bool key = 0;
	string name;
	bool nonull = 0;
	Table* ref = 0;
	string refName;
	string type = "text";
	//

	Field(string name): name(name) {
	}
};

struct Table {
	string name;
	vector<Field*> fields;
	vector<Table*> links;

	Table(string name): name(name) {
	}
};

vector<Table*> tables;

template <class T> void topologicalSortRecur(const vector<T>& v, vector<T>& o, unordered_set<T>& visited, T a) {
	if (!visited.insert(a).second)
		return;
	for (auto b: a->links)
		topologicalSortRecur(v, o, visited, b);
	o.push_back(a);
}

template <class T> void topologicalSort(vector<T>& v) {
	unordered_set<T> visited;
	vector<T> o;
	for (auto a: v)
		topologicalSortRecur(v, o, visited, a);
	v = o;
}

int main(int argc, char** argv) {
	try {
		if (argc < 2)
			return 1;
		file = argv[1];

		// parse
		readText();
		src = text.data();
		lex();
		while (tok.size()) {
			auto table = new Table(word());
			expect("{");
			do {
				auto field = new Field(word());

				// type
				set<string> types{
					// SORT
					"bigint",
					"date",
					"decimal",
					"integer",
					"smallint",
				};
				if (types.count(tok)) {
					field->type = tok;
					lex();
				}

				// primary key / not null
				if (eat("key"))
					field->key = 1;
				else if (eat("nonull"))
					field->nonull = 1;

				// foreign key
				if (eat("ref"))
					if (tok == ";")
						field->refName = field->name;
					else
						field->refName = word();

				expect(";");
				table->fields.push_back(field);
			} while (!eat("}"));
			tables.push_back(table);
		}

		// link table references
		unordered_map<string, Table*> tableMap;
		for (auto table: tables)
			tableMap[table->name] = table;
		for (auto table: tables)
			for (auto field: table->fields)
				if (field->refName.size()) {
					field->ref = tableMap.at(field->refName);
					auto key = field->ref->fields[0];
					field->type = key->type;
					table->links.push_back(field->ref);
				}

		// eliminate forward references
		topologicalSort(tables);

		// schema.hxx
		ofstream os("schema.hxx");

		for (auto table: tables) {
			os << "Table " << table->name << "Table{\"" << table->name << "\", vector<Field>{{";
			for (auto field: table->fields) {
				os << "{";
				os << field->key << ',';
				os << '"' << field->name << "\",";
				os << field->nonull << ',';
				if (field->ref)
					os << '&' << field->refName << "Table,";
				else
					os << "0,";
				os << '"' << field->type << '"';
				os << "},";
			}
			os << "}}};";
		}

		os << "array<Table*," << tables.size() << "> tables{";
		for (auto table: tables)
			os << '&' << table->name << "Table,";
		os << "};";

		return 0;
	} catch (exception& e) {
		cout << e.what() << '\n';
		return 1;
	}
}
