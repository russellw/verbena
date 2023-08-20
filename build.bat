setlocal
set cc=cl -EHsc -MDd -W2 -WX -nologo -std:c++20

md bin
cd bin

rem tools
for %%a in (..\tools\*.cc) do %cc% %%a setargv.obj||exit /b

rem database programs
compile-schema ..\db\schema.h||exit /b
compile-csv ..\db\*.csv||exit /b
%cc% ..\sqlite\sqlite3.c -c||exit /b
for %%a in (..\db\*.cc) do %cc% -I..\db -I. %%a sqlite3.obj||exit /b

rem main program
compile-pages ..\src\*-page.h||exit /b
compile-bytes ..\src\*.png||exit /b
%cc% -I..\src -I. ..\src\*.cc *.cxx sqlite3.obj /Feverbena||exit /b
