echo The following code is getting this error:>\t\q.txt
cargo test 2>>\t\q.txt
echo ```>>\t\q.txt
type %1 >>\t\q.txt
echo ```>>\t\q.txt
clip <\t\q.txt
