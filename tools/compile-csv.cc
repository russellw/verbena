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

void readCsv(const string& file, vector<vector<string>>& vs) {
	auto text = readFile(file);
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
		case '\r':
			++s;
			break;
		case '\n':
			++s;
			vs.push_back(v);
			v.clear();
			break;
		case 0:
			return;
		default: {
			string t;
			while (!(*s == ',' || *s == '\n' || *s == '\t' || *s == '\r'))
				t += *s++;
			if (*s != '\n')
				++s;
			v.push_back(t);
		}
		}
}

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
