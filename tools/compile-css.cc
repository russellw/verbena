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

string o;

void parse() {
	for (;;) {
		switch (*src) {
		case ' ':
		case '\f':
		case '\n':
		case '\r':
		case '\t':
			++src;
			continue;
		case '-':
		case '0':
		case '1':
		case '2':
		case '3':
		case '4':
		case '5':
		case '6':
		case '7':
		case '8':
		case '9':
		case 'A':
		case 'B':
		case 'C':
		case 'D':
		case 'E':
		case 'F':
		case 'G':
		case 'H':
		case 'I':
		case 'J':
		case 'K':
		case 'L':
		case 'M':
		case 'N':
		case 'O':
		case 'P':
		case 'Q':
		case 'R':
		case 'S':
		case 'T':
		case 'U':
		case 'V':
		case 'W':
		case 'X':
		case 'Y':
		case 'Z':
		case 'a':
		case 'b':
		case 'c':
		case 'd':
		case 'e':
		case 'f':
		case 'g':
		case 'h':
		case 'i':
		case 'j':
		case 'k':
		case 'l':
		case 'm':
		case 'n':
		case 'o':
		case 'p':
		case 'q':
		case 'r':
		case 's':
		case 't':
		case 'u':
		case 'v':
		case 'w':
		case 'x':
		case 'y':
		case 'z':
			if (o.size() && isword(o.back()))
				o += ' ';
			do
				o += *src++;
			while (isword(*src));
			continue;
		case '/':
			if (src[1] == '*') {
				src += 2;
				while (!eq(src, "*/")) {
					if (!*src)
						err("unclosed block comment");
					++src;
				}
				src += 2;
				continue;
			}
			break;
		case 0:
			return;
		}
		o += *src++;
	}
}

void decl(string name, int n) {
	out("const char " + name + "Data[" + to_string(n) + ']');
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
			parse();

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
			out("=" + esc(header + o) + ";\n");

			fclose(outf);
		}
		return 0;
	} catch (exception& e) {
		println(e.what());
		return 1;
	}
}
