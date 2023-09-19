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

char* body(char* s) {
	s = strstr(s, "\r\n\r\n");
	if (!s)
		throw runtime_error("HTTP request has no body");
	return s;
}

static char* unesc(char* s) {
	auto r = s;
	while (*s != '"') {
		if (*s == '\\')
			++s;
		if (!*s)
			throw runtime_error("Unclosed quote in JSON value");
		*r++ = *s++;
	}
	return s;
}

void jsonParse(char*& s, vector<char*>& vals) {
	auto t = s;
	vals.push_back(t);
	while (*t != '"') {
		switch (*s) {
		case '\\':
		case 0:
			t = unesc(s);
			break;
		default:
			continue;
		}
		break;
	}
	*t = 0;
	s = t + 1;
}
