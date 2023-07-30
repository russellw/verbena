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

#include <filesystem>
using std::filesystem::path;

// types
enum {
	// SORT
	t_date,
};

struct Type {
	int tag;
	vector<Type*> v;
};

// terms
enum {
	// SORT
	a_add,
	a_assign,
	a_call,
	a_literal,
	a_lt,
	a_mul,
	a_print,
	a_sub,
	a_word,
};

struct Term {
	int tag;
	string s;
	vector<Term*> v;

	Term(int tag, const string& s): tag(tag), s(s) {
	}

	Term(int tag, Term* a): tag(tag), v{a} {
	}

	Term(int tag, Term* a, Term* b): tag(tag), v{a, b} {
	}

	Term(int tag, const vector<Term*>& v): tag(tag), v(v) {
	}
};

// parser
Term* expr();

Term* primary() {
	switch (tok) {
	case k_literal:
		return new Term(a_literal, atom());
	case k_word:
		return new Term(a_word, word());
	}
	err("expected expression");
}

Term* postfix() {
	auto a = primary();
	if (!eat('('))
		return a;
	vector<Term*> v(1, a);
	if (tok != ')')
		do
			v.push_back(expr());
		while (eat(','));
	expect(')');
	return new Term(a_call, v);
}

struct Op {
	char prec;
	char tag;

	Op(int prec, int tag): prec(prec), tag(tag) {
	}
};

Op ops[end_k];

int prec = 99;

void op(int k, int tag) {
	ops[k] = Op(prec, tag);
}

struct Init {
	Init() {
		op('*', a_mul);

		prec--;
		op('+', a_add);
		op('-', a_sub);

		prec--;
		op('<', a_lt);
		op('>', a_lt);

		prec--;
		op('=', a_assign);
	}
} init;

Term* infix(int prec) {
	auto a = prefix();
	for (;;) {
		auto k = tok;
		auto prec1 = ops[k].prec;
		if (prec1 < prec)
			return a;
		lex();
		auto b = infix(prec1 + 1);
		switch (k) {
		case '>':
			k = '<';
			swap(a, b);
			break;
		}
		a = new Term(ops[k].tag, a, b);
	}
}

Term* expr() {
	return infix(1);
}

void literal(vector<Term*> o, const char* s) {
	o.push_back(new Term(a_print, new Term(a_literal, s)));
}

void stmt(vector<Term*> o) {
	switch (tok) {
	case k_quote:
		o.push_back(new Term(a_print, primary()));
		expect(';');
		return;
	}
	// SORT
	if (eat("script")) {
		literal(o, "<script>");
		literal(o, "</script>");
		return;
	}

	expect(';');

	o.push_back(expr());
}

// SORT
string camelCase(const string& s) {
	string r;
	for (int i = 0; i < s.size();) {
		if (s[i] == '-') {
			r += toupper(s[i + 1]);
			i += 2;
			continue;
		}
		r += s[i++];
	}
	return r;
}

bool endsWith(const string& s, const char* t) {
	auto n = strlen(t);
	if (s.size() < n)
		return 0;
	for (auto i = 0; i < n; ++i)
		if (s[s.size() - n + i] != t[i])
			return 0;
	return 1;
}

string titleCase(const string& s) {
	string r;
	for (auto c: s) {
		if (c == '-')
			c = ' ';
		r += c;
	}
	r[0] = toupper(r[0]);
	return r;
}

// as an optimization, when we output multiple consecutive string literals, fuse them together
vector<string> literals;

void literal(string s) {
	literals.push_back(s);
}

void code(string t) {
	if (literals.size()) {
		out("o +=");
		Separator separator;
		for (auto& s: literals) {
			if (separator())
				out("\n\t");
			out(esc(s));
		}
		literals.clear();
		out(";\n");
	}
	out(t);
}

