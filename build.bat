setlocal
set cc=cl -EHsc -MDd -W2 -nologo -std:c++20 "-IC:\Program Files\PostgreSQL\15\include"

md bin
cd bin

for %%a in (..\tools\*.cc) do %cc% %%a setargv.obj||exit /b
for %%a in (..\data\*.csv) do compile-csv %%a||exit /b
compile-schema ..\src\schema.h||exit /b
%cc% -I..\src -I. ..\src\*.cc *.cxx "C:\Program Files\PostgreSQL\15\lib\libpq.lib" /Feverbena||exit /b
