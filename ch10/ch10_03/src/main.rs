

fn main() {
	println!("Hello, world!");
	let x = String::from("a");
	let b = String::from("b");

	let a = longest(&x, &b);
	println!("{}", a);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
	if x.len() > y.len() {
		x
	} else {
		y
	}
}