// recur on the abstract syntax tree
void compose(Element* a) {
	switch (a->tag) {
	case a_grid: {
		literal("<table>");

		// table header
		literal("<tr>");
		for (auto b: a->v)
			if (b->tag == a_field) {
				literal("<th>");
				literal(titleCase(b->name));
				literal("</th>");
			}
		literal("</tr>");

		// sql
		string sql = "SELECT ";
		Separator separator;
		for (auto b: a->v)
			if (b->tag == a_field) {
				if (separator())
					sql += ',';
				sql += b->name;
			}
		sql += " FROM " + a->from;

		// table rows
		code("auto S = prep(\"" + sql + "\");\n");

		// for each row
		code("while (step(S)) {\n");
		literal("<tr>");

		// for each column
		int i = 0;
		for (auto b: a->v)
			if (b->tag == a_field) {
				literal("<td>");
				code("o += get(S," + to_string(i++) + ");\n");
				literal("</td>");
			}

		literal("</tr>");
		code("}\n");

		literal("</table>");
		return;
	}
	case a_ul:
		literal("<ul>");
		for (auto b: a->v)
			compose(b);
		literal("</ul>");
		return;
	case a_link:
		literal("<a href=\"");
		literal(a->val);
		literal("\">");
		literal(titleCase(a->val));
		literal("</a>");
		return;
	case a_h1:
		literal("<h1>");
		literal(a->val);
		literal("</h1>");
		return;
	case a_h2:
		literal("<h2>");
		literal(a->val);
		literal("</h2>");
		return;
	case a_h3:
		literal("<h3>");
		literal(a->val);
		literal("</h3>");
		return;
	case a_div:
		literal("<div");
		if (a->style.size())
			literal(" style=\"" + a->style + '"');
		literal(">");
		for (auto b: a->v)
			compose(b);
		literal("</div>");
		return;
	}
}

int main(int argc, char** argv) {
	try {
		if (argc < 3 || argv[1][0] == '-') {
			puts("compile-pages schema.h *-page.h\n"
				 "Writes pages.cxx");
			return 1;
		}

		file = argv[1];
		readSchema();

		// pages.cxx
		file = "pages.cxx";
		outf = xfopen("wb");
		out("// AUTO GENERATED - DO NOT EDIT\n");
		out("#include <main.h>\n");

		// pages
		vector<string> pages;
		for (int i = 2; i < argc; ++i) {
			auto stem = path(argv[i]).stem().string();
			auto name = camelCase(stem);
			pages.push_back(name);

			// read
			file = argv[i];
			preprocess();

			// parse
			vector<Element*> v;
			while (tok)
				v.push_back(element());

			// page generator function
			out("void " + name + "(string& o) {\n");

			// header
			assert(literals.empty());
			literal("<!DOCTYPE html>");
			literal("<html lang=\"en\">");
			literal("<head>");
			literal("<title>");
			auto title = stem;
			if (endsWith(title, "-page"))
				title = title.substr(0, title.size() - 5);
			literal(titleCase(title));
			literal("</title>");
			literal("<style>");
			literal("body{");
			literal("display:flex;");
			literal("font-family:Arial,sans-serif;");
			literal("font-size:20px;");
			literal("}");
			literal("table{");
			literal("border-collapse:collapse;");
			literal("width:100%;");
			literal("}");
			literal("th,td{");
			literal("border:1px solid #d3d3d3;");
			literal("padding:8px;");
			literal("text-align:left;");
			literal("}");
			literal("th{");
			literal("background-color:#f2f2f2;");
			literal("}");
			literal("</style>");
			literal("</head>");

			// body
			literal("<body>");
			for (auto a: v)
				compose(a);

			// flush remaining literals before closing function
			code("}\n");
		}

		// dispatch
		out("void dispatch(const char* req, string& o) {\n");
		for (auto& name: pages) {
			auto s = name;
			if (endsWith(s, "Page"))
				s = s.substr(0, s.size() - 4);
			if (s == "main")
				s.clear();
			out("if (eq(req, \"" + s + " \")) {\n");
			out(name + "(o);\n");
			out("return;\n");
			out("}\n");
		}
		out("}\n");

		fclose(outf);
		return 0;
	} catch (exception& e) {
		println(e.what());
		return 1;
	}
}
