use delete_prefix::delete_prefix;

fn main() {
	println!("{:?}", delete_prefix("ab", "cdabefghijklmnop"));
	println!("{:?}", delete_prefix("x", "abcdefghijklmnop"));
}