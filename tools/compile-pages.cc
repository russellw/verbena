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
	t_bool,
	t_date,
	t_decimal,
	t_integer,
	t_record,
	t_table,
	t_text,
};

struct Type {
	int tag;
	vector<Type*> v;

	Type(int tag): tag(tag) {
	}
};

// terms
enum {
	// SORT
	a_add,
	a_assign,
	a_call,
	a_dot,
	a_field,
	a_for,
	a_function,
	a_id,
	a_js,
	a_let,
	a_list,
	a_literal,
	a_lt,
	a_mul,
	a_not,
	a_print,
	a_select,
	a_sub,
	a_subscript,
	a_table,
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

	// only some terms have types
	Type* type = 0;

	// contents
	string s;
	vector<Term*> v;

	// constructors
	Term(int tag): file(::file), line(::line), tag(tag) {
	}

	Term(int tag, const string& s): file(::file), line(::line), tag(tag), s(s) {
	}

	Term(int tag, Term* a): file(::file), line(::line), tag(tag), v{a} {
	}
};

unordered_map<string, Term*> tableTerms;

// expressions
Term* expr();

Term* primary() {
	switch (tok) {
	case k_literal: {
		auto a = new Term(a_literal, str);
		lex();
		return a;
	}
	case k_word: {
		if (eat("select")) {
			auto a = new Term(a_select);
			expect('(');
			a->v.push_back(tableTerms.at(atom()));
			while (eat(','))
				a->v.push_back(expr());
			expect(')');
			return a;
		}
		auto a = new Term(a_id, str);
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
		case '[':
			a = new Term(a_subscript, a);
			lex();
			a->v.push_back(expr());
			expect(']');
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

// statements
unordered_set<string> selfClosing{
	"area",
	"base",
	"br",
	"col",
	"embed",
	"hr",
	"img",
	"input",
	"link",
	"meta",
	"source",
	"track",
	"wbr",
};

string snakeCase(const string& s) {
	string r;
	for (auto c: s) {
		if (c == '_')
			c = '-';
		r += c;
	}
	return r;
}

void literal(vector<Term*>& o, string s) {
	o.push_back(new Term(a_print, new Term(a_literal, s)));
}

void stmt(vector<Term*>& o);

void attrs(vector<Term*>& o) {
	for (;;)
		switch (tok) {
		case '&': {
			// JavaScript attribute
			lex();
			literal(o, ' ' + atom() + "=\"");
			auto a = new Term(a_js);
			expect('{');
			while (!eat('}'))
				stmt(a->v);
			o.push_back(a);
			literal(o, "\"");
			break;
		}
		case '@':
			lex();
			literal(o, ' ' + atom());
			switch (tok) {
			case ';':
				// boolean attribute
				lex();
				break;
			case '{': {
				// compound attribute
				literal(o, "=\"");
				lex();
				Separator separator;
				while (!eat('}')) {
					if (separator())
						literal(o, ";");
					literal(o, snakeCase(atom()) + '=');
					literal(o, atom());
					expect(';');
				}
				literal(o, "\"");
				break;
			}
			default:
				// simple attribute
				literal(o, "=\"");
				auto a = new Term(a_print);
				a->v.push_back(expr());
				expect(';');
				o.push_back(a);
				literal(o, "\"");
			}
			break;
		default:
			return;
		}
}

void stmts(vector<Term*>& o) {
	if (eat('{')) {
		while (!eat('}'))
			stmt(o);
		return;
	}
	stmt(o);
}

void stmt(vector<Term*>& o) {
	switch (tok) {
	case ';':
		lex();
		return;
	case k_quote:
		// a literal string by itself is shorthand for a print statement
		literal(o, atom());
		expect(';');
		return;
	}

	// SORT
	if (eat("for")) {
		auto a = new Term(a_for);
		expect('(');
		expect("auto");
		a->v.push_back(primary());
		expect(':');
		a->v.push_back(expr());
		expect(')');
		stmts(a->v);
		o.push_back(a);
		return;
	}

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
		literal(o, '<' + tag);
		if (selfClosing.count(tag)) {
			switch (tok) {
			case ';':
				lex();
				break;
			case '{':
				lex();
				attrs(o);
				expect('}');
				break;
			default:
				err("syntax error");
			}
			literal(o, ">");
			return;
		}
		switch (tok) {
		case ';':
			literal(o, "/>");
			lex();
			return;
		case '{':
			lex();
			attrs(o);
			literal(o, ">");
			while (!eat('}'))
				stmt(o);
			break;
		default:
			auto a = new Term(a_print);
			a->v.push_back(expr());
			expect(';');
			o.push_back(a);
		}
		literal(o, "</" + tag + '>');
		return;
	}

	if (eat("print")) {
		auto a = new Term(a_print);
		a->v.push_back(expr());
		expect(';');
		o.push_back(a);
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

// resolve names of tables, fields and variables
void resolve(Term*& a, unordered_map<string, Term*>& m);

void resolve(Term* a, int i, unordered_map<string, Term*>& m) {
	for (; i < a->v.size(); ++i)
		resolve(a->v[i], m);
}

void resolve(Term*& a, unordered_map<string, Term*>& m) {
	switch (a->tag) {
	case a_for: {
		auto mcopy = m;
		auto& m = mcopy;

		// sequence
		resolve(a->v[1], m);

		// variable
		auto y = a->v[0];
		m.insert_or_assign(y->s, y);

		// body
		resolve(a, 2, m);
		return;
	}
	case a_id:
		if (!m.count(a->s))
			a->err('\'' + a->s + "': not found");
		a = m.at(a->s);
		return;
	case a_js:
		return;
	case a_let: {
		// value
		resolve(a->v[1], m);

		// variable
		auto y = a->v[0];
		if (!m.insert({y->s, y}).second)
			a->err('\'' + y->s + "': already defined");
		return;
	}
	case a_select: {
		auto mcopy = m;
		auto& m = mcopy;

		// table
		for (auto field: a->v[0]->v)
			if (!m.count(field->s))
				m.insert({field->s, field});

		// where, fields
		resolve(a, 1, m);
		return;
	}
	}
	resolve(a, 0, m);
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
	case a_id:
		out(a->s);
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
	case a_select: {
		out("query(\"SELECT ");
		Separator separator;
		for (int i = 2; i < a->v.size(); ++i) {
			if (separator())
				out(',');
			expr(0, a->v[i]);
		}
		out(" FROM " + a->v[0]->s);
		if (a->v[1]->tag != a_literal) {
			out(" WHERE ");
			expr(0, a->v[1]);
		}
		out("\")");
		return;
	}
	case a_sub:
		infix(parent, a, "-");
		return;
	}
	a->err("cxx::expr: " + to_string(a->tag));
}

void stmt(Term* a) {
	switch (a->tag) {
	case a_for:
		out("for (auto " + a->v[0]->s + ':');
		expr(0, a->v[1]);
		out(") {\n");
		for (int i = 2; i < a->v.size(); ++i)
			stmt(a->v[i]);
		out("}\n");
		return;
	case a_let:
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

		unordered_map<string, Type*> types{
			// SORT
			{"bool", new Type(t_bool)},
			{"date", new Type(t_date)},
			{"decimal", new Type(t_decimal)},
			{"integer", new Type(t_integer)},
			{"text", new Type(t_text)},
		};

		for (auto table: tables) {
			auto t = new Term(a_table, table->name);
			t->type = new Type(t_table);
			for (auto field: table->fields) {
				auto f = new Term(a_field, field->name);
				f->type = types.at(field->type);
				t->v.push_back(f);
			}
			tableTerms.insert({table->name, t});
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
			unordered_map<string, Term*> m;
			resolve(a, m);

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
