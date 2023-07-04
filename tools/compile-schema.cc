/*
Copyright 2023 Russell Wallace
This file is part of Olivine.

Olivine is free software: you can redistribute it and/or modify it under the
terms of the GNU Affero General Public License as published by the Free Software
Foundation, either version 3 of the License, or (at your option) any later
version.

Olivine is distributed in the hope that it will be useful, but WITHOUT ANY
WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A
PARTICULAR PURPOSE.  See the GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License along
with Olivine.  If not, see <http:www.gnu.org/licenses/>.
*/

#include <olivine.h>
using namespace olivine;

enum {
	k_word = 0x100,
};

string file;
string text;

char* tokBegin;
char* src;

int tok;
string str;

void err(const string& msg) {
	size_t line = 1;
	for (auto s = text.data(); s < tokBegin; ++s)
		if (*s == '\n')
			++line;
	throw runtime_error(file + ':' + to_string(line) + ": error: " + msg);
}

// tokenizer
void lex() {
	for (;;) {
		auto s = tokBegin = src;
		switch (*s) {
		case ' ':
		case '\f':
		case '\n':
		case '\r':
		case '\t':
			src = s + 1;
			continue;
		case '/':
			if (s[1] == '/') {
				src = strchr(s, '\n');
				continue;
			}
			if (s[1] == '*') {
				++s;
				do {
					++s;
					if (!*s)
						err("unclosed block comment");
				} while (!(s[0] == '*' && s[1] == '/'));
				src = s + 2;
				continue;
			}
			break;
		case '0':
		case '1':
		case '2':
		case '3':
		case '4':
		case '5':
		case '6':
		case '7':
		case '8':
		case '9':
		case 'A':
		case 'B':
		case 'C':
		case 'D':
		case 'E':
		case 'F':
		case 'G':
		case 'H':
		case 'I':
		case 'J':
		case 'K':
		case 'L':
		case 'M':
		case 'N':
		case 'O':
		case 'P':
		case 'Q':
		case 'R':
		case 'S':
		case 'T':
		case 'U':
		case 'V':
		case 'W':
		case 'X':
		case 'Y':
		case 'Z':
		case 'a':
		case 'b':
		case 'c':
		case 'd':
		case 'e':
		case 'f':
		case 'g':
		case 'h':
		case 'i':
		case 'j':
		case 'k':
		case 'l':
		case 'm':
		case 'n':
		case 'o':
		case 'p':
		case 'q':
		case 'r':
		case 's':
		case 't':
		case 'u':
		case 'v':
		case 'w':
		case 'x':
		case 'y':
		case 'z':
			do
				++s;
			while (isid(*s));
			str.assign(src, s);
			src = s;
			tok = k_word;
			return;
		case 0:
			tok = 0;
			return;
		}
		src = s + 1;
		tok = *s;
		return;
	}
}

// parser
bool eat(int k) {
	if (tok == k) {
		lex();
		return 1;
	}
	return 0;
}

bool eat(const char* s) {
	if (tok == k_word && str == s) {
		lex();
		return 1;
	}
	return 0;
}

void expect(char c) {
	if (!eat(c))
		err(string("expected '") + c + '\'');
}

void expect(const char* s) {
	if (!eat(s))
		err(string("expected '") + s + '\'');
}

void word(string& s) {
	if (tok != k_word)
		err("expected word");
	s = str;
	lex();
}

// schema
struct STable;

struct SField {
	string name;
	string type;
	string size = "0";
	string refName;
	STable* ref = 0;
};

struct STable {
	string name;
	vector<SField*> fields;
	vector<STable*> links;
};

bool istype(const string& s) {
	return s == "bigint" || s == "date" || s == "decimal" || s == "integer" || s == "string";
}

template <class T> void topologicalSortRecur(const vector<T>& v, vector<T>& r, unordered_set<T>& visited, T a) {
	if (visited.count(a))
		return;
	visited.insert(a);
	for (auto b: a->links)
		topologicalSortRecur(v, r, visited, b);
	r.push_back(a);
}

template <class T> void topologicalSort(vector<T>& v) {
	unordered_set<T> visited;
	vector<T> r;
	for (auto a: v)
		topologicalSortRecur(v, r, visited, a);
	v = r;
}

int main(int argc, char** argv) {
	try {
		if (argc < 2 || argv[1][0] == '-') {
			puts("compile-schema file.h\n"
				 "Writes schema.hxx, schema.cxx");
			return 1;
		}
		file = argv[1];

		// read
		text = readFile(file);

		// parse
		src = text.data();
		lex();
		vector<STable*> tables;
		while (tok) {
			expect("table");
			auto table = new STable;
			word(table->name);
			expect('{');
			do {
				auto field = new SField;

				word(field->name);

				word(field->type);
				if (!istype(field->type))
					field->refName = field->type;
				if (eat('(')) {
					word(field->size);
					expect(')');
				}

				expect(';');
				table->fields.push_back(field);
			} while (!eat('}'));
			tables.push_back(table);
		}

		// link table references
		unordered_map<string, STable*> tableMap;
		for (auto table: tables)
			tableMap[table->name] = table;
		for (auto table: tables)
			for (auto field: table->fields)
				if (field->refName.size()) {
					field->ref = tableMap.at(field->refName);
					auto key = field->ref->fields[0];
					field->type = key->type;
					field->size = key->size;
					table->links.push_back(field->ref);
				}

		// eliminate forward references to make the schema palatable to SQL databases
		topologicalSort(tables);

		// header
		string o = "// AUTO GENERATED - DO NOT EDIT\n";
		for (auto table: tables) {
			o += "enum{\n";
			for (auto field: table->fields)
				o += table->name + '_' + field->name + ",\n";
			o += "};\n";
			o += "extern Table " + table->name + "_table;\n";
		}
		o += "extern Table* tables[];\n";
		writeFile("schema.hxx", o);

		// definitions
		o = "// AUTO GENERATED - DO NOT EDIT\n";
		o += "#include <olivine.h>\n";
		o += "using namespace olivine;\n";
		o += "#include \"schema.hxx\"\n";

		for (auto table: tables) {
			o += "Field " + table->name + "_fields[]{\n";
			for (auto field: table->fields) {
				o += '{' + quote(field->name) + ",Type::" + field->type + ',' + field->size;
				if (field->ref)
					o += ",&" + field->refName + "_table";
				o += "},\n";
			}
			o += "0\n";
			o += "};\n";

			o += "Table " + table->name + "_table{" + quote(table->name) + ',' + table->name + "_fields};\n";
		}

		o += "Table* tables[]{\n";
		for (auto table: tables)
			o += '&' + table->name + "_table,\n";
		o += "0\n";
		o += "};\n";

		writeFile("schema.cxx", o);
		return 0;
	} catch (exception& e) {
		println(e.what());
		return 1;
	}
}
