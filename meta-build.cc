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

#include <assert.h>
#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <exception>
#include <filesystem>
#include <stdexcept>
#include <string>
#include <unordered_set>
#include <vector>
using std::exception;
using std::runtime_error;
using std::string;
using std::unordered_set;
using std::vector;
using namespace std::filesystem;

#include <fcntl.h>
#include <sys/stat.h>

#ifdef _WIN32
#include <io.h>
#else
#include <unistd.h>
#define O_BINARY 0
#endif

inline void print(const char* s) {
	fwrite(s, 1, strlen(s), stdout);
}

inline void print(const string& s) {
	fwrite(s.data(), 1, s.size(), stdout);
}

inline void println(const char* s) {
	auto n = strlen(s);
	fwrite(s, 1, n, stdout);
	if (n && s[n - 1] != '\n')
		putchar('\n');
}

template <class T> void println(const T& a) {
	print(a);
	putchar('\n');
}

void writeFile(const string& file, const string& s) {
	auto f = open(file.data(), O_CREAT | O_WRONLY | O_TRUNC | O_BINARY, 0644);
	if (f < 0)
		throw runtime_error(file + ": " + strerror(errno));
	write(f, s.data(), s.size());
	close(f);
}

vector<path> places;

path find(string file) {
	print(file + ": ");
	for (auto& dir: places)
		for (auto& e: recursive_directory_iterator(dir, directory_options::skip_permission_denied)) {
			auto p = e.path();
			if (p.filename() == file) {
				println(p.string());
				return p.parent_path();
			}
		}
	throw runtime_error("not found");
}

string esc(path p) {
	string s;
	for (auto c: p.string()) {
		if (c == ':')
			s += '$';
		s += c;
	}
	return s;
}

string quoteSpace(string s) {
	if (s.find(' ') < s.size())
		return '"' + s + '"';
	return s;
}

string incl(path p) {
	return quoteSpace("-I" + esc(p));
}

// the current output
string o;

void rule(string name, string command) {
	o += "rule " + name + '\n';
	o += "  command=" + command + '\n';
}

int main(int argc, char** argv) {
	try {
		// places to look for database driver
		for (int i = 1; i < argc; ++i) {
			auto s = argv[i];
			if (*s == '-') {
				puts("meta-build [place...]");
				return 0;
			}
			places.push_back(s);
		}
		if (places.empty())
			places.push_back("C:\\Program Files");
		puts("Places to look for database driver:");
		for (auto dir: places)
			println(dir.string());

		// for simplicity, this build process assumes source filenames are unique
		// so verify that they are
		auto cd = current_path();
		unordered_set<string> sourceFiles;
		for (auto dir: {"src", "tools", "example"})
			for (auto& e: directory_iterator(cd / dir)) {
				auto p = e.path();
				if (p.extension() == ".cc" && !sourceFiles.insert(p.filename().string()).second)
					throw runtime_error(p.string() + ": duplicate filename");
			}

		// Postgres
		auto postgresIncl = find("libpq-fe.h");

		// compiler options
		string cc = "cl -EHsc -MDd -W2 -nologo -showIncludes -std:c++20";
		cc += ' ' + incl(postgresIncl);

		// compose build.ninja
		o = "# AUTO GENERATED - DO NOT EDIT\n";
		o += "msvc_deps_prefix=Note: including file:\n";
		o += "rule cc\n";
		o += "  deps=msvc\n";
		o += "  command=" + cc + " $ipath $in -c\n";

		// ==========================================================
		o += "\n# FRAMEWORK\n";

		o += "# object files\n";
		string objs;
		for (auto& e: directory_iterator(cd / "src")) {
			auto p = e.path();
			if (p.extension() == ".cc") {
				auto obj = p.filename().replace_extension(".obj").string();
				o += "build " + obj + ": cc " + esc(p) + '\n';
				objs += ' ' + obj;
			}
		}

		o += "# static library\n";
		rule("lib", "lib -nologo $in -out$:$out");
		o += "build verbena.lib: lib" + objs + '\n';

		// ==========================================================
		o += "\n# TOOLS\n";

		rule("cc-link", cc + ' ' + incl(cd / "src") + " $in setargv.obj -Fe$out");
		for (auto& e: directory_iterator(cd / "tools")) {
			auto p = e.path();
			if (p.extension() == ".cc")
				o += "build " + p.filename().replace_extension(".exe").string() + ": cc-link " + esc(p) + " verbena.lib\n";
		}

		// ==========================================================
		o += "\n# EXAMPLE\n";
		o += "# generated C++ files don't need current directory in the include path, because they are in it\n";

		o += "# schema\n";
		rule("schema", "compile-schema $in");

		o += "build schema.hxx schema.cxx: schema " + esc(cd / "example" / "schema.h") + "|compile-schema.exe\n";

		o += "build schema.obj: cc schema.cxx\n";
		o += "  ipath=" + incl(cd / "src") + '\n';
		objs = "schema.obj";

		o += "# static data files\n";
		rule("bytes", "compile-bytes $in");
		rule("csv", "compile-csv $in");
		for (auto& e: directory_iterator(cd / "example")) {
			auto p = e.path();

			auto hxx = p.filename().replace_extension(".hxx").string();
			auto cxx = p.filename().replace_extension(".cxx").string();
			auto obj = p.filename().replace_extension(".obj").string();

			if (p.extension() == ".png")
				o += "build " + hxx + ' ' + cxx + ": bytes " + esc(p) + "|compile-bytes.exe\n";
			else if (p.extension() == ".csv")
				o += "build " + hxx + ' ' + cxx + ": csv " + esc(p) + "|compile-csv.exe\n";
			else
				continue;

			o += "build " + obj + ": cc " + cxx + '\n';
			objs += ' ' + obj;
		}

		o += "# object files\n";
		o += "# source C++ files need current directory in the include path, to pick up generated header files\n";
		for (auto& e: directory_iterator(cd / "example")) {
			auto p = e.path();
			if (p.extension() == ".cc") {
				auto obj = p.filename().replace_extension(".obj").string();
				o += "build " + obj + ": cc " + esc(p) + '\n';
				o += "  ipath=" + incl(cd / "src") + " -I.\n";
				objs += ' ' + obj;
			}
		}

		o += "# program\n";
		string command = "link -noexp -noimplib -nologo $in";
		command += ' ' + quoteSpace(esc(postgresIncl.parent_path() / "lib" / "libpq.lib"));
		command += " /out$:$out";
		rule("link", command);

		o += "build example.exe: link " + objs + " verbena.lib\n";

		// ----------------------------------------------------------
		create_directory("bin");
		writeFile("bin/build.ninja", o);
		return 0;
	} catch (exception& e) {
		println(e.what());
		return 1;
	}
}
