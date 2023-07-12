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
using std::smatch;

// SORT
regex assignRegex(R"((\w+) = )");
regex commentRegex(R"(\s*//.*)");
regex fnBraceRegex(R"((\w+)\(.*\{$)");
regex fnRegex(R"((\w+)\()");
regex rbraceNamespaceRegex(R"(\} // namespace.*)");
regex rbraceRegex(R"(\s*\})");
regex rbraceSemiRegex(R"(\s*\};?)");
regex sortCommentRegex(R"(\s*// SORT)");
regex structBraceRegex(R"((\w+) \{$)");
regex varRegex(R"((\w+)[;,])");
//

string file;
vector<string> v;

string rbrace = "}";
const string& at(size_t i) {
	if (i < v.size())
		return v[i];
	return rbrace;
}

struct Block {
	size_t first, last;
	string key;

	Block(int dent, size_t i): first(i) {
		while (regex_match(at(i), commentRegex))
			++i;
		auto& s = at(i);
		smatch m;
		if (regex_search(s, m, fnBraceRegex)) {
			key = m[1].str() + ':' + s;
			do {
				++i;
				if (i == v.size())
					throw runtime_error(file + ':' + to_string(first + 1) + ": unclosed function");
			} while (!(indent(v, i) == dent && regex_match(at(i), rbraceRegex)));
			last = i + 1;
			return;
		}
		if (regex_search(s, m, structBraceRegex)) {
			key = m[1].str() + ':' + s;
			do {
				++i;
				if (i == v.size())
					throw runtime_error(file + ':' + to_string(first + 1) + ": unclosed definition");
			} while (!(indent(v, i) == dent && regex_match(at(i), rbraceSemiRegex)));
			last = i + 1;
			return;
		}
		if (regex_search(s, m, assignRegex) || regex_search(s, m, fnRegex) || regex_search(s, m, varRegex)) {
			key = m[1].str() + ':' + s;
			last = i + 1;
			return;
		}
		throw runtime_error(file + ':' + to_string(i + 1) + ": unknown syntax");
	}

	bool operator<(const Block& b) {
		return key < b.key;
	}

	void to(vector<string>& r) {
		r.insert(r.end(), v.begin() + first, v.begin() + last);
	}
};

int main(int argc, char** argv) {
	try {
		for (int i = 1; i < argc; ++i) {
			if (argv[i][0] == '-') {
				puts("sort-c file...");
				return 0;
			}
			file = argv[i];
			readLines(file, v);
			auto old = v;
			for (size_t i = 0; i < v.size();) {
				if (!regex_match(v[i], sortCommentRegex)) {
					++i;
					continue;
				}

				// sortable blocks should be indented at the same level as the marker comment
				auto dent = indent(v, i);
				++i;

				// get group of blocks
				size_t j = i;
				vector<Block> blocks;
				for (;;) {
					// skip intervening blank lines
					while (at(j).empty())
						++j;

					// end of group?
					if (indent(v, j) < dent)
						break;
					auto& s = v[j];
					if (regex_match(s, commentRegex))
						break;
					if (regex_match(s, rbraceNamespaceRegex))
						break;

					// get the next block
					Block block(dent, j);
					j = block.last;
					blocks.push_back(block);
				}

				// sort
				sort(blocks.begin(), blocks.end());

				// if blocks are multiline, separate with blank lines
				bool blanks = 0;
				for (auto block: blocks)
					if (block.last - block.first > 1) {
						blanks = 1;
						break;
					}

				// update
				vector<string> r;
				for (auto block: blocks) {
					if (blanks && r.size())
						r.push_back("");
					block.to(r);
				}
				if (blanks && regex_match(at(j), commentRegex))
					r.push_back("");
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
