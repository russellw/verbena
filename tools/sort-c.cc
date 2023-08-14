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

#include <regex>
using std::regex;
using std::smatch;

// SORT
regex assignRegex(R"((\w+) = )");
regex commentRegex(R"(\s*//.*)");
regex fnBraceRegex(R"((\w+)\(.*\{$)");
regex fnRegex(R"((\w+)\()");
regex lbraceRegex(R"(.*\{$)");
regex rbraceNamespaceRegex(R"(\} // namespace.*)");
regex rbraceRegex(R"(\s*\};?)");
regex sortCommentRegex(R"(\s*// SORT)");
regex varRegex(R"((\w+)[;,]?)");
//

string rbrace = "}";
const string& at(int i) {
	if (i < V.size())
		return V[i];
	return rbrace;
}

struct Block {
	int first, last;
	string key;

	Block(int dent, int i): first(i) {
		while (regex_match(at(i), commentRegex))
			++i;
		auto& s = at(i);
		smatch m;
		if (regex_search(s, m, fnBraceRegex)) {
			key = m[1].str() + ':' + s;
			do {
				++i;
				if (i == V.size())
					throw runtime_error(file + ':' + to_string(first + 1) + ": unclosed function");
			} while (!(indent(i) == dent && regex_match(at(i), rbraceRegex)));
			last = i + 1;
			return;
		}
		if (regex_search(s, m, lbraceRegex)) {
			key = s;
			do {
				++i;
				if (i == V.size())
					throw runtime_error(file + ':' + to_string(first + 1) + ": unclosed brace");
			} while (!(indent(i) == dent && regex_match(at(i), rbraceRegex)));
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

	void to(vector<string>& o) {
		o.insert(o.end(), V.begin() + first, V.begin() + last);
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
			readLines();
			auto old = V;
			for (int i = 0; i < V.size();) {
				if (!regex_match(V[i], sortCommentRegex)) {
					++i;
					continue;
				}

				// sortable blocks should be indented at the same level as the marker comment
				auto dent = indent(i);
				++i;

				// get group of blocks
				int j = i;
				vector<Block> blocks;
				for (;;) {
					// skip intervening blank lines
					while (at(j).empty())
						++j;

					// end of group?
					if (indent(j) < dent)
						break;
					auto& s = V[j];
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
				vector<string> o;
				for (auto block: blocks) {
					if (blanks && o.size())
						o.push_back("");
					block.to(o);
				}
				if (blanks && regex_match(at(j), commentRegex))
					o.push_back("");
				V.erase(V.begin() + i, V.begin() + j);
				V.insert(V.begin() + i, o.begin(), o.end());

				i += o.size();
			}
			if (old != V)
				writeLines();
		}
		return 0;
	} catch (exception& e) {
		println(e.what());
		return 1;
	}
}
