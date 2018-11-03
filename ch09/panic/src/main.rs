
use std::fs::File;
use std::io::ErrorKind;
use std::io::Read;
use std::io;

fn stringify(x: u32) -> String { format!("error code: {}", x) }
fn main() {
	{
		// panic!("carsh and burn");
		// let v = vec![1,2,3];
		// v[99];
		}

	{
		let x: Result<u32, u32> = Ok(2);
		assert_eq!(x.map_err(stringify), Ok(2));

		let x: Result<u32, u32> = Err(13);
		assert_eq!(x.map_err(stringify), Err("error code: 13".to_string()))
	}

	{
		// let f = File::open("hello1.txt").expect("Failed to open hello.txt");
	}

	{
		let f = File::open("hello.txt");

		let _f = match f {
			Ok(file) => file,
			Err(error) => match error.kind() {
				ErrorKind::NotFound => match File::create("hello.txt") {
					Ok(fc) => fc,
					Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
				},
				other_error => panic!("There was a problem opening the file: {:?}", other_error),
			},
		};

		let _str = read_username_from_file("hello.txt");
		println!("{:?}", _str);
		let _short = short_read_username_from_file();
		println!("{:?}", _short);
	}

	{
		fn read_username_from_file() -> Result<String, io::Error> {
			let mut s = String::new();
			File::open("hello.txt")?.read_to_string(&mut s)?;
			Ok(s)
		}

		println!("{:?}", read_username_from_file());
	}

	{
		use std::fs;
		fn read_username_from_file() -> Result<String, io::Error> {
			fs::read_to_string("hello.txt")
		}

		println!("{:?}", read_username_from_file());
	}
}

fn read_username_from_file(filename: &str) -> Result<String, io::Error> {
	println!("filename: {}", filename);
	let f = File::open(filename);

	let mut f = match f {
		Ok(file) => file,
		Err(e) => return Err(e),
	};

	let mut s = String::new();

	match f.read_to_string(&mut s) {
		Ok(_) => Ok(s),
		Err(e) => Err(e),
	}
}

// A Shortcut for Propagating Error: the ? Operator
fn short_read_username_from_file() -> Result<String, io::Error> {
	let mut f = File::open("hello.txt")?;
	let mut s = String::new();
	f.read_to_string(&mut s)?;
	Ok(s)
}
