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

void decl(string name, int n) {
	out("unsigned char " + name + "Data[" + to_string(n) + ']');
}

int main(int argc, char** argv) {
	try {
		if (argc < 2 || argv[1][0] == '-') {
			puts("compile-png file.png...\n"
				 "Appends data.hxx, data.cxx");
			return 1;
		}
		for (int i = 1; i < argc; ++i) {
			file = argv[i];
			auto name = path(file).stem().string();

			// input file
			ifstream is(file, std::ios::in | std::ios::binary);
			vector<unsigned char> bytes{istreambuf_iterator<char>(is), istreambuf_iterator<char>()};

			// HTTP header
			auto header = "HTTP/1.1 200 OK\r\nContent-Type:image/png\r\nContent-Length:" + to_string(bytes.size()) + "\r\n\r\n";

			// data.hxx
			file = "data.hxx";
			outf = xfopen("ab");

			out("extern ");
			decl(name, header.size() + bytes.size());
			out(";\n");

			fclose(outf);

			// data.cxx
			file = "data.cxx";
			outf = xfopen("ab");

			decl(name, header.size() + bytes.size());
			out("{");
			for (auto c: header)
				fprintf(outf, "%d,", c);
			for (auto c: bytes)
				fprintf(outf, "%d,", c);
			out("};\n");

			fclose(outf);
		}
		return 0;
	} catch (exception& e) {
		println(e.what());
		return 1;
	}
}
