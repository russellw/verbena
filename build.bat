setlocal
set cc=cl -EHsc -MDd -W2 -WX -nologo -std:c++20

md bin
cd bin

rem tools
rem for %%a in (..\tools\*.cc) do %cc% %%a setargv.obj||exit /b

rem database utilities
del *.cxx
compile-schema ..\db\schema.h||exit /b
for %%a in (..\db\*.csv) do compile-csv %%a||exit /b
for %%a in (..\db\*.cc) do %cc% -I..\db -I. %%a *.cxx ..\sqlite\sqlite3.c||exit /b

rem main program
del *.cxx
compile-pages ..\src\*-page.h||exit /b
for %%a in (..\src\*.png) do compile-bytes %%a||exit /b
%cc% -I..\src -I. ..\src\*.cc *.cxx ..\sqlite\sqlite3.c /Feverbena||exit /b
