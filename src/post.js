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

document.addEventListener("DOMContentLoaded", function() {
    const form = document.getElementById("form");
    form.addEventListener("submit", async function (event) {
        try {
          event.preventDefault();
          const data = {}
          for(var element of form.querySelectorAll("input,textarea"))
            if(element.value)
              data[element.name]=element.value
          if(!Object.keys(data).length)return;

          const response = await fetch(window.location.href, {
            method: "POST",
            body:JSON.stringify(data)
          });
          if (!response.ok)
            throw new Error(await response.text());

          //window.location.href = 'customers';
        } catch (e) {
            const error = document.createElement("div");
            error.classList.add("error");
            error.textContent = e;
            document.body.appendChild(error);
            console.error("error ", e);
        }
    });
});