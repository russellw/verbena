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

#include <filesystem>
using std::filesystem::path;

// terms
enum {
	// SORT
	a_add,
	a_and,
	a_assign,
	a_call,
	a_dot,
	a_eq,
	a_flip,
	a_function,
	a_id,
	a_le,
	a_let,
	a_list,
	a_lt,
	a_mul,
	a_ne,
	a_not,
	a_number,
	a_or,
	a_print,
	a_quote,
	a_select,
	a_sub,
	a_subscript,
	a_while,
	//
	end_a
};

struct Term {
	int tag;
	string s;
	vector<Term*> v;

	Term(int tag): tag(tag) {
	}

	Term(int tag, const string& s): tag(tag), s(s) {
	}

	Term(int tag, Term* a): tag(tag), v{a} {
	}

	Term(int tag, Term* a, Term* b): tag(tag), v{a, b} {
	}
};

// ============================================================================
Term* expr();

Term* primary() {
	switch (tok) {
	case k_number:
		return new Term(a_number, atom());
	case k_quote:
		return new Term(a_quote, atom());
	case k_word:
		return new Term(a_id, atom());
	}
	err("expected expression");
}

Term* postfix() {
	auto a = primary();
	for (;;)
		switch (tok) {
		case '(':
			lex();
			a = new Term(a_call, a);
			if (tok != ')')
				do
					a->v.push_back(expr());
				while (eat(','));
			expect(')');
			break;
		case '.':
			lex();
			a = new Term(a_dot, a, primary());
			break;
		case '[':
			lex();
			a = new Term(a_subscript, a, expr());
			expect(']');
			break;
		default:
			return a;
		}
}

