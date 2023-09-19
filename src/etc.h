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

#ifdef NDEBUG
#define debug(a)
#else
#define debug(a) cout << __FILE__ << ':' << __LINE__ << ": " << __func__ << ": " << #a << ": " << a << '\n'
#endif

struct Separator {
	bool subsequent = 0;

	bool operator()() {
		auto a = subsequent;
		subsequent = 1;
		return a;
	}
};

char* body(char* s);

inline bool eq(const char* s, const char* t) {
	return memcmp(s, t, strlen(t)) == 0;
}

void jsonParse(char*& s, vector<char*>& vals);

inline void jsonField1(const char* name, char*& s, vector<char*>& vals) {
	if (eq(s, name)) {
		s += strlen(name) + 2;
		jsonParse(s, vals);
	}
}

#define jsonField(name) jsonField1(name##"\"", s, vals)
