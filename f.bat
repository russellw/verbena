for %%x in (src\*.rs) do rustfmt %%x||exit /b
git diff
