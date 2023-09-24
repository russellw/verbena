<!--
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
-->

?id

#include "head.html"
  <title>Customer @(id)</title>

#include "sidebar.html"

@{
    auto S=prep(${
        select name, email, phone, delivery_address,billing_address from customer where id=$1
    });
	bind(S, 1, id);
    if(step(S)){
        @{
         <div  class="display-form">
          <label>Customer</label>
          <span  >@(id)</span>

          <label>Name</label>
          <span  >@get(S,0)</span>

          <label>Email</label>
          <span  >@get(S,1)</span>

          <label>Phone</label>
          <span  >@get(S,2)</span>

          <label>Delivery address</label>
          <span  >@{
            appendHtml(get(S,3),o);
            }</span>

          <label>Billing address</label>
          <span  >@{
            appendHtml(get(S,4),o);
            }</span>
         </div>
        }
		sqlite3_finalize(S);
    }
    else
    {
    }
}
