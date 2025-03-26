set RUSTFLAGS=--emit=llvm-ir
cargo build --release||exit /b
set RUSTFLAGS=
wc target\debug\deps\verbena.ll
copy target\debug\deps\verbena.ll \t\1.ll
