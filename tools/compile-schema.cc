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

#include "tools.h"

#include "parser.h"

struct Table;

struct Field {
	// SORT
	bool key = 0;
	string name;
	bool nonull = 0;
	Table* ref = 0;
	string refName;
	string scale = "2";
	string size = "0";
	string type = "text";
	//

	Field(const string& name): name(name) {
	}
};

struct Table {
	string name;
	vector<Field*> fields;
	vector<Table*> links;

	Table(const string& name): name(name) {
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

string quote(const string& s) {
	return '"' + s + '"';
}

int main(int argc, char** argv) {
	try {
		if (argc < 2 || argv[1][0] == '-') {
			puts("compile-schema schema.h\n"
				 "Writes schema.hxx");
			return 1;
		}
		file = argv[1];

		// parse
		preprocess();
		while (tok) {
			auto table = new Table(atom());
			expect('{');
			do {
				auto field = new Field(atom());

				// type
				if (eat("date"))
					field->type = "date";
				else if (eat("decimal")) {
					field->type = "decimal";
					if (eat('(')) {
						field->size = atom();
						if (eat(','))
							field->scale = atom();
						expect(')');
					}
				} else if (eat("integer"))
					field->type = "integer";

				// primary key / not null
				if (eat("key"))
					field->key = 1;
				else if (eat("nonull"))
					field->nonull = 1;

				// foreign key
				if (eat("ref"))
					if (tok == ';')
						field->refName = field->name;
					else
						field->refName = atom();

				expect(';');
				table->fields.push_back(field);
			} while (!eat('}'));
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
					field->size = key->size;
					table->links.push_back(field->ref);
				}

		// eliminate forward references
		topologicalSort(tables);

		// schema.hxx
		file = "schema.hxx";
		outf = xfopen("wb");
		out("// AUTO GENERATED - DO NOT EDIT\n");

		for (auto table: tables) {
			out("Table " + table->name + "Table{" + quote(table->name) + '\n');
			out(", vector<Field>{{\n");
			for (auto field: table->fields) {
				out("{");
				out(to_string(field->key));
				out(",");
				out(quote(field->name));
				out(",");
				out(to_string(field->nonull));
				out(",");
				if (field->ref)
					out('&' + field->refName + "Table");
				else
					out("0");
				out(",");
				out(field->scale);
				out(",");
				out(field->size);
				out(",");
				out("t_" + field->type);
				out("},\n");
			}
			out("}}\n");
			out("};\n");
		}

		out("array<Table*," + to_string(tables.size()) + "> tables{\n");
		for (auto table: tables)
			out('&' + table->name + "Table,\n");
		out("};\n");

		fclose(outf);
		return 0;
	} catch (exception& e) {
		println(e.what());
		return 1;
	}
}
