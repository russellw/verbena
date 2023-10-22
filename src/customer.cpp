?id

Query q(@(
    select name, email, phone, delivery_address,billing_address from customer where id=$1
),id);

if(q.empty()){
    @{
    #include "head.html"
     <style>
        #include "styles.css"
    </style>
    <title>Not found</title>
    #include "menu.html"
    <div  NOT_FOUND_STYLE>
        Customer @id not found.
    </div>
    }
    return;
}

auto name=q(0,0);
@{
#include "head.html"
     <style>
        #include "styles.css"
    </style>
<title>@name</title>
#include "menu.html"

<div SIDEBARRED_STYLE>
<div SIDEBAR_STYLE>
<a href="outstanding-orders">Outstanding orders</a>
</div >

 <div FORM_STYLE>
  <label>Customer</label>
  <span  >@id</span>

  <label>Name</label>
  <span  >@name</span>

  <label>Email</label>
  <span  >@q(0,1)</span>

  <label>Phone</label>
  <span  >@q(0,2)</span>

  <label>Delivery address</label>
  <span  >@{
    appendHtml(q(0,3),o);
    }</span>

  <label>Billing address</label>
  <span  >@{
    appendHtml(q(0,4),o);
    }</span>
 </div>
</div>
}
