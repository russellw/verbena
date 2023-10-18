@echo off
clang-format -i --style=file *.cc||exit /b
clang-format -i --style=file src\*.cc src\*.h||exit /b
sort-c -i *.cc||exit /b
sort-c -i src\*.h src\*.cc||exit /b
sort-cases -i *.cc||exit /b
sort-cases -i src\*.h src\*.cc||exit /b
git diff
