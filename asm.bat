setlocal
set cc=cl -EHsc -Fa -O2 -W2 -nologo -std:c++20

md bin
cd bin

rem tools
for %%a in (..\tools\*.cc) do %cc% %%a setargv.obj||exit /b

rem main program
for %%a in (..\data\*.csv) do compile-csv %%a||exit /b
compile-schema ..\src\schema.h||exit /b
compile-pages ..\src\schema.h ..\src\*-page.h||exit /b
%cc% -I..\src -I. ..\src\*.cc *.cxx ..\sqlite\sqlite3.c /Feverbena||exit /b
