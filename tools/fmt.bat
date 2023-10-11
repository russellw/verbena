@echo off
cd %~dp0..

copy db %tmp% >nul:
copy src %tmp% >nul:
copy tools %tmp% >nul:

clang-format -i --style=file db\*.h db\*.cc||exit /b
clang-format -i --style=file src\*.h src\*.cc||exit /b
clang-format -i --style=file tools\*.h tools\*.cc||exit /b
sort-c -i db\*.h db\*.cc||exit /b
sort-c -i src\*.h src\*.cc||exit /b
sort-c -i tools\*.h tools\*.cc||exit /b
sort-cases -i db\*.h db\*.cc||exit /b
sort-cases -i src\*.h src\*.cc||exit /b
sort-cases -i tools\*.h tools\*.cc||exit /b
