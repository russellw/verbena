setlocal
set cc=cl -EHsc -MDd -W2 -WX -nologo -std:c++20

md bin
cd bin

rem tools
for %%a in (..\tools\*.cc) do %cc% %%a ..\sqlite\sqlite3.c setargv.obj||exit /b

rem main program
for %%a in (..\data\*.csv) do compile-csv %%a||exit /b
for %%a in (..\data\*.png) do compile-bytes %%a||exit /b
compile-schema ..\src\schema.h||exit /b
compile-pages ..\src\*-page.h||exit /b
%cc% -I..\src -I. ..\src\*.cc *.cxx ..\sqlite\sqlite3.c /Feverbena||exit /b
