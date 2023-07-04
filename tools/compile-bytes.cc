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

void decl(const string& file, const vector<unsigned char>& v, string& o) {
	o += "const unsigned char ";
	o += file;
	o += "_data[";
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

		auto file = path(argv[1]).stem().string();

		// header
		string o = "// AUTO GENERATED - DO NOT EDIT\n";

		o += "extern ";
		decl(file, v, o);
		o += ";\n";

		writeFile(file + ".hxx", o);

		// definitions
		o = "// AUTO GENERATED - DO NOT EDIT\n";
		o += "#include \"";
		o += file;
		o += ".hxx\"\n";

		decl(file, v, o);
		o += "{\n";
		const size_t n = 16;
		for (size_t i = 0; i < v.size(); i += n) {
			for (auto j = i; j < i + n && j < v.size(); ++j) {
				o += to_string(v[j]);
				o += ',';
			}
			o += '\n';
		}
		o += "};\n";

		writeFile(file + ".cxx", o);
		return 0;
	} catch (exception& e) {
		println(e.what());
		return 1;
	}
}
