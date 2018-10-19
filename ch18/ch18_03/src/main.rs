
fn main() {
	println!("Hello, world!");

	let x = Some(5);
	let y = 10;

	match x {
		Some(50) => println!("Got 50"),
		Some(y) => println!("Matched, y = {:?}", y),
		_ => println!("Default case, x = {:?}", x),
	}

	println!("at the end: x = {:?}, y = {:?}", x, y);

	{
		let x = 1;
		match x {
			1 | 2 => println!("one or two"),
			3 => println!("three"),
			_ => println!("anything"),
		}
	}

	{
		let x = 5;
		match x {
			1 ... 5 => println!("one through five"),
			_ => println!("something else"),
		}
	}

	{
		let x = 'c';
		match x {
			'a' ... 'j' => println!("early ASCII letter"),
			'k' ... 'z' => println!("late ASCII letter"),
			_ => println!("something else"),
		}
	}

	{
		let p = Point { x: 0, y: 7 };
		let Point {x: a, y: b} = p;
		println!("a: {}, b: {}", a, b);
	}

	{
		let p = Point { x: 0, y: 7 };

		let Point { x, y } = p;
		println!("x: {}, y: {}", x, y);

		{
			match p {
				Point { x, y: 0 } => println!("On the x axis at {}", x),
				Point { x: 0, y } => println!("On the y axis at {}", y),
				Point { x, y } => println!("On neither axis: ({}, {})", x, y),
			}
		}
	}

	{
		let msg = Message::Write(String::from("asdf"));

		match msg {
			Message::Quit => {
				println!("The Quit variant has not date to destructure")
			},
			Message::Move { x , y } => {
				println!(
					"Move in the x direction {} and in the y direction {}",
					x,
					y
				);
			},
			Message::Write(text) => println!("Text message: {}", text),
			Message::ChangeColor(r, g, b) => {
				println!(
					"Change the color to red {}, green {} and blue {}",
					r,
					g,
					b
				)
			}
		}
	}

	{
		let msg = MessageColor::ChangeColor(Color::Hsv(0, 160, 255));

		match msg {
			MessageColor::ChangeColor(Color::Rgb(r, g, b)) => {
				println!(
					"Change the color to red {}, green {}, and blue {}",
					r,
					g,
					b
				)
			},
			MessageColor::ChangeColor(Color::Hsv(h, s, v)) => {
				println!(
					"Change the color to hue {}, saturation {}, and value {}",
					h,
					s,
					v
				)
			}
			_ => ()
		}
	}

	{
		let points = vec![
			Point {x: 0, y: 0},
			Point {x: 1, y: 5},
			Point {x: 10, y: -3},
		];

		let sum_of_squares: i32 = points
			.iter()
			.map(|Point {x, y}| x * x + y * y)
			.sum();

		println!("sum of squares: {}", sum_of_squares);


		let ((feet, inches), Point {x, y}) = ((3, 10), Point {x: 3, y: -10});
		println!("{} {} {} {}", feet, inches, x, y);

		foo(3, 4);
	}

	{
		let mut setting_value = Some(5);
		let new_setting_value = Some(10);

		match (setting_value, new_setting_value) {
			(Some(_), Some(_)) => {
				println!("Can't overwrite an existing customized value");
			}
			_ => {
				setting_value = new_setting_value;
			}
		}

		println!("setting is {:?}", setting_value);

		let numbers = (2, 4, 8, 16, 32);

		match numbers {
			(first, _, third, _, fifth) => {
				println!("Some numbers: {}, {}, {}", first, third, fifth)
			},
		}

		let _x = 5;
		let y = 10;

		let s = Some(String::from("Hello!"));

		if let Some(_s) = s {
			println!("found a string!");
		}

		// println!("{:?}", s);
	}

	{
		let s = Some(String::from("Hello!"));

		if let Some(_) = s {
			println!("found a string");
		}

		println!("{:?}", s);

		struct Point {
			x: i32,
			y: i32,
			z: i32,
		}

		let origin = Point { x: 0, y: 0, z: 0 };
		match origin {
			Point {x, .. } => println!("x is {}", x),
		}

		let numbers = (2, 4, 8, 16, 32);

		match numbers {
			(first, .. , last) => {
				println!("Some numbers: {}, {}", first, last);
			},
		}

		// match numbers {
		// 	(.., second, ..) => {
		// 		println!("Some numbers: {}", second);
		// 	},
		// }

		let num = Some(10);
		match num {
			Some(x) if x < 5 => println!("less than five: {}", x),
			Some(x) => println!("{}", x),
			None => (),
		}
	}

	{
		let x = Some(5);
		let y = 10;

		match x {
			Some(50) => println!("Got 50"),
			Some(n) if n == y => println!("Matched, n = {:?}", n),
			_ => println!("Default case, x = {:?}", x),
		}

		println!("at the end: x = {:?}, y = {:?}", x, y);
	}

	{
		let x = 4;
		let y = false;

		match x {
			4 | 5 | 6 if y => println!("yes"),
			_ => println!("no")
		}
	}

	{
		enum Message {
			Hello { id: i32 },
		}

		let msg = Message::Hello { id: 5 };

		match msg {
			Message::Hello { id: id_variable @ 3...7 } => {
				println!("Found an id in range: {}", id_variable)
			},
			Message::Hello { id: 10...12 } => {
				println!("Found an id in another range");
			},
			Message::Hello { id } => {
				println!("Found some other id: {}", id)
			},
		}

		let robot_name = &Some(String::from("Bors"));

		match robot_name {
			&Some(ref name) => println!("Found a name: {}", name),
			None => (),
		}

		println!("robot name is: {:?}", robot_name);
	}
}

struct Point {
	x: i32,
	y: i32,
}

enum Message {
	Quit,
	Move { x: i32, y: i32 },
	Write(String),
	ChangeColor(i32, i32, i32),
}

enum Color {
	Rgb(i32, i32, i32),
	Hsv(i32, i32, i32)
}

enum MessageColor {
	Quit,
	Move { x: i32, y: i32 },
	Write(String),
	ChangeColor(Color),
}

fn foo(_: i32, y: i32) {
	println!("This code only uses the y parameter: {}", y);
}
