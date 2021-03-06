fn main() {
	let v: Vec<i32> = 
	Vec::new();
	println!("{:?}", v);

	{
		let v = vec![1,2,3];
		println!("{:?}", v);
	}

	{
		let mut v = Vec::new();

		v.push(5);
		v.push(6);
		v.push(7);
		v.push(8);

		println!("{:?}", v);
	}

	{
		let v = vec![1,2,3,4,5];
		let third: &i32 = &v[2];
		println!("{}", third);

		let v_index = 2;
		match v.get(v_index) {
			Some(_) => { println!("Reachable element at index: {}", v_index);},
			None => { println!("Unreachable element at index: {}", v_index);}
		}
	}

	{
		// let v = vec![1,2,3,4,5];

		// let does_not_exist = &v[100];
		// let does_not_exist = v.get(100);
	}

	{
		let mut v = vec![1,2,3,4,5];
		{
			v.push(6);
		}
		let first = &v[0];
		println!("{:?}", v);
	}

	{
		let v = vec![100, 32, 57];
		for i in &v {
			println!("{}", i);
		}
	}

	{
		let mut v = vec![100, 32, 57];
		println!("{:?}", v);

		for i in &mut v {
			*i += 50;
		}
		let _a = &v[0];
		
	}

	{
		let row = vec![
			SpreadsheetCell::Int(3),
			SpreadsheetCell::Text(String::from("blue")),
			SpreadsheetCell::Float(10.12),
		];
		println!("{:?}", row);
	}
}
#[derive(Debug)]
enum SpreadsheetCell {
	Int(i32),
	Float(f64),
	Text(String),
}

