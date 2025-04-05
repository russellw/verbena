cargo fmt||exit /b
call js-beautify --end-with-newline -r src\prefix.js
git diff
