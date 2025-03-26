set RUSTFLAGS=--emit=llvm-ir
cargo build --release||exit /b
set RUSTFLAGS=
wc target\release\deps\verbena.ll
move \t\9.ll \t\8.ll
copy target\release\deps\verbena.ll \t\9.ll