Term* prefix() {
	switch (tok) {
	case '!':
		lex();
		return new Term(a_not, prefix());
	case '*':
		lex();
		return new Term(a_flip, prefix());
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
		op(k_ge, a_le);
		op(k_le, a_le);

		prec--;
		op(k_eq, a_eq);
		op(k_ne, a_ne);

		prec--;
		op(k_and, a_and);

		prec--;
		op(k_or, a_or);

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
		a = new Term(ops[k].tag, a, infix(prec1 + 1));
		switch (k) {
		case '>':
		case k_ge:
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
	string o;
	for (auto c: s) {
		if (c == '_')
			c = '-';
		o += c;
	}
	return o;
}

void pquote(vector<Term*>& o, string s) {
	o.push_back(new Term(a_print, new Term(a_quote, s)));
}

void stmt(vector<Term*>& o);
void stmts(vector<Term*>& o);

void attrs(vector<Term*>& o) {
	for (;;)
		switch (tok) {
		case '&': {
			// JavaScript attribute
			lex();
			pquote(o, ' ' + atom() + "=\"");
			auto a = new Term(a_flip);
			expect('{');
			while (!eat('}'))
				stmt(a->v);
			o.push_back(a);
			pquote(o, "\"");
			break;
		}
		case '@':
			lex();
			pquote(o, ' ' + atom());
			switch (tok) {
			case ';':
				// boolean attribute
				lex();
				break;
			case '{': {
				// compound attribute
				pquote(o, "=\"");
				lex();
				Separator separator;
				while (!eat('}')) {
					if (separator())
						pquote(o, ";");
					pquote(o, snakeCase(atom()) + '=');
					pquote(o, atom());
					expect(';');
				}
				pquote(o, "\"");
				break;
			}
			default:
				// simple attribute
				pquote(o, "=\"");
				auto a = new Term(a_print);
				a->v.push_back(expr());
				expect(';');
				o.push_back(a);
				pquote(o, "\"");
			}
			break;
		default:
			return;
		}
}

void stmt(vector<Term*>& o) {
	switch (tok) {
	case ';':
		lex();
		return;
	case k_quote:
		// a quoted string by itself is shorthand for a print statement
		pquote(o, atom());
		expect(';');
		return;
	}

	// SORT
	if (eat("function")) {
		// name
		auto f = new Term(a_function, atom());

		// parameters
		expect('(');
		auto params = new Term(a_list);
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
		pquote(o, '<' + tag);
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
			pquote(o, ">");
			return;
		}
		switch (tok) {
		case ';':
			lex();
			pquote(o, "/>");
			return;
		case '{':
			lex();
			attrs(o);
			pquote(o, ">");
			while (!eat('}'))
				stmt(o);
			break;
		default:
			pquote(o, ">");
			auto a = new Term(a_print, expr());
			expect(';');
			o.push_back(a);
		}
		pquote(o, "</" + tag + '>');
		return;
	}

	if (eat("print")) {
		auto a = new Term(a_print, expr());
		expect(';');
		o.push_back(a);
		return;
	}

	if (eat("script")) {
		expect('{');
		pquote(o, "<script>");
		auto a = new Term(a_flip);
		while (!eat('}'))
			stmt(a->v);
		o.push_back(a);
		pquote(o, "</script>");
		return;
	}

	if (eat("select")) {
		auto a = new Term(a_select, atom());
		expect('(');
		do
			a->v.push_back(expr());
		while (eat(','));
		expect(')');
		expect(';');
		o.push_back(a);
		return;
	}

	if (eat("while")) {
		expect('(');
		auto a = new Term(a_while, expr());
		expect(')');
		stmts(a->v);
		o.push_back(a);
		return;
	}

	// a statement can consist of an expression followed by a semicolon
	o.push_back(expr());
	expect(';');
}

void stmts(vector<Term*>& o) {
	if (eat('{')) {
		while (!eat('}'))
			stmt(o);
		return;
	}
	stmt(o);
}

// ============================================================================
namespace js {
char precs[end_a];
int prec = 99;

void op(int tag) {
	precs[tag] = prec;
}

struct Init {
	Init() {
		op(a_dot);

		prec--;
		op(a_not);

		prec--;
		op(a_mul);

		prec--;
		op(a_add);
		op(a_sub);

		prec--;
		op(a_le);
		op(a_lt);

		prec--;
		op(a_eq);
		op(a_ne);

		prec--;
		op(a_and);

		prec--;
		op(a_or);

		prec--;
		op(a_assign);
	}
} init;

// JavaScript cannot be generated straight to output file
// it needs to be composed in memory for further processing
// e.g. merging with other string literals as an optimization
vector<Term*> o;

void expr(Term* parent, Term* a);
void exprs(Term* a, int i);

void infix(Term* parent, Term* a, const char* op) {
	auto parens = parent && precs[parent->tag] >= precs[a->tag];
	if (parens)
		pquote(o, "(");
	expr(a, a->v[0]);
	pquote(o, op);
	expr(a, a->v[1]);
	if (parens)
		pquote(o, ")");
}

void expr(Term* parent, Term* a) {
	switch (a->tag) {
	case a_add:
		infix(parent, a, "+");
		return;
	case a_and:
		infix(parent, a, "&&");
		return;
	case a_assign:
		infix(parent, a, "=");
		return;
	case a_call:
		expr(0, a->v[0]);
		pquote(o, "(");
		for (int i = 1; i < a->v.size(); ++i) {
			if (i > 1)
				pquote(o, ",");
			expr(0, a->v[i]);
		}
		pquote(o, ")");
		return;
	case a_dot:
		expr(a, a->v[0]);
		pquote(o, '.' + a->v[1]->s);
		return;
	case a_flip:
		a->tag = a_print;
		o.push_back(a);
		return;
	case a_function: {
		pquote(o, "function " + a->s + '(');
		Separator separator;
		for (auto b: a->v[0]->v) {
			if (separator())
				pquote(o, ",");
			pquote(o, b->s);
		}
		pquote(o, "){");
		exprs(a, 1);
		pquote(o, "}");
		return;
	}
	case a_id:
		pquote(o, a->s);
		return;
	case a_le:
		infix(parent, a, "<=");
		return;
	case a_let:
		pquote(o, "let " + a->v[0]->s + '=');
		expr(a, a->v[0]);
		return;
	case a_lt:
		infix(parent, a, "<");
		return;
	case a_mul:
		infix(parent, a, "*");
		return;
	case a_not:
		pquote(o, "!");
		expr(a, a->v[0]);
		return;
	case a_number:
		pquote(o, a->s);
		return;
	case a_or:
		infix(parent, a, "||");
		return;
	case a_quote:
		pquote(o, esc(a->s));
		return;
	case a_sub:
		infix(parent, a, "-");
		return;
	case a_subscript:
		expr(0, a->v[0]);
		pquote(o, "[");
		expr(0, a->v[1]);
		pquote(o, "]");
		return;
	}
	throw runtime_error("js::expr: " + to_string(a->tag));
}

void exprs(Term* a, int i) {
	Separator separator;
	for (; i < a->v.size(); ++i) {
		if (separator())
			pquote(o, ";");
		expr(0, a->v[i]);
	}
}

void block(Term* a, int i) {
	if (a->v.size() - i == 1) {
		expr(0, a->v[i]);
		return;
	}
	pquote(o, "{");
	exprs(a, i);
	pquote(o, "}");
}

void compose(Term* a) {
	for (int i = 0; i < a->v.size();) {
		auto b = a->v[i];
		if (b->tag == a_flip) {
			o.clear();
			exprs(b, 0);
			a->v.erase(a->v.begin() + i);
			a->v.insert(a->v.begin() + i, o.begin(), o.end());
			i += o.size() - 1;
			continue;
		}
		compose(b);
		++i;
	}
}
} // namespace js

// ============================================================================
bool ispquote(Term* a) {
	return a->tag == a_print && a->v[0]->tag == a_quote;
}

void mergePrint(Term* a) {
	for (auto b: a->v)
		mergePrint(b);

	vector<Term*> v;
	for (auto b: a->v) {
		if (v.size() && ispquote(v.back()) && ispquote(b)) {
			v.back()->v[0]->s += b->v[0]->s;
			continue;
		}
		v.push_back(b);
	}
	a->v = v;
}

// ============================================================================
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
		op(a_le);
		op(a_lt);

		prec--;
		op(a_eq);
		op(a_ne);

		prec--;
		op(a_and);

		prec--;
		op(a_or);

		prec--;
		op(a_assign);
	}
} init;

