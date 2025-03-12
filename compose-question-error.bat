echo The following code is getting this error:>\t\post.txt
cargo test 2>>\t\post.txt
echo ```>>\t\post.txt
type %1 >>\t\post.txt
echo ```>>\t\post.txt
clip <\t\post.txt
