#include <assert.h>
#include <ctype.h>
#include <errno.h>
#include <limits.h>
#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <libpq-fe.h>

#ifdef _WIN32
#include <winsock2.h>
#include <ws2tcpip.h>
#endif

#include <algorithm>
#include <exception>
#include <iostream>
#include <stdexcept>
#include <string>
#include <vector>
using namespace std;

#include "db.h"
#include "etc.h"

#include <compiled-http.hxx>

// pages.cxx
void dispatch(char* s, string& o);
void dispatchPOST(char* s);
void dispatchPUT(char* s);
