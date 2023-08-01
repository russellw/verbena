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

vector<unsigned char> bytes;

void readBytes() {
	auto f = open(file.data(), O_RDONLY | O_BINARY);
	struct stat st;
	if (f < 0 || fstat(f, &st))
		throw runtime_error(file + ": " + strerror(errno));
	auto n = st.st_size;

	bytes.resize(n);
	read(f, bytes.data(), n);

	close(f);
}

void decl(const string& name, int n) {
	out("const unsigned char " + name + "Data[" + to_string(n) + ']');
}

int main(int argc, char** argv) {
	try {
		if (argc < 2 || argv[1][0] == '-') {
			puts("compile-bytes file\n"
				 "Writes file.hxx, file.cxx");
			return 1;
		}
		file = argv[1];
		auto name = path(file).stem().string();

		// input file
		readBytes();

		// HTTP header
		auto header = "HTTP/1.1 200 OK\r\nContent-Type:image/png\r\nContent-Length:" + to_string(bytes.size()) + "\r\n\r\n";

		// .hxx
		file = name + ".hxx";
		outf = xfopen("wb");
		out("// AUTO GENERATED - DO NOT EDIT\n");

		out("extern ");
		decl(name, header.size() + bytes.size());
		out(";\n");

		fclose(outf);

		// .cxx
		file = name + ".cxx";
		outf = xfopen("wb");
		out("// AUTO GENERATED - DO NOT EDIT\n");
		out("#include \"" + name + ".hxx\"\n");

		decl(name, header.size() + bytes.size());
		out("{\n");

		for (auto c: header) {
			out('\'');
			switch (c) {
			case '\n':
				out("\\n");
				break;
			case '\r':
				out("\\r");
				break;
			default:
				out(c);
			}
			out("',");
		}
		out('\n');

		int n = 16;
		for (int i = 0; i < bytes.size(); i += n) {
			for (auto j = i; j < i + n && j < bytes.size(); ++j)
				fprintf(outf, "%d,", bytes[j]);
			out('\n');
		}

		out("};\n");

		fclose(outf);
		return 0;
	} catch (exception& e) {
		println(e.what());
		return 1;
	}
}
