@echo off
clang-format -i --style=file src\*.cc src\*.h||exit /b
clang-format -i --style=file tools\*.cc tools\*.h||exit /b
sort-c -i src\*.h src\*.cc||exit /b
sort-c -i tools\*.h tools\*.cc||exit /b
sort-cases -i src\*.h src\*.cc||exit /b
sort-cases -i tools\*.h tools\*.cc||exit /b
git diff
