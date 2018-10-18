fn main() {
	let favorite_color: Option<&str> = None;
	let is_tuesday = false;
	let age: Result<u8, _> = "34".parse();

	if let Some(color) = favorite_color {
		println!("Using your favorite color, {}, as background", color);
	} else if is_tuesday {
		println!("Tuesday is green day!");
	} else if let Ok(age) = age {
		if age > 30 {
			println!("Using purple as the background color");
		} else {
			println!("Using orange as the background color");
		}
	} else {
		println!("Using blue as the background color");
	}

	{
		let mut stack = Vec::new();

		stack.push(1);
		stack.push(2);
		stack.push(3);

		while let Some(top) = stack.pop() {
			println!("{}", top);
		}

		let v = &['a', 'b', 'c'];

		for (index, value) in v.iter().enumerate() {
			println!("{} is at index {}", value, index);
		}

		println!("{:?}", v);
	}

	{
		let (x, y, z) = (1, 2, 3);
		println!("{} {} {}", x, y, z);
	}

	let point = (3, 5);
	print_coordinates(&point);

	{
		let s = Some(5);
		// let Some(x) = s;
		if let Some(x) = s {
			println!("{}", x);
		}

		// if let x = 5 {
		// 	println!("{}", x);
		// }
	}

	{
		let x = 1;
		match x {
			1 => println!("one"),
			2 => println!("two"),
			3 => println!("three"),
			_ => println!("anything"),
		}
	}
}

fn print_coordinates(&(x, y): &(i32, i32)) {
	println!("Current location: ({}, {})",x , y);
}
  