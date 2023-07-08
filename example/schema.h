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

// SORT
table countries {
	field code {
		type = char(2);
		key;
	}
	field name {
	}
}

table customers {
	field customerNo {
		type = integer;
		generated;
		key;
	}
	field name {
	}
}

table estimateLines {
	field estimate {
		ref = estimates;
	}
	field line {
		type = smallint;
	}
	field product {
		ref = products;
	}
	field description {
	}
	field qty {
		type = decimal;
	}
	field price {
		type = decimal;
	}
}

table estimates {
	field estimateNo {
		type = bigint;
		generated;
		key;
	}
	field customer {
		ref = customers;
	}
	field date {
		type = date;
	}
	field expires {
		type = date;
	}
}

table products {
	field code {
		key;
	}
	field description {
	}
	field cost {
		type = decimal;
	}
	field price {
		type = decimal;
	}
}
