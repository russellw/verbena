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
	a_dot,
	a_function,
	a_init,
	a_js,
	a_list,
	a_literal,
	a_lt,
	a_mul,
	a_not,
	a_print,
	a_select,
	a_sub,
	a_var,
	a_word,
	end_a
};

struct Term {
	// position in source text
	string file;
	int line;

	// so it can be used in error reports after parsing
	void err(string msg) {
		throw runtime_error(file + ':' + to_string(line) + ": " + msg);
	}

	// what kind of term
	int tag;

	// contents
	string s;
	vector<Term*> v;

	// constructors
	Term(int tag): file(file), line(line), tag(tag) {
	}

	Term(int tag, const string& s): file(file), line(line), tag(tag), s(s) {
	}

	Term(int tag, Term* a): file(file), line(line), tag(tag), v{a} {
	}

	// resolve names of tables, fields and variables
	void resolve(unordered_map<string, Term*> m);
};

unordered_map<string, Term*> tableTerms;

void Term::resolve(unordered_map<string, Term*> m) {
	if (tag == a_select) {
		auto table = tableTerms.at(v[0]->s);
		v[0] = table;
		for (auto field: table->v)
			if (!m.count(field->s))
				m.insert(make_pair(field->s, field));
	}
	for (auto& a: v) {
		switch (a->tag) {
		case a_assign: {
			auto y = a->v[0];
			if (!m.count(y->s)) {
				a->tag = a_init;
				y->tag = a_var;
				m.insert(make_pair(y->s, y));
			}
			break;
		}
		case a_word:
			a = m.at(a->s);
			continue;
		}
		a->resolve(m);
	}
}

// parser
Term* expr();

Term* primary() {
	switch (tok) {
	case k_literal: {
		auto a = new Term(a_literal, str);
		lex();
		return a;
	}
	case k_word: {
		auto a = new Term(a_word, str);
		lex();
		return a;
	}
	}
	err("expected expression");
}

Term* postfix() {
	auto a = primary();
	for (;;)
		switch (tok) {
		case '(':
			a = new Term(a_call, a);
			lex();
			if (tok != ')')
				do
					a->v.push_back(expr());
				while (eat(','));
			expect(')');
			break;
		case '.':
			a = new Term(a_dot, a);
			lex();
			a->v.push_back(primary());
			break;
		default:
			return a;
		}
}

Term* prefix() {
	switch (tok) {
	case '!': {
		auto a = new Term(a_not);
		a->v.push_back(prefix());
		return a;
	}
	}
	return postfix();
}

struct Op {
	char prec;
	char tag;
};

Op ops[end_k];
int prec = 99;

void op(int k, int tag) {
	ops[k].prec = prec;
	ops[k].tag = tag;
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
		a = new Term(ops[k].tag, a);
		lex();
		a->v.push_back(infix(prec1 + 1));
		switch (k) {
		case '>':
			swap(a->v[0], a->v[1]);
			break;
		}
	}
}

Term* expr() {
	return infix(1);
}

string snakeCase(const string& s) {
	string r;
	for (auto c: s) {
		if (c == '_')
			c = '-';
		r += c;
	}
	return r;
}

void literal(vector<Term*> o, string s) {
	o.push_back(new Term(a_print, new Term(a_literal, s)));
}

