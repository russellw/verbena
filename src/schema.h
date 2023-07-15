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
table country {
	field code {
		key;
		size = 2;
	}
	field name {
		nonull;
	}
}

table customer {
	field no {
		type = integer;
		key;
	}
	field name {
		nonull;
	}
	field email {
	}
	field phone {
	}
	field deliveryAddress {
	}
	field billingAddress {
	}
}

table estimate {
	field no {
		type = integer;
		key;
	}
	field customer {
		nonull;
		ref;
	}
	field date {
		nonull;
		type = date;
	}
	field expires {
		type = date;
	}
}

table estimateLine {
	field estimate {
		nonull;
		ref;
	}
	field line {
		nonull;
		type = integer;
	}
	field product {
		ref;
	}
	field description {
	}
	field qty {
		type = decimal;
		scale = 3;
	}
	field price {
		type = decimal;
	}
}

table product {
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
