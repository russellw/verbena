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

@{
#include "head.html"
  <title>Customers</title>

#include "menu.html"

<div SIDEBARRED_STYLE>
<table id="table" TABLE_STYLE>
    <tr>
      <th>#
      <th>Name
      <th>Email
      <th>Phone

    @{
        auto S=prep(@(
            select id, name, email, phone from customer
        ));
		while (step(S)) @{
            <tr class="link-row"  data-id="@get(S,0)">
              <td>@get(S,0)
              <td>@get(S,1)
              <td>@get(S,2)
              <td>@get(S,3)
        }
    }
</table>
<div SIDEBAR_STYLE>
<a href="new-customer">New customer</a>
</div >
</div>

<script>
document.getElementById('table').addEventListener('click', function(event) {
    const r = event.target.closest('tr[data-id]');
    if (r)
		window.location.href = 'customer?id=' + r.getAttribute('data-id');
});
</script>
}