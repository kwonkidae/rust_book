use std::fmt::Display;
struct ImportantExcerpt<'a> {
	part: &'a str,
}

fn main() {
	println!("Hello, world!");
	let x = String::from("a");
	let b = String::from("b");

	let a = longest(&x, &b);
	println!("{}", a);
	{
		let novel = String::from("Call me Ishmael. Some years ago...");
		let first_sentence = novel.split('.')
				.next()
				.expect("Could not find a '.'");
		let i = ImportantExcerpt {part: first_sentence };
		println!("first_sentence: {}", first_sentence);
		println!("{}", novel);
	}
	{
		let string1 = String::from("long string is long");

		{
			let string2 = String::from("xyz");
			let result = longest(string1.as_str(), string2.as_str());

			println!("The longest string is {}", result);
		}
	}



	// {
	// 	let string1 = String::from("long string is long");
	// 	let result;
	// 	{
	// 		let string2 = String::from("xyz");
	// 		result = longest(string1.as_str(), string2.as_str());
	// 	}
	// }

	// {
	// 	fn longest<'a>(x: &str, y: &str) -> &str {
	// 		let result = String::from("really long string");
	// 		result.as_str()
	// 	}
	// }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
	if x.len() > y.len() {
		x
	} else {
		y
	}
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str 
	where T: Display
{
	println!("Announcement! {}", ann);
	if x.len() > y.len() {
		x
	} else {
		y
	}
}
