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

// C headers
#include <assert.h>
#include <errno.h>
#include <limits.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// C++ headers
#include <array>
using std::array;

#include <exception>
using std::exception;

#include <stdexcept>
using std::runtime_error;

#include <string>
using std::string;
using std::to_string;

#include <vector>
using std::vector;

// library headers
#include "../sqlite/sqlite3.h"

// own headers
#include "db.h"
#include "etc.h"

// generated headers
#include <country.hxx>
#include <favicon.hxx>
#include <schema.hxx>
void dispatch(const char* req, string& o);
