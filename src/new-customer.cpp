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

<form method="post" style = "display:flex; flex-direction:row; align-items: flex-start; margin-top:15px">
    <div style="display:grid; gap:10px; grid-template-columns:max-content max-content;">
      <label for="name">Name *</label>
      <input  id="name" name="name" required>

      <label  for="email">Email</label>
      <input  type="email" id="email" name="email">

      <label  for="phone">Phone</label>
      <input  type="tel" id="phone" name="phone">
    </div>

    <div style="display:grid; column-gap:10px; grid-template-columns:max-content max-content; margin-left:15px">
      <label  for="deliver_name">Deliver to *</label>
      <input  id="deliver_name" name="deliver_name" required>

      <label  style="margin-top: 10px;" for="deliver_address_1">Address</label>
      <input  style="margin-top: 10px;" id="deliver_address_1" name="deliver_address_1" >

      <label style="margin-top: 5px;" for="deliver_address_2"></label>
      <input style="margin-top: 5px;" id="deliver_address_2" name="deliver_address_2" >

      <label style="margin-top: 10px;"  for="deliver_city">City</label>
      <input style="margin-top: 10px;" id="deliver_city" name="deliver_city" >

      <label style="margin-top: 10px;"  for="deliver_region">Region</label>
      <input style="margin-top: 10px;" id="deliver_region" name="deliver_region" >

      <label style="margin-top: 10px;"  for="deliver_postal_code">Postal code</label>
      <input style="margin-top: 10px;" id="deliver_postal_code" name="deliver_postal_code" >

      <label style="margin-top: 10px;"  for="deliver_country">Country *</label>
      <input style="margin-top: 10px;" id="deliver_country" name="deliver_country" >

#include "save.html"
    </div>
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
