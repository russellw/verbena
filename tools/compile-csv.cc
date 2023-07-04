/*
Copyright 2023 Russell Wallace
This file is part of Olivine.

Olivine is free software: you can redistribute it and/or modify it under the
terms of the GNU Affero General Public License as published by the Free Software
Foundation, either version 3 of the License, or (at your option) any later
version.

Olivine is distributed in the hope that it will be useful, but WITHOUT ANY
WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A
PARTICULAR PURPOSE.  See the GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License along
with Olivine.  If not, see <http:www.gnu.org/licenses/>.
*/

#include <olivine.h>
using namespace olivine;

#include <filesystem>
using std::filesystem::path;

void decl(const string& file, const vector<vector<string>>& vs, string& o) {
	// it would be slightly more efficient to define a struct
	// of which each record would be an instance
	// then each field for which the difference between average and longest value is smaller than a pointer
	// could be defined as an inline char array, instead of char*
	o += "const char*";
	o += file;
	o += "_data[";
	o += to_string(vs.size());
	o += "][";
	o += to_string(vs[0].size());
	o += ']';
}

string esc(const string& s) {
	bool q = 0;
	string r;
	for (auto c: s) {
		if (isprint1(c)) {
			if (!q) {
				r += '"';
				q = 1;
			}
			r += c;
			continue;
		}
		if (q) {
			r += '"';
			q = 0;
		}
		r += "\"\\x";
		char buf[3];
		sprintf(buf, "%02x", (unsigned char)c);
		r += buf;
		r += '"';
	}
	if (q)
		r += '"';
	return r;
}

int main(int argc, char** argv) {
	try {
		if (argc < 2 || argv[1][0] == '-') {
			puts("compile-csv file.csv\n"
				 "Writes file.hxx, file.cxx");
			return 1;
		}

		vector<vector<string>> vs;
		readCsv(argv[1], vs);

		auto file = path(argv[1]).stem().string();

		// header
		string o = "// AUTO GENERATED - DO NOT EDIT\n";

		o += "extern ";
		decl(file, vs, o);
		o += ";\n";

		writeFile(file + ".hxx", o);

		// definitions
		o = "// AUTO GENERATED - DO NOT EDIT\n";
		o += "#include \"";
		o += file;
		o += ".hxx\"\n";

		decl(file, vs, o);
		o += "{\n";
		for (auto& v: vs) {
			o += '{';
			bool comma = 0;
			for (auto& s: v) {
				if (comma)
					o += ',';
				comma = 1;
				o += esc(s);
			}
			o += "},\n";
		}
		o += "};\n";

		writeFile(file + ".cxx", o);
		return 0;
	} catch (exception& e) {
		println(e.what());
		return 1;
	}
}
