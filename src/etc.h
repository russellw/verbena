#ifdef NDEBUG
#define debug(a)
#else
#define debug(a) cout << __FILE__ << ':' << __LINE__ << ": " << __func__ << ": " << #a << ": " << a << '\n'
#endif

struct Separator {
	bool subsequent = 0;

	bool operator()() {
		auto a = subsequent;
		subsequent = 1;
		return a;
	}
};

char* body(char* s);

inline bool eq(const char* s, const char* t) {
	return memcmp(s, t, strlen(t)) == 0;
}

void jsonParse(char*& s, string& sql, vector<char*>& vals);

inline void jsonField(const char* name, const char* name1, char*& s, string& sql, vector<char*>& vals) {
	// include the close quote in the field name check
	// because one field name might be a substring of another
	if (eq(s, name1)) {
		s += strlen(name1) + strlen("=\"");
		sql.append(name, strlen(name));
		jsonParse(s, sql, vals);
	}
}

#define JSON_FIELD(name) jsonField(name, name##"\"", s, sql, vals)

void appendHtml(const char* s, string& o);
