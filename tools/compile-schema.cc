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

#include "compile.h"

#include <unordered_set>
using std::unordered_set;

template <class T> void topologicalSortRecur(const vector<T>& v, vector<T>& r, unordered_set<T>& visited, T a) {
	if (!visited.insert(a).second)
		return;
	for (auto b: a->links)
		topologicalSortRecur(v, r, visited, b);
	r.push_back(a);
}

template <class T> void topologicalSort(vector<T>& v) {
	unordered_set<T> visited;
	vector<T> r;
	for (auto a: v)
		topologicalSortRecur(v, r, visited, a);
	v = r;
}

int main(int argc, char** argv) {
	try {
		if (argc < 2 || argv[1][0] == '-') {
			puts("compile-schema schema.h\n"
				 "Writes schema.hxx, schema.cxx");
			return 1;
		}

		file = argv[1];
		readSchema();

		// eliminate forward references to make the schema palatable to SQL databases
		topologicalSort(tables);

		// schema.hxx
		string o = "// AUTO GENERATED - DO NOT EDIT\n";
		for (auto table: tables) {
			o += "enum{\n";
			for (auto field: table->fields)
				o += table->name + '_' + field->name + ",\n";
			o += "};\n";
			o += "extern Table " + table->name + "Table;\n";
		}
		o += "extern Table* tables[];\n";
		writeFile("schema.hxx", o);

		// schema.cxx
		o = "// AUTO GENERATED - DO NOT EDIT\n";
		o += "#include <main.h>\n";

		for (auto table: tables) {
			o += "Field " + table->name + "Fields[]{\n";
			for (auto field: table->fields) {
				o += '{' + quote(field->name) + ", t_" + field->type + ',' + field->size;
				o += ',' + to_string(field->nonull);
				o += ',' + to_string(field->key);
				if (field->ref)
					o += ", &" + field->refName + "Table";
				o += "},\n";
			}
			o += "0\n";
			o += "};\n";

			o += "Table " + table->name + "Table{" + quote(table->name) + ',' + table->name + "Fields};\n";
		}

		o += "Table* tables[]{\n";
		for (auto table: tables)
			o += '&' + table->name + "Table,\n";
		o += "0\n";
		o += "};\n";

		writeFile("schema.cxx", o);
		return 0;
	} catch (exception& e) {
		println(e.what());
		return 1;
	}
}