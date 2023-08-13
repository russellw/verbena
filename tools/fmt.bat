@echo off
cd %~dp0..

copy src %tmp% >nul:
copy tools %tmp% >nul:

clang-format -i --style=file src\*.h src\*.c*||exit /b
clang-format -i --style=file tools\*.h tools\*.cc||exit /b
bin\sort-c src\*.h src\*.c*||exit /b
bin\sort-c tools\*.h tools\*.cc||exit /b
bin\sort-cases src\*.h src\*.c*||exit /b
bin\sort-cases tools\*.h tools\*.cc||exit /b
