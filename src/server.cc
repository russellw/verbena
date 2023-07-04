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

#include "olivine.h"
#include <winsock2.h>
#include <ws2tcpip.h>

#pragma comment(lib, "ws2_32.lib")

namespace olivine {
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
} // namespace

void server() {
	WSADATA wsaData;
	check("WSAStartup", WSAStartup(MAKEWORD(2, 2), &wsaData));

	addrinfo hints{0};
	addrinfo* r;
	check("getaddrinfo", getaddrinfo(0, "80", 0, &r));

	auto listenSocket = socket(r->ai_family, r->ai_socktype, r->ai_protocol);
	if (listenSocket == INVALID_SOCKET)
		err("socket");
	if (bind(listenSocket, r->ai_addr, r->ai_addrlen) == SOCKET_ERROR)
		err("bind");
	if (listen(listenSocket, SOMAXCONN) == SOCKET_ERROR)
		err("listen");

	auto clientSocket = accept(listenSocket, 0, 0);
	if (clientSocket == INVALID_SOCKET)
		err("listen");

	static char buf[999];
	auto n = recv(clientSocket, buf, sizeof buf, 0);
	if (n < 0)
		err("recv");
	print(buf);

	strcpy(buf, "HTTP/1.1 200 OK\r\nContent-Length: 13\r\n\r\nHello, World!");
	if (send(clientSocket, buf, strlen(buf), 0) == SOCKET_ERROR)
		err("send");
}
} // namespace olivine
