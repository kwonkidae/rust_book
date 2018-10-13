enum List {
	Cons(i32, Box<List>),
	Nil,
}

use List::{Cons, Nil};

struct CustomSmartPointer {
	data: String,
}

impl Drop for CustomSmartPointer {
	fn drop(&mut self) {
		println!("Dropping CustomSmartPointer with data {}!", self.data);
	}
}

fn main() {
	{
		let b = Box::new(5);
		println!("b = {}", b);

		let _list = Cons(1, 
			Box::new(Cons(2, 
				Box::new(Cons(3, 
					Box::new(Nil))))));
	}

	{
		let c = CustomSmartPointer { data: String::from("my stuff") };
		// c.drop();
		drop(c);
		let _d = CustomSmartPointer { data: String::from("other stuff") };

		println!("CustomSmartPointers created.");
	}
}
