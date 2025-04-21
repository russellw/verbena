call npx eslint --fix .||exit /b
call prettier --no-semi --print-width 132 --use-tabs --tab-width 8 -w .||exit /b
git diff
