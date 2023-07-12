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

void readBytes(const string& file, vector<unsigned char>& v) {
	auto f = open(file.data(), O_RDONLY | O_BINARY);
	struct stat st;
	if (f < 0 || fstat(f, &st))
		throw runtime_error(file + ": " + strerror(errno));
	auto n = st.st_size;

	v.resize(n);
	read(f, v.data(), n);

	close(f);
}

void decl(const string& file, const vector<unsigned char>& v, string& o) {
	o += "const unsigned char ";
	o += file;
	o += "Data[";
	o += to_string(v.size());
	o += ']';
}

int main(int argc, char** argv) {
	try {
		if (argc < 2 || argv[1][0] == '-') {
			puts("compile-bytes file\n"
				 "Writes file.hxx, file.cxx");
			return 1;
		}

		vector<unsigned char> v;
		readBytes(argv[1], v);

		auto name = path(argv[1]).stem().string();

		// .hxx
		string o = "// AUTO GENERATED - DO NOT EDIT\n";

		o += "extern ";
		decl(name, v, o);
		o += ";\n";

		writeFile(name + ".hxx", o);

		// .cxx
		o = "// AUTO GENERATED - DO NOT EDIT\n";
		o += "#include \"";
		o += name;
		o += ".hxx\"\n";

		decl(name, v, o);
		o += "{\n";
		size_t n = 16;
		for (size_t i = 0; i < v.size(); i += n) {
			for (auto j = i; j < i + n && j < v.size(); ++j) {
				o += to_string(v[j]);
				o += ',';
			}
			o += '\n';
		}
		o += "};\n";

		writeFile(name + ".cxx", o);
		return 0;
	} catch (exception& e) {
		println(e.what());
		return 1;
	}
}
