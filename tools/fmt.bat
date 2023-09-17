@echo off
cd %~dp0..

copy db %tmp% >nul:
copy src %tmp% >nul:
copy tools %tmp% >nul:

clang-format -i --style=file db\*.h db\*.cc||exit /b
clang-format -i --style=file src\*.h src\*.cc||exit /b
clang-format -i --style=file tools\*.h tools\*.cc||exit /b
sort-c db\*.h db\*.cc||exit /b
sort-c src\*.h src\*.cc||exit /b
sort-c tools\*.h tools\*.cc||exit /b
sort-cases db\*.h db\*.cc||exit /b
sort-cases src\*.h src\*.cc||exit /b
sort-cases tools\*.h tools\*.cc||exit /b
