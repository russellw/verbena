cargo fmt||exit /b
call js-beautify -n -t -r src\prefix.js
git diff
