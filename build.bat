setlocal
set cc=cl -EHsc -MDd -W2 -WX -nologo -std:c++20

md bin
cd bin

%cc% ..\compile-pages.cc setargv.obj||exit /b
compile-http ..\src\*.png||exit /b
compile-pages ..\src\*.cpp||exit /b
%cc% "-IC:\Program Files\PostgreSQL\15\include" -I..\src -I. ..\src\*.cc *.cxx "C:\Program Files\PostgreSQL\15\lib\libpq.lib" /Feverbena||exit /b
