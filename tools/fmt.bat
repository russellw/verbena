@echo off
cd %~dp0..

copy src %tmp% >nul:
copy tools %tmp% >nul:
copy example %tmp% >nul:

clang-format -i --style=file meta-build.cc
if %errorlevel% neq 0 goto :eof

clang-format -i --style=file src\*.h src\*.cc
if %errorlevel% neq 0 goto :eof

clang-format -i --style=file tools\*.h tools\*.cc
if %errorlevel% neq 0 goto :eof

clang-format -i --style=file example\*.h example\*.cc
if %errorlevel% neq 0 goto :eof

bin\sort-c src\*.h src\*.cc
if %errorlevel% neq 0 goto :eof

bin\sort-c tools\*.cc
if %errorlevel% neq 0 goto :eof

bin\sort-c example\*.h example\*.cc
if %errorlevel% neq 0 goto :eof

bin\sort-cases src\*.h src\*.cc
if %errorlevel% neq 0 goto :eof

bin\sort-cases tools\*.cc
if %errorlevel% neq 0 goto :eof

bin\sort-cases example\*.cc
if %errorlevel% neq 0 goto :eof
