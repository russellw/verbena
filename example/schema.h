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

// schema file syntax uses the .h extension
// but is not meant for consumption by the C++ compiler
// it should be processed by compile-schema

// SORT
table countries {
	code string(2);
	name string;
}

table customers {
	number integer;
	name string;
}

table estimateLines {
	number bigint;
	estimate estimates;
	product products;
	description string;
	qty decimal;
	price decimal;
}

table estimates {
	number bigint;
	customer customers;
	date date;
	expires date;
}

table products {
	partNumber string;
	description string;
	cost decimal;
	price decimal;
}