void expr(Term* parent, Term* a);

void infix(Term* parent, Term* a, const char* op) {
	auto parens = parent && precs[parent->tag] >= precs[a->tag];
	if (parens)
		out("(");
	expr(a, a->v[0]);
	out(op);
	expr(a, a->v[1]);
	if (parens)
		out(")");
}

void expr(Term* parent, Term* a) {
	switch (a->tag) {
	case a_add:
		infix(parent, a, "+");
		return;
	case a_and:
		infix(parent, a, "&&");
		return;
	case a_assign:
		infix(parent, a, "=");
		return;
	case a_call:
		expr(0, a->v[0]);
		out("(");
		for (int i = 1; i < a->v.size(); ++i) {
			if (i > 1)
				out(",");
			expr(0, a->v[i]);
		}
		out(")");
		return;
	case a_id:
		out(a->s);
		return;
	case a_le:
		infix(parent, a, "<=");
		return;
	case a_lt:
		infix(parent, a, "<");
		return;
	case a_mul:
		infix(parent, a, "*");
		return;
	case a_not:
		out("!");
		expr(a, a->v[0]);
		return;
	case a_number:
		out(a->s);
		return;
	case a_or:
		infix(parent, a, "||");
		return;
	case a_quote:
		out(esc(a->s));
		return;
	case a_sub:
		infix(parent, a, "-");
		return;
	case a_subscript:
		expr(0, a->v[0]);
		out("[");
		expr(0, a->v[1]);
		out("]");
		return;
	}
	throw runtime_error("cxx::expr: " + to_string(a->tag));
}

