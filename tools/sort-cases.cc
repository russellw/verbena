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

#include <regex>
using std::regex;


// SORT
regex caseRegex(R"(\s*(case|default)\W.*)");
regex rbraceRegex(R"(\s*\})");
regex switchRegex(R"(\s*switch .*)");
//

string file;
vector<string> v;

struct Block {
	size_t first, last;

	Block(int dent, size_t i): first(i) {
		if (i == v.size())
			throw runtime_error(file + ": unexpected end of file");
		if (!regex_match(v[i], caseRegex))
			throw runtime_error(file + ':' + to_string(i + 1) + ": expected case");
		do {
			++i;
			if (i == v.size())
				throw runtime_error(file + ": unexpected end of file");
		} while (regex_match(v[i], caseRegex));
		do
			++i;
		while (dent < indent(v, i));
		last = i;
	}

	string& key() const {
		return v[first];
	}

	bool operator<(const Block& b) {
		return key() < b.key();
	}

	void to(vector<string>& r) {
		auto i = last;
		while (first < i && v[i - 1].empty())
			--i;
		r.insert(r.end(), v.begin() + first, v.begin() + i);
	}
};

int main(int argc, char** argv) {
	try {
		for (int i = 1; i < argc; ++i) {
			if (argv[i][0] == '-') {
				puts("sort-cases file...");
				return 0;
			}
			file = argv[i];
			readLines(file, v);
			auto old = v;

			// case labels
			for (size_t i = 0; i < v.size();) {
				if (!regex_match(v[i], caseRegex)) {
					++i;
					continue;
				}

				// group of case labels
				auto j = i + 1;
				while (regex_match(v[j], caseRegex))
					++j;

				// does the last one have a brace?
				auto& s = v[j - 1];
				bool brace = 0;
				if (s.back() == '{') {
					s.pop_back();
					if (s.back() == ' ')
						s.pop_back();
					brace = 1;
				}

				// just sorting lines, so can sort in place
				sort(v.begin() + i, v.begin() + j);

				// brace needs to be restored on the new last label
				if (brace)
					v[j - 1] += " {";

				i = j;
			}

			// case blocks
			for (size_t i = 0; i < v.size();) {
				if (!regex_match(v[i], switchRegex)) {
					++i;
					continue;
				}

				auto dent = indent(v, i);
				++i;

				// if an entire case block is surrounded by preprocessor directives
				// the syntax is too complicated to handle confidently
				// so bail
				if (v[i][0] == '#')
					continue;

				// get group of blocks
				size_t j = i;
				vector<Block> blocks;
				for (;;) {
					// end of group?
					if (indent(v, j) == dent && regex_match(v[j], rbraceRegex))
						break;

					// get the next block
					Block block(dent, j);
					j = block.last;
					blocks.push_back(block);
				}

				// sort
				sort(blocks.begin(), blocks.end());

				// update
				vector<string> r;
				for (auto block: blocks)
					block.to(r);
				v.erase(v.begin() + i, v.begin() + j);
				v.insert(v.begin() + i, r.begin(), r.end());

				i += r.size();
			}

			if (old != v)
				writeLines(file, v);
		}
		return 0;
	} catch (exception& e) {
		println(e.what());
		return 1;
	}
}
