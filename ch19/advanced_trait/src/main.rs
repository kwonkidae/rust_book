use std::ops::Add;
use std::fmt;
#[derive(Debug, PartialEq)]
struct Point {
	x: i32,
	y: i32,
}

impl Add for Point {
	type Output = Point;

	fn add(self, other: Point) -> Point {
		Point {
			x: self.x + other.x,
			y: self.y + other.y,
		}
	}
}

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
	type Output = Millimeters;
	
	fn add(self, other: Meters) -> Millimeters {
		Millimeters(self.0 + (other.0 * 1000))
	}
}

trait Pilot {
	fn fly(&self);
}

trait Wizard {
	fn fly(&self);
}

struct Human;

impl Pilot for Human {
	fn fly(&self) {
		println!("This is your captain speaking.");
	}
}

impl Wizard for Human {
	fn fly(&self) {
		println!("up");
	}
}

impl Human {
	fn fly(&self) {
		println!("@waving arms furiously*");
	}
}

trait Animal {
	fn baby_name() -> String;
}

struct Dog;

impl Dog {
	fn baby_name() -> String {
		String::from("Spot")
	}
}

impl Animal for Dog {
	fn baby_name() -> String {
		String::from("puppy")
	}
}

fn main() {
	let p1 = Point { x: 1, y: 0 };

	p1.outline_print();
	let p2 = Point { x: 2, y: 3 };

	println!("{:?}", p1 + p2);
	// println!("{:?}", p1);

	let person = Human;
	person.fly();
	Pilot::fly(&person);
	Wizard::fly(&person);

	println!("A baby dog is called a {}", Dog::baby_name());
	println!("A baby dog is called a {}", <Dog as Animal>::baby_name());

	let w = Wrapper(vec![String::from("hello"), String::from("world")]);
	println!("w = {}", w);
}

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
	fn fmt(&self, f: &mut fmt::Formatter) ->fmt::Result {
		write!(f, "[{}]", self.0.join(", "))
	}
}

impl OutlinePrint for Point {}

impl fmt::Display for Point {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "({} {})", self.x, self.y)
	}
}

trait OutlinePrint: fmt::Display {
	fn outline_print(&self) {
		let output = self.to_string();
		let len = output.len();
		println!("{}", "*".repeat(len + 4));
		println!("*{}*", " ".repeat(len + 2));
		println!("* {} *", output);
		println!("*{}*", " ".repeat(len + 2));
		println!("{}", "*".repeat(len + 4));
	}
}
