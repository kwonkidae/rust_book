pub trait Draw {
	fn draw(&self);
}

pub struct Screen {
	pub components: Vec<Box<Draw>>,
}


impl Screen {
	pub fn run(&self) {
		for component in self.components.iter() {
			component.draw();
		}
	}
}

pub struct Button {
	pub width: u32,
	pub height: u32,
	pub label: String,
}

struct SelectBox {
	width: u32,
	height: u32,
	options: Vec<String>,
}

impl Draw for SelectBox {
	fn draw(&self) {
		println!("SelectBox width = {}, height = {} options = {:?}", self.width, self.height, self.options);
	}
}

impl Draw for Button {
	fn draw(&self) {
		println!("width: {}, height: {}, label: {}", self.width, self.height, self.label);
	}
}

fn main() {
	let screen = Screen {
		components: vec![
			Box::new(SelectBox {
				width: 75,
				height: 10,
				options: vec![
					String::from("Yes"),
					String::from("Maybe"),
					String::from("No")
				],
			}),
			Box::new(Button {
				width: 50,
				height: 10,
				label: String::from("OK"),
		}),
		],
	};
	screen.run();
}
