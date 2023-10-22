@{
#include "head.html"
     <style>
        #include "styles.css"
        #include "table-styles.css"
    </style>
  <title>Customers</title>

#include "menu.html"

<a href="new-customer" BLOCK_PADDING_STYLE>New customer</a>
<table  >
    <tr>
      <th>#
      <th>Name
      <th>Email
      <th>Phone

    @{
        Query q(@(
            select id, name, email, phone from customer
        ));
		for(auto i:q){
		    auto id=q(i,0);
		     @{
            <tr  data-id="@id">
              <td>@id
              <td>@q(i,1)
              <td>@q(i,2)
              <td>@q(i,3)
            }
        }
    }
</table>

<script>
document.querySelector('table').addEventListener('click', function(event) {
    const r = event.target.closest('tr[data-id]');
    if (r)
		window.location.href = 'customer?id=' + r.getAttribute('data-id');
});
</script>
}
