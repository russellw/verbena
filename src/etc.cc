#include "all.h"

char* body(char* s) {
	s = strstr(s, "\r\n\r\n");
	if (!s)
		throw runtime_error("HTTP request has no body");
	return s + strlen("\r\n\r\n");
}

static char* unesc(char* s) {
	// if the value string contains an escaped character
	// it needs to be rewritten with the escape resolved
	// but the result will be smaller than the original string
	// so the rewrite can be done in place
	auto r = s;
	while (*s != '"') {
		auto c = *s++;
		switch (c) {
		case '\\':
			c = *s++;
			switch (c) {
			case 'n':
				c = '\n';
				break;
			case '"':
			case '\\':
				break;
			default:
				throw runtime_error("Unknown JSON escape sequence");
			}
			break;
		case 0:
			throw runtime_error("Unclosed JSON quote");
		}
		*r++ = c;
	}

	// null-terminate the value for the benefit of the database driver
	*r = 0;

	// need to know where it ended, to find the next field
	return s;
}

void jsonParse(char*& s, string& sql, vector<char*>& vals) {
	// SQL wants the list of field names to be comma-separated
	// but we don't know in advance how many fields the user will supply
	// so just terminate each field name with a comma now
	// the trailing comma will be removed later
	sql += ',';

	// the current value is the string beginning at this position
	auto t = s;
	vals.push_back(t);

	// find the end
	// fast-track the common case where there are no escaped characters
	// so the value string can be kept as is
	while (*t != '"') {
		if (*t == '\\' || !*t) {
			t = unesc(s);
			break;
		}
		++t;
	}

	// null-terminate the value for the benefit of the database driver
	*t = 0;

	// and move to the next field name
	s = t + strlen("\",\"");
}

void appendHtml(const char* s, string& o) {
	for (;;) {
		auto c = *s++;
		switch (c) {
		case '\n':
			o += "<br>";
			break;
		case 0:
			return;
		default:
			o += c;
		}
	}
}
