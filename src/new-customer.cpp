@{
#include "head.html"
     <style>
        #include "styles.css"
    </style>
<title>New customer</title>
#include "menu.html"

<form method="post" style = "display:flex; flex-direction:row; align-items: flex-start; margin:15px">
    <div style="display:grid; gap:10px; grid-template-columns:max-content max-content;">
      <label for="name">Name *</label>
      <input  id="name" name="name" required>

      <label  for="email">Email</label>
      <input  type="email" id="email" name="email">

      <label  for="phone">Phone</label>
      <input  type="tel" id="phone" name="phone">
    </div>

    <div style="display:grid; gap:10px; grid-template-columns:max-content max-content; margin-left:15px">
      <label  for="bill_name">Bill to *</label>
      <input  id="bill_name" name="bill_name" required>

      <label   for="bill_address_1">Address</label>
      <input   id="bill_address_1" name="bill_address_1" >

      <label  for="bill_address_2"></label>
      <input  id="bill_address_2" name="bill_address_2" >

      <label   for="bill_city">City</label>
      <input  id="bill_city" name="bill_city" >

      <label   for="bill_region">Region</label>
      <input  id="bill_region" name="bill_region" >

      <label   for="bill_postal_code">Postal code</label>
      <input  id="bill_postal_code" name="bill_postal_code" >

      <label   for="bill_country">Country *</label>
      <input  id="bill_country" name="bill_country" required>
    </div>

    <div style="display:grid; gap:10px; grid-template-columns:max-content max-content; margin-left:15px">
      <label  for="deliver_name">Deliver to *</label>
      <input  id="deliver_name" name="deliver_name" required>

      <label   for="deliver_address_1">Address</label>
      <input   id="deliver_address_1" name="deliver_address_1" >

      <label  for="deliver_address_2"></label>
      <input  id="deliver_address_2" name="deliver_address_2" >

      <label   for="deliver_city">City</label>
      <input  id="deliver_city" name="deliver_city" >

      <label   for="deliver_region">Region</label>
      <input  id="deliver_region" name="deliver_region" >

      <label   for="deliver_postal_code">Postal code</label>
      <input  id="deliver_postal_code" name="deliver_postal_code" >

      <label   for="deliver_country">Country *</label>
      <input  id="deliver_country" name="deliver_country" required>

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
