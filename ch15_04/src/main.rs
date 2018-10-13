#![allow(unused_variables, dead_code)]
enum List {
	Cons(i32, Rc<List>),
	Nil,
}

use List::{Cons, Nil};
use std::rc::Rc;

fn main() {
	// let a = Cons(5,
	// 	Box::new(Cons(10,
	// 		Box::new(Nil))));
		
	// let b = Cons(3, Box::new(a));
	// let c = Cons(3, Box::new(a));

	{
		let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
		println!("count after creating a = {}", Rc::strong_count(&a));
		let b = Cons(3, Rc::clone(&a));
		println!("count after creating b = {}", Rc::strong_count(&a));
		{
			let c = Cons(4, Rc::clone(&a));
			println!("count after creating c = {}", Rc::strong_count(&a));
		}
		println!("count after c goes out of scope = {}", Rc::strong_count(&a));
	}

	// let string1 = String::from("kkdosk");
	// let string2 = String::from("oskkkd");

	// let result = longest(string1.as_str(), string2.as_str());
	// println!("{}", result);

	// {
	// 	let string1 = String::from("long string is long");
	// 	let string2 = String::from("xyz");
	// 	let result;
	// 	{
			
	// 		result = longest(string1.as_str(), string2.as_str());
			
	// 	}
	// 	println!("The longest string is {}", result);
	// }
}

// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
// 	if x.len() > y.len() {
// 		x
// 	} else {
// 		y
// 	}
// }
