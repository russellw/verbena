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

#ifdef _WIN32
#include <windows.h>
#endif

int main(int argc, char** argv) {
	try {
		if (argc < 2)
			return 1;
		file = argv[1];
		readLines();
		bool blockComment = 0;
#ifdef _WIN32
		auto console = GetStdHandle(STD_OUTPUT_HANDLE);
#endif
		int i = 0;
		for (auto s: V) {
			++i;

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
			if (s[0] == '}' && !startsWith(s, "} //"))
				continue;

			printf("%6d  ", i);
#ifdef _WIN32
			if (startsWith(s, "//"))
				SetConsoleTextAttribute(console, FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_INTENSITY);
			else if (startsWith(s, "#"))
				SetConsoleTextAttribute(console, FOREGROUND_GREEN | FOREGROUND_INTENSITY);
			else if (startsWith(s, "namespace ") || startsWith(s, "} // namespace"))
				SetConsoleTextAttribute(console, FOREGROUND_GREEN | FOREGROUND_BLUE | FOREGROUND_INTENSITY);
			else if (startsWith(s, "class ") || startsWith(s, "struct "))
				SetConsoleTextAttribute(console, FOREGROUND_BLUE | FOREGROUND_INTENSITY);
			else
				SetConsoleTextAttribute(console, FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_BLUE | FOREGROUND_INTENSITY);
#endif
			println(s);
#ifdef _WIN32
			SetConsoleTextAttribute(console, FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_BLUE | FOREGROUND_INTENSITY);
#endif
		}
		return 0;
	} catch (exception& e) {
		println(e.what());
		return 1;
	}
}
