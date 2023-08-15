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

#include <filesystem>
using std::filesystem::path;

bool isseparator(int c) {
	switch (c) {
	case ',':
	case '\t':
		return 1;
	}
	return 0;
}

bool isdelimiter(int c) {
	return isseparator(c) || c == '\n' || c == '\r';
}

void plain(char*& s, vector<string>& v) {
	string t;
	while (!isdelimiter(*s))
		t += *s++;
	if (isseparator(*s))
		++s;
	v.push_back(t);
}

vector<string> names;

void readCsv(vector<vector<string>>& vs) {
	readFile();

	// field names
	auto s = text.data();
	while (*s != '\n')
		plain(s, names);
	++s;

	// data
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
			if (isseparator(*s))
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
			plain(s, v);
		}
}

int main(int argc, char** argv) {
	try {
		if (argc < 2 || argv[1][0] == '-') {
			puts("compile-csv file.csv\n"
				 "Writes file.hxx");
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

		auto structName = (char)toupper(name[0]) + name.substr(1);
		out("struct " + structName + "{\n");
		for (auto& s: names)
			out("const char*" + s + ";\n");
		out("};\n");

		out("array<" + structName + ',' + to_string(vs.size()) + '>' + name + "Data{{\n");
		for (auto& v: vs) {
			out("{");
			Separator separator;
			for (auto& s: v) {
				if (separator())
					out(",");
				out(esc(s));
			}
			out("},\n");
		}
		out("}};\n");

		fclose(outf);
		return 0;
	} catch (exception& e) {
		println(e.what());
		return 1;
	}
}
