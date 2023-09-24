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

#include "all.h"

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

void decl(ostream& os, string name, int n) {
	os << "char " << name << "Data[" << to_string(n) << ']';
}

int main(int argc, char** argv) {
	try {
		for (int i = 1; i < argc; ++i) {
			file = argv[i];
			auto name = path(file).stem().string();

			// parse
			readText();
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
			auto header = "HTTP/1.1 200\r\n"
						  "Content-Type:text/css;charset=utf-8\r\n"
						  "Content-Length:" +
						  to_string(o.size()) + "\r\n\r\n";

			// data.hxx
			{
				ofstream os("data.hxx", ios::app);
				os << "extern ";
				decl(os, name, header.size() + o.size());
				os << ';';
			}

			// data.cxx
			{
				ofstream os("data.cxx", ios::app);
				decl(os, name, header.size() + o.size());
				os << '{';
				for (auto c: header)
					os << (int)c << ',';
				for (auto c: o)
					os << (int)c << ',';
				os << "};";
			}
		}
		return 0;
	} catch (exception& e) {
		cout << e.what() << '\n';
		return 1;
	}
}
