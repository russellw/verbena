let error=null
document.addEventListener("DOMContentLoaded", function() {
    const form = document.querySelector("form");
    form.addEventListener("submit", async function (event) {
        try {
          event.preventDefault();
          const data = {}
          for(const element of form.querySelectorAll("input,textarea"))
            if(element.value)
              data[element.name]=element.value
          if(!Object.keys(data).length)return;

          const response = await fetch(window.location.href, {
            method: "POST",
            body:JSON.stringify(data)
          });
          if (!response.ok)
            throw new Error(await response.text());

          for(const element of form.querySelectorAll("input,textarea"))
            element.value = "";
        } catch (e) {
            if(error)error.remove()
            error = document.createElement("div");
            error.textContent = e;

            error.style.padding='16px'
            error.style.color='#b00'
            error.style.position='fixed'
            error.style.width='100%'
            error.style.textAlign='center'
            error.style.bottom='0'

            document.body.appendChild(error);
        }
    });
});
