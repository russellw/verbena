cargo fmt||exit /b
call prettier --no-semi --print-width 132 -w src
git diff
