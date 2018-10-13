fn main() {
	let s1 = String::from("hello");
	let s2 = s1.clone();

	println!("s1 = {}, s2 = {}", s1, s2);

	let x = 5;
	let y = x;

	println!("x = {}, y = {}", x, y);

	takes_ownership(s2);

	makes_copy(x);

	// println!("s2 = {}", s2);
	println!("x = {}", x);

	{
		let _s1 = gives_ownership();
		let s2 = String::from("hello");
		let _s3 = takes_and_gives_back(s2);
	}

	{
		let s1 = String::from("hello");
		let (s2, len) = calculate_length(s1);
		// println!("s1 = {}", s1);
		println!("The length of '{}' is {}.", s2, len);
	}
}

fn takes_ownership(some_string: String) {
	println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
	println!("{}", some_integer);
}

fn gives_ownership() -> String {
	let some_string = String::from("world");

	some_string
}

fn takes_and_gives_back(a_string: String) -> String {
	a_string
}

fn calculate_length(s: String) -> (String, usize) {
	let length = s.len();

	(s, length)
}
