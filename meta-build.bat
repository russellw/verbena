cl -EHsc -MDd -W2 -nologo -std:c++20 meta-build.cc
if %errorlevel% neq 0 goto :eof

meta-build %*
if %errorlevel% neq 0 goto :eof

del meta-build.obj
del meta-build.exe

rem run build a few times to let ninja figure out header dependencies
call build
call build
call build
