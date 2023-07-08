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

template <class T> void topologicalSortRecur(const vector<T>& v, vector<T>& r, unordered_set<T>& visited, T a) {
	if (visited.count(a))
		return;
	visited.insert(a);
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
			puts("compile-schema file.h\n"
				 "Writes schema.hxx, schema.cxx");
			return 1;
		}
		file = argv[1];
		vector<STable*> tables;
		parseSchema(tables);

		// eliminate forward references to make the schema palatable to SQL databases
		topologicalSort(tables);

		// header
		string o = "// AUTO GENERATED - DO NOT EDIT\n";
		for (auto table: tables) {
			o += "enum{\n";
			for (auto field: table->fields)
				o += table->name + '_' + field->name + ",\n";
			o += "};\n";
			o += "extern Table " + table->name + "_table;\n";
		}
		o += "extern Table* tables[];\n";
		writeFile("schema.hxx", o);

		// definitions
		o = "// AUTO GENERATED - DO NOT EDIT\n";
		o += "#include <verbena.h>\n";
		o += "using namespace verbena;\n";
		o += "#include \"schema.hxx\"\n";

		for (auto table: tables) {
			o += "Field " + table->name + "_fields[]{\n";
			for (auto field: table->fields) {
				o += '{' + quote(field->name) + ",t_" + field->type + ',' + field->size;
				o += ',' + to_string(field->generated);
				o += ',' + to_string(field->key);
				if (field->ref)
					o += ",&" + field->refName + "_table";
				o += "},\n";
			}
			o += "0\n";
			o += "};\n";

			o += "Table " + table->name + "_table{" + quote(table->name) + ',' + table->name + "_fields};\n";
		}

		o += "Table* tables[]{\n";
		for (auto table: tables)
			o += '&' + table->name + "_table,\n";
		o += "0\n";
		o += "};\n";

		writeFile("schema.cxx", o);
		return 0;
	} catch (exception& e) {
		println(e.what());
		return 1;
	}
}
