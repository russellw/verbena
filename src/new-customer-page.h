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

#include "sidebar.h"

form {
	label {
		'Name';
		input {
			@autofocus;
			@name name;
		}
	}
	label {
		'Email';
		input {
			@type email;
			@name email;
		}
	}
	label {
		'Phone';
		input {
			@type tel;
			@name phone;
		}
	}
	label {
		'Delivery address';
		textarea {
			@name delivery_address;
		}
	}
	label {
		'Billing address';
		textarea {
			@name billing_address;
		}
	}
	button {
		@type submit;
		'Save';
	}
}
