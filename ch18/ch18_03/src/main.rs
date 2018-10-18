
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
