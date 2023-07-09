setlocal
set cc=cl -EHsc -MDd -W2 -nologo -std:c++20 "-IC:\Program Files\PostgreSQL\15\include"

md bin
cd bin

for %%a in (..\tools\*.cc) do %cc% %%a setargv.obj||exit /b
