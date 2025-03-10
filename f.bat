for %%x in (src\*) do rustfmt %%x||exit /b
git diff
