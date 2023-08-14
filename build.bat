setlocal
set cc=cl -EHsc -MDd -W2 -WX -nologo -std:c++20

md bin
cd bin

rem tools
for %%a in (..\tools\*.cc) do %cc% %%a setargv.obj||exit /b

rem database utilities
compile-schema ..\db\schema.h||exit /b
for %%a in (..\db\*.csv) do compile-csv %%a||exit /b
for %%a in (..\db\*.cc) do %cc% -I..\db -I. %a *.cxx ..\sqlite\sqlite3.c||exit /b
del *.cxx

rem main program
compile-pages ..\src\*-page.h||exit /b
for %%a in (..\src\*.png) do compile-bytes %%a||exit /b
%cc% -I..\src -I. ..\src\*.cc *.cxx ..\sqlite\sqlite3.c /Feverbena||exit /b
