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

html form {
	html label {
		"Name";
		html input {
			@autofocus;
			@name name;
		}
	}
	html label {
		"Email";
		html input {
			@type email;
			@name email;
		}
	}
	html label {
		"Phone";
		html input {
			@type tel;
			@name phone;
		}
	}
	html label {
		"Delivery address";
		html textarea {
			@name deliveryAddress;
		}
	}
	html label {
		"Billing address";
		html textarea {
			@name billingAddress;
		}
	}
	html button {
		@type submit;
		"Save";
	}
}
