void initdb(const char* info);
void execInsert(string& sql, const vector<char*>& vals);

class Query {
	PGresult* r;
	int n;

public:
	Query(const char* sql);
	Query(const char* sql, const char* val1);

	~Query() {
		PQclear(r);
	}

	bool empty() {
		return !n;
	}

	struct Iterator {
		int i;

		Iterator(int i): i(i) {
		}

		int operator*() {
			return i;
		}

		Iterator& operator++() {
			++i;
			return *this;
		}

		bool operator!=(Iterator b) const {
			return i != b.i;
		}
	};

	Iterator begin() {
		return Iterator(0);
	}

	Iterator end() {
		return Iterator(n);
	}

	char* operator()(int i, int j) {
		return PQgetvalue(r, i, j);
	}
};
