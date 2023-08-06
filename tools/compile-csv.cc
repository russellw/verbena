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

#include <filesystem>
using std::filesystem::path;

void readCsv(vector<vector<string>>& vs) {
	readFile();
	auto s = strchr(text.data(), '\n') + 1;
	vector<string> v;
	for (;;)
		switch (*s) {
		case '"': {
			++s;
			string t;
			while (*s != '"') {
				if (*s == '\n')
					throw runtime_error(file + ": unclosed quote");
				t += *s++;
			}
			++s;
			if (*s == ',' || *s == '\t')
				++s;
			v.push_back(t);
			break;
		}
		case '\n':
			++s;
			vs.push_back(v);
			v.clear();
			break;
		case '\r':
			++s;
			break;
		case 0:
			return;
		default:
			string t;
			while (!(*s == ',' || *s == '\n' || *s == '\t' || *s == '\r'))
				t += *s++;
			if (*s != '\n')
				++s;
			v.push_back(t);
		}
}

void decl(const string& name, const vector<vector<string>>& vs) {
	// it would be slightly more efficient to define a struct
	// of which each record would be an instance
	// then each field for which the difference between average and longest value is smaller than a pointer
	// could be defined as an inline char array, instead of char*
	out("const char*" + name + "Data[" + to_string(vs.size()) + "][" + to_string(vs[0].size()) + ']');
}

int main(int argc, char** argv) {
	try {
		if (argc < 2 || argv[1][0] == '-') {
			puts("compile-csv file.csv\n"
				 "Writes file.hxx, file.cxx");
			return 1;
		}
		file = argv[1];
		auto name = path(file).stem().string();

		// input file
		vector<vector<string>> vs;
		readCsv(vs);

		// .hxx
		file = name + ".hxx";
		outf = xfopen("wb");
		out("// AUTO GENERATED - DO NOT EDIT\n");

		out("extern ");
		decl(name, vs);
		out(";\n");

		fclose(outf);

		// .cxx
		file = name + ".cxx";
		outf = xfopen("wb");
		out("// AUTO GENERATED - DO NOT EDIT\n");
		out("#include \"" + name + ".hxx\"\n");

		decl(name, vs);
		out("{\n");
		for (auto& v: vs) {
			out('{');
			Separator separator;
			for (auto& s: v) {
				if (separator())
					out(',');
				out(esc(s));
			}
			out("},\n");
		}
		out("};\n");

		fclose(outf);
		return 0;
	} catch (exception& e) {
		println(e.what());
		return 1;
	}
}
