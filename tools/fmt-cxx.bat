@echo off
cd %~dp0..

clang-format -i --style=file bin\*.hxx bin\*.cxx
