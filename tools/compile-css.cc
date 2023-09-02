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

bool isword(int c) {
	return isalnum(c) || c == '-';
}

char* src;

[[noreturn]] void err(string msg) {
	int line = 1;
	for (auto s = text.data(); s < src; ++s)
		if (*s == '\n')
			++line;
	throw runtime_error(file + ':' + to_string(line) + ": " + msg);
}

void decl(string name, int n) {
	out("char " + name + "Data[" + to_string(n) + ']');
}

int main(int argc, char** argv) {
	try {
		if (argc < 2 || argv[1][0] == '-') {
			puts("compile-css file.css...\n"
				 "Appends data.hxx, data.cxx");
			return 1;
		}
		for (int i = 1; i < argc; ++i) {
			file = argv[i];
			auto name = path(file).stem().string();

			// parse
			readFile();
			src = text.data();
			string o;
			while (*src) {
				if (isspace(*src)) {
					++src;
					continue;
				}
				if (eq(src, "/*")) {
					src += 2;
					while (!eq(src, "*/")) {
						if (!*src)
							err("unclosed block comment");
						++src;
					}
					src += 2;
					continue;
				}
				if (isword(*src)) {
					if (o.size() && isword(o.back()))
						o += ' ';
					do
						o += *src++;
					while (isword(*src));
					continue;
				}
				o += *src++;
			}

			// HTTP header
			auto header = "HTTP/1.1 200 OK\r\nContent-Type:text/css\r\nContent-Length:" + to_string(o.size()) + "\r\n\r\n";

			// data.hxx
			file = "data.hxx";
			outf = xfopen("ab");

			out("extern ");
			decl(name, header.size() + o.size());
			out(";\n");

			fclose(outf);

			// data.cxx
			file = "data.cxx";
			outf = xfopen("ab");

			decl(name, header.size() + o.size());
			out("{");
			for (auto c: header)
				fprintf(outf, "%d,", c);
			for (auto c: o)
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
