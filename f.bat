for /r %%x in (*.rs) do rustfmt %%x||exit /b
git diff
