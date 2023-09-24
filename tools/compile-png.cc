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

void decl(ostream& os, string name, int n) {
	os << "unsigned char " << name << "Data[" << to_string(n) << ']';
}

int main(int argc, char** argv) {
	try {
		for (int i = 1; i < argc; ++i) {
			file = argv[i];
			auto name = path(file).stem().string();

			// input file
			ifstream is(file, ios::in | ios::binary);
			vector<unsigned char> bytes{istreambuf_iterator<char>(is), istreambuf_iterator<char>()};

			// HTTP header
			auto header = "HTTP/1.1 200\r\n"
						  "Content-Type:image/png\r\n"
						  "Content-Length:" +
						  to_string(bytes.size()) + "\r\n\r\n";

			// data.hxx
			{
				ofstream os("data.hxx", ios::app);
				os << "extern ";
				decl(os, name, header.size() + bytes.size());
				os << ';';
			}

			// data.cxx
			{
				ofstream os("data.cxx", ios::app);
				decl(os, name, header.size() + bytes.size());
				os << '{';
				for (auto c: header)
					os << (int)c << ',';
				for (auto c: bytes)
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