void stmt(vector<Term*> o) {
	switch (tok) {
	case k_quote:
		// a literal string by itself is shorthand for a print statement
		literal(o, atom());
		expect(';');
		return;
	}

	// SORT
	if (eat("function")) {
		// name
		auto f = new Term(a_function, atom());

		// parameters
		auto params = new Term(a_list);
		expect('(');
		if (tok != ')')
			do
				params->v.push_back(primary());
			while (eat(','));
		expect(')');
		f->v.push_back(params);

		// body
		expect('{');
		while (!eat('}'))
			stmt(f->v);

		// definition
		o.push_back(f);
		return;
	}

	if (eat("html")) {
		auto tag = atom();

		// open
		literal(o, '<' + tag);
		switch (tok) {
		case ';':
			literal(o, ">");
			lex();
			return;
		case '{':
			// attributes
			while (tok == '@' || tok == '&') {
				auto k = tok;
				lex();
				literal(o, ' ' + atom() + "=\"");
				if (k == '@') {
					if (eat('{')) {
						// compound attribute
						Separator separator;
						while (!eat('}')) {
							if (separator())
								literal(o, ";");
							literal(o, snakeCase(atom()) + '=');
							literal(o, atom());
						}
					} else {
						auto a = new Term(a_print);
						a->v.push_back(expr());
						o.push_back(a);
					}
				} else {
					auto a = new Term(a_js);
					expect('{');
					while (!eat('}'))
						stmt(a->v);
					o.push_back(a);
				}
				literal(o, "\"");
			}
			literal(o, ">");

			// body
			while (!eat('}'))
				stmt(o);
			break;
		default:
			err("syntax error");
		}

		// close
		literal(o, "</" + tag + '>');
		return;
	}

	if (eat("script")) {
		literal(o, "<script>");
		auto a = new Term(a_js);
		expect('{');
		while (!eat('}'))
			stmt(a->v);
		o.push_back(a);
		literal(o, "</script>");
		return;
	}

	// a statement can consist of an expression followed by a semicolon
	o.push_back(expr());
	expect(';');
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

namespace cxx {
char precs[end_a];
int prec = 99;

void op(int tag) {
	precs[tag] = prec;
}

struct Init {
	Init() {
		op(a_not);

		prec--;
		op(a_mul);

		prec--;
		op(a_add);
		op(a_sub);

		prec--;
		op(a_lt);

		prec--;
		op(a_assign);
	}
} init;

void expr(Term* parent, Term* a);

void infix(Term* parent, Term* a, const char* op) {
	auto parens = parent && precs[parent->tag] >= precs[a->tag];
	if (parens)
		out('(');
	expr(a, a->v[0]);
	out(op);
	expr(a, a->v[1]);
	if (parens)
		out(')');
}

void expr(Term* parent, Term* a) {
	switch (a->tag) {
	case a_add:
		infix(parent, a, "+");
		return;
	case a_assign:
		infix(parent, a, "=");
		return;
	case a_literal:
		out(esc(a->s));
		return;
	case a_lt:
		infix(parent, a, "<");
		return;
	case a_mul:
		infix(parent, a, "*");
		return;
	case a_not:
		out('!');
		expr(a, a->v[0]);
		return;
	case a_sub:
		infix(parent, a, "-");
		return;
	case a_var:
		out(a->s);
		return;
	}
	a->err("cxx::expr: " + to_string(a->tag));
}

void stmt(Term* a) {
	switch (a->tag) {
	case a_init:
		out("auto " + a->v[0]->s + '=');
		a = a->v[1];
		break;
	case a_print:
		out("o +=");
		a = a->v[0];
		break;
	}
	expr(0, a);
	out(";\n");
}
} // namespace cxx

// recur on the abstract syntax tree
void compose(Element* a) {
	switch (a->tag) {
	case a_grid: {
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
		literal(o, "<tr>");

		// for each column
		int i = 0;
		for (auto b: a->v)
			if (b->tag == a_field) {
				literal(o, "<td>");
				code("o += get(S," + to_string(i++) + ");\n");
				literal(o, "</td>");
			}
	}
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

		// schema.h
		readSchema();
		for (auto table: tables) {
			auto t = new Term(a_table, table->name);
			tableTerms.insert(table->name, t);
			for (auto field: table->fields)
				t->v.push_back(new Term(a_field, field->name));
		}

		// pages.cxx
		file = "pages.cxx";
		outf = xfopen("wb");
		out("// AUTO GENERATED - DO NOT EDIT\n");
		out("#include <main.h>\n");

		// pages
		vector<string> pages;
		for (int i = 2; i < argc; ++i) {
			file = argv[i];
			auto stem = path(file).stem().string();
			auto name = camelCase(stem);
			pages.push_back(name);

			// predefined header
			auto a = new Term(a_list);
			literal(a->v, "<!DOCTYPE html>");
			literal(a->v, "<html lang=\"en\">");
			literal(a->v, "<head>");
			literal(a->v, "<title>");
			auto title = stem;
			if (endsWith(title, "-page"))
				title = title.substr(0, title.size() - 5);
			literal(a->v, titleCase(title));
			literal(a->v, "</title>");
			literal(a->v, "<style>");
			literal(a->v, "body{");
			literal(a->v, "display:flex;");
			literal(a->v, "font-family:Arial,sans-serif;");
			literal(a->v, "font-size:20px;");
			literal(a->v, "}");
			literal(a->v, "table{");
			literal(a->v, "border-collapse:collapse;");
			literal(a->v, "width:100%;");
			literal(a->v, "}");
			literal(a->v, "th,td{");
			literal(a->v, "border:1px solid #d3d3d3;");
			literal(a->v, "padding:8px;");
			literal(a->v, "text-align:left;");
			literal(a->v, "}");
			literal(a->v, "th{");
			literal(a->v, "background-color:#f2f2f2;");
			literal(a->v, "}");
			literal(a->v, "</style>");
			literal(a->v, "</head>");
			literal(a->v, "<body>");

			// source file
			preprocess();
			while (tok)
				stmt(a->v);
			a->resolve();

			// page generator function
			out("void " + name + "(string& o) {\n");
			for (auto b: a->v)
				cxx::stmt(b);
			out("}\n");
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
