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
     <style>
        #include "styles.css"
    </style>
<title>New customer</title>
#include "menu.html"

<form method="post"  FORM_STYLE>
      <label for="name">Name</label>
      <input  id="name" name="name" >

      <label for="email">Email</label>
      <input type="email" id="email" name="email">

      <label for="phone">Phone</label>
      <input type="tel" id="phone" name="phone">

      <label for="delivery_address">Delivery address</label>
      <textarea rows="4" id="delivery_address" name="delivery_address"></textarea>

      <label for="billing_address">Billing address</label>
      <textarea rows="4" id="billing_address" name="billing_address"></textarea>

#include "save.html"
</form>

<script>
#include "post.js"
</script>
}
@POST
s=body(s)+strlen("{\"");
string sql = "INSERT INTO customer(";
vector<char*>vals;
JSON_FIELD("name");
JSON_FIELD("email");
JSON_FIELD("phone");
JSON_FIELD("delivery_address");
JSON_FIELD("billing_address");
execInsert(sql,vals);
