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

// SORT
country {
	id key;
	name nonull;
}

customer {
	id integer key;
	name nonull;
	email;
	phone;
	delivery_address;
	billing_address;
}

estimate {
	id integer key;
	customer nonull ref;
	date date nonull;
	expires date;
}

estimate_detail {
	estimate nonull ref;
	line integer nonull;
	product ref;
	description;
	qty decimal(0, 3);
	price decimal;
}

product {
	id key;
	description;
	cost decimal;
	price decimal;
}