void stmt(Term* a) {
	switch (a->tag) {
	case a_let:
		out("auto " + a->v[0]->s + '=');
		a = a->v[1];
		break;
	case a_list:
		for (auto b: a->v)
			stmt(b);
		return;
	case a_print:
		out("o +=");
		a = a->v[0];
		break;
	case a_select:
		out("select " + a->s + "(\"SELECT ");
		for (int i = 2; i < a->v.size(); ++i) {
			if (i > 2)
				out(",");
			expr(0, a->v[i]);
		}
		out(" FROM " + a->v[0]->s);
		if (a->v[1]->tag != a_number) {
			out(" WHERE ");
			expr(0, a->v[1]);
		}
		out("\");\n");
		return;
	case a_while:
		out("while (");
		expr(0, a->v[0]);
		out(") {\n");
		for (int i = 1; i < a->v.size(); ++i)
			stmt(a->v[i]);
		out("}\n");
		return;
	}
	expr(0, a);
	out(";\n");
}
} // namespace cxx

// ============================================================================
// SORT
string camelCase(const string& s) {
	string o;
	for (int i = 0; i < s.size();) {
		if (s[i] == '-') {
			o += toupper(s[i + 1]);
			i += 2;
			continue;
		}
		o += s[i++];
	}
	return o;
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
	string o;
	for (auto c: s) {
		if (c == '-')
			c = ' ';
		o += c;
	}
	o[0] = toupper(o[0]);
	return o;
}

// main
int main(int argc, char** argv) {
	try {
		if (argc < 2 || argv[1][0] == '-') {
			puts("compile-pages *-page.h\n"
				 "Writes pages.cxx");
			return 1;
		}

		// pages.cxx
		file = "pages.cxx";
		outf = xfopen("wb");
		out("// AUTO GENERATED - DO NOT EDIT\n");
		out("#include <main.h>\n");

		// pages
		vector<string> pages;
		for (int i = 1; i < argc; ++i) {
			file = argv[i];
			auto stem = path(file).stem().string();
			auto name = camelCase(stem);
			pages.push_back(name);

			// predefined header
			auto a = new Term(a_list);
			pquote(a->v, "<!DOCTYPE html>");
			pquote(a->v, "<html lang=\"en\">");
			pquote(a->v, "<head>");
			pquote(a->v, "<title>");
			auto title = stem;
			if (endsWith(title, "-page"))
				title = title.substr(0, title.size() - 5);
			pquote(a->v, titleCase(title));
			pquote(a->v, "</title>");
			pquote(a->v, "<style>");
			pquote(a->v, "body{");
			pquote(a->v, "display:flex;");
			pquote(a->v, "font-family:Arial,sans-serif;");
			pquote(a->v, "font-size:20px;");
			pquote(a->v, "}");
			pquote(a->v, "table{");
			pquote(a->v, "border-collapse:collapse;");
			pquote(a->v, "width:100%;");
			pquote(a->v, "}");
			pquote(a->v, "th,td{");
			pquote(a->v, "border:1px solid #d3d3d3;");
			pquote(a->v, "padding:8px;");
			pquote(a->v, "text-align:left;");
			pquote(a->v, "}");
			pquote(a->v, "th{");
			pquote(a->v, "background-color:#f2f2f2;");
			pquote(a->v, "}");
			pquote(a->v, "</style>");
			pquote(a->v, "</head>");
			pquote(a->v, "<body>");

			// source file
			preprocess();
			while (tok)
				stmt(a->v);

			// JavaScript
			js::compose(a);

			// optimize
			mergePrint(a);

			// page generator function
			out("void " + name + "(string& o) {\n");
			cxx::stmt(a);
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
