for %%x in (src\*.rs) do rustfmt %%x||exit /b
for %%x in (src\bin\*.rs) do rustfmt %%x||exit /b
git diff
