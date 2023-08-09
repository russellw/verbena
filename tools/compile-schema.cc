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

#include "tools.h"

string quote(const string& s) {
	return '"' + s + '"';
}

int main(int argc, char** argv) {
	try {
		if (argc < 2 || argv[1][0] == '-') {
			puts("compile-schema schema.h\n"
				 "Writes schema.hxx, schema.cxx");
			return 1;
		}
		file = argv[1];

		// source file
		readSchema();

		// schema.hxx
		file = "schema.hxx";
		outf = xfopen("wb");
		out("// AUTO GENERATED - DO NOT EDIT\n");

		for (auto table: tables) {
			out("enum{\n");
			for (auto field: table->fields)
				out(table->name + '_' + field->name + ",\n");
			out("};\n");
			out("extern Table " + table->name + "Table;\n");
		}

		out("extern Table* tables[];\n");

		fclose(outf);

		// schema.cxx
		file = "schema.cxx";
		outf = xfopen("wb");
		out("// AUTO GENERATED - DO NOT EDIT\n");
		out("#include <main.h>\n");

		for (auto table: tables) {
			out("Field " + table->name + "Fields[]{\n");
			for (auto field: table->fields) {
				out('{' + quote(field->name));
				if (field->ref)
					out(", &" + field->refName + "Table");
				else
					out(", 0");

				out(',' + to_string(field->size));

				out(',' + to_string(field->scale));
				out(", t_" + field->type);

				out(',' + to_string(field->key));
				out(',' + to_string(field->nonull));
				out("},\n");
			}
			out("0\n");
			out("};\n");

			out("Table " + table->name + "Table{" + quote(table->name) + ',' + table->name + "Fields};\n");
		}

		out("Table* tables[]{\n");
		for (auto table: tables)
			out('&' + table->name + "Table,\n");
		out("0\n");
		out("};\n");

		fclose(outf);
		return 0;
	} catch (exception& e) {
		println(e.what());
		return 1;
	}
}
