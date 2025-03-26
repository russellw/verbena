set RUSTFLAGS=--emit=llvm-ir
cargo build --release||exit /b
set RUSTFLAGS=
wc target\debug\deps\verbena.ll
move \t\9.ll \t\8.ll
copy target\debug\deps\verbena.ll \t\9.ll
