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

#include "all.h"

#ifdef _WIN32
#pragma comment(lib, "ws2_32.lib")
#endif

namespace {
void err(const char* s, int e) {
	char* t;
	FormatMessage(
		FORMAT_MESSAGE_ALLOCATE_BUFFER | FORMAT_MESSAGE_FROM_SYSTEM | FORMAT_MESSAGE_IGNORE_INSERTS, 0, e, 0, (LPTSTR)&t, 0, 0);
	throw runtime_error(string(s) + ": " + t);
}

void err(const char* s) {
	err(s, WSAGetLastError());
}

void check(const char* s, int e) {
	if (e)
		err(s, e);
}

void send1(SOCKET socket, const void* s, int n) {
	if (send(socket, (const char*)s, n, 0) == SOCKET_ERROR)
		err("send");
}

void send1(SOCKET socket, const char* s) {
	send1(socket, s, strlen(s));
}
} // namespace

int main(int argc, char** argv) {
	try {
		// set up socket
		WSADATA wsaData;
		check("WSAStartup", WSAStartup(MAKEWORD(2, 2), &wsaData));

		addrinfo hints{0};
		addrinfo* ai;
		check("getaddrinfo", getaddrinfo(0, "80", 0, &ai));

		auto listenSocket = socket(ai->ai_family, ai->ai_socktype, ai->ai_protocol);
		if (listenSocket == INVALID_SOCKET)
			err("socket");
		if (bind(listenSocket, ai->ai_addr, ai->ai_addrlen) == SOCKET_ERROR)
			err("bind");
		if (listen(listenSocket, SOMAXCONN) == SOCKET_ERROR)
			err("listen");

		static char buf[10000];
		buf[sizeof buf - 2] = ' ';
		for (;;) {
			// accept connection
			auto clientSocket = accept(listenSocket, 0, 0);
			if (clientSocket == INVALID_SOCKET)
				err("listen");

			// receive request
			auto n = recv(clientSocket, buf, sizeof buf - 2, 0);
			if (n < 0)
				err("recv");
			buf[n] = 0;
			debug(0);
			cout << buf;

			// respond
			try {
				switch (buf[1]) {
				case 'E': {
					// GET /
					auto s = buf + 5;

					// files that need their own distinct Content-Type are handled separately
					if (eq(s, "favicon.ico")) {
						send1(clientSocket, faviconData, sizeof faviconData);
						continue;
					}
					if (eq(s, "styles.css")) {
						send1(clientSocket, stylesData, sizeof stylesData);
						continue;
					}

					// HTTP header
					auto header = "HTTP/1.1 200\r\nContent-Length:      \r\n\r\n";
					auto headerLen = strlen(header);

					// content
					string o = header;
					dispatch(s, o);

					// fill in Content-Length
					auto contentLen = to_string(o.size() - headerLen);
					auto i = headerLen - 4 - contentLen.size();
					assert(o[i] == ' ');
					memcpy(o.data() + i, contentLen.data(), contentLen.size());

					// send response
					send1(clientSocket, o.data(), o.size());

					// dump HTML for validation
					auto f = fopen("/t/a.html", "wb");
					if (f) {
						fwrite(o.data() + headerLen, 1, o.size() - headerLen, f);
						fclose(f);
					}
					break;
				}
				case 'O':
					// POST /
					dispatchPOST(buf + 6);
					send1(clientSocket, "HTTP/1.1 200");
					break;
				default:
					// PUT /
					dispatchPUT(buf + 5);
					send1(clientSocket, "HTTP/1.1 200");
				}
			} catch (exception& e) {
				send1(clientSocket, "HTTP/1.1 500");
			}

			// done with this client for now
			closesocket(clientSocket);
		}
	} catch (exception& e) {
		cout << e.what() << '\n';
		return 1;
	}
}
