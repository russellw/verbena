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

int main(int argc, char** argv) {
	try {
		if (argc != 2 || argv[1][0] == '-') {
			puts("outline file\n"
				 "Print outline of C++ source file");
			return 0;
		}
		file = argv[1];
		readLines();
		bool blockComment = 0;
		for (auto& s: V) {
			// skip blank lines
			if (s.empty())
				continue;

			// skip block comments
			// this is heuristic, not an exact parse
			// it will not work for all possible C++ code
			if (startsWith(s, "/*"))
				blockComment = 1;
			if (endsWith(s, "*/")) {
				blockComment = 0;
				continue;
			}
			if (blockComment)
				continue;

			// skip implementation details
			if (isspace(s[0]))
				continue;

			// skip trailing boilerplate
			if (s[0] == '}')
				continue;

			// print outline
			println(s);
		}
		return 0;
	} catch (exception& e) {
		println(e.what());
		return 1;
	}
}
