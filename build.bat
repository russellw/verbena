setlocal
set cc=cl -EHsc -MDd -W2 -WX -nologo -std:c++20

md bin
cd bin

rem tools
for %%a in (..\tools\*.cc) do %cc% %%a setargv.obj||exit /b

rem main program
compile-pages ..\src\*.cpp||exit /b
compile-png ..\src\*.png||exit /b
%cc% "-IC:\Program Files\PostgreSQL\15\include" -I..\src -I. ..\src\*.cc *.cxx "C:\Program Files\PostgreSQL\15\lib\libpq.lib" /Feverbena||exit /b
