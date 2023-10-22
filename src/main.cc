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

void sendContentLen(int headerLen, string& o, SOCKET socket) {
	// fill in Content-Length
	auto contentLen = to_string(o.size() - headerLen);
	auto i = headerLen - strlen("\r\n\r\n") - contentLen.size();
	assert(o[i] == ' ');
	memcpy(o.data() + i, contentLen.data(), contentLen.size());

	// send response
	send1(socket, o.data(), o.size());
}
} // namespace

int main(int argc, char** argv) {
	try {
		initdb("dbname=verbena user=postgres password=a");

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
		for (;;) {
			// accept connection
			auto clientSocket = accept(listenSocket, 0, 0);
			if (clientSocket == INVALID_SOCKET)
				err("listen");

			// receive request
			// leave enough unused buffer that optimized memcmp is safe
			auto n = recv(clientSocket, buf, sizeof buf - 8, 0);
			if (n < 0)
				err("recv");

			// null terminate the request, but also pad with extra zeros
			// so it's safe for parsing code to look a byte or two ahead
			*(int32_t*)(buf + n) = 0;
			cout << buf << '\n';

			// respond
			try {
				switch (buf[1]) {
				case 'E': {
					// GET /
					auto s = buf + 5;

					// files that need their own distinct Content-Type are handled separately
					if (eq(s, "favicon.")) {
						send1(clientSocket, faviconData, sizeof faviconData);
						break;
					}

					// HTTP header
					auto header = "HTTP/1.1 200\r\n"
								  "Content-Type:text/html;charset=utf-8\r\n"
								  "Content-Length:      \r\n\r\n";
					auto headerLen = strlen(header);

					// content
					string o = header;
					dispatch(s, o);

					// send response
					sendContentLen(headerLen, o, clientSocket);

#ifndef NDEBUG
					// dump HTML for validation
					auto f = fopen("/t/a.html", "wb");
					if (f) {
						fwrite(o.data() + headerLen, 1, o.size() - headerLen, f);
						fclose(f);
					}
#endif
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
				// HTTP header
				auto header = "HTTP/1.1 500\r\n"
							  "Content-Type:text/plain;charset=utf-8\r\n"
							  "Content-Length:      \r\n\r\n";
				auto headerLen = strlen(header);

				// content
				string o = header;
				o += e.what();

				// send response
				sendContentLen(headerLen, o, clientSocket);
			}

			// done with this client for now
			closesocket(clientSocket);
		}
	} catch (exception& e) {
		cerr << e.what() << '\n';
		return 1;
	}
}
