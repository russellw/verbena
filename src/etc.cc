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

#include "verbena.h"

#include <fcntl.h>
#include <sys/stat.h>

#ifdef _WIN32
#include <io.h>
#else
#include <unistd.h>
#define O_BINARY 0
#endif

// SORT
void readBytes(const string& file, vector<unsigned char>& v) {
	auto f = open(file.data(), O_RDONLY | O_BINARY);
	struct stat st;
	if (f < 0 || fstat(f, &st))
		throw runtime_error(file + ": " + strerror(errno));
	auto n = st.st_size;

	v.resize(n);
	read(f, v.data(), n);

	close(f);
}

string readFile(const string& file) {
	auto f = open(file.data(), O_RDONLY | O_BINARY);
	struct stat st;
	if (f < 0 || fstat(f, &st))
		throw runtime_error(file + ": " + strerror(errno));
	auto n = st.st_size;

	string s;
	s.resize(n);
	read(f, s.data(), n);

	close(f);

	// make sure input ends with a newline, to simplify parser code
	if (s.empty() || s.back() != '\n')
		s += '\n';
	return s;
}

void readLines(const string& file, vector<string>& v) {
	auto text = readFile(file);
	auto s = text.data();
	v.clear();
	while (*s) {
		auto u = strchr(s, '\n');
		auto t = u;
		while (s < t && (t[-1] == ' ' || t[-1] == '\t' || t[-1] == '\r'))
			--t;
		v.push_back(string(s, t));
		s = u + 1;
	}
}

void writeFile(const string& file, const string& s) {
	auto f = open(file.data(), O_CREAT | O_WRONLY | O_TRUNC | O_BINARY, 0644);
	if (f < 0)
		throw runtime_error(file + ": " + strerror(errno));
	write(f, s.data(), s.size());
	close(f);
}

void writeLines(const string& file, const vector<string>& v) {
	auto f = fopen(file.data(), "wb");
	if (!f)
		throw runtime_error(file + ": " + strerror(errno));
	for (auto& s: v) {
		fwrite(s.data(), 1, s.size(), f);
		fputc('\n', f);
	}
	fclose(f);
}
