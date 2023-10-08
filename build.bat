setlocal
set cc=cl -EHsc -MDd -W2 -WX -nologo -std:c++20
set ipg="-IC:\Program Files\PostgreSQL\15\include"
set lpg="C:\Program Files\PostgreSQL\15\lib\libpq.lib"

md bin
cd bin

rem tools
for %%a in (..\tools\*.cc) do %cc% %%a setargv.obj||exit /b

rem database programs
compile-schema ..\db\schema.h||exit /b
compile-csv ..\db\*.csv||exit /b
for %%a in (..\db\*.cc) do %cc% %ipg% -I..\db -I. %%a %lpg%||exit /b

rem main program
compile-pages ..\src\*.cpp||exit /b
del data.cxx
del data.hxx
compile-png ..\src\*.png||exit /b
%cc% %ipg% -I..\src -I. ..\src\*.cc *.cxx %lpg% /Feverbena||exit /b
