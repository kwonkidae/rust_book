
use std::fs::File;
use std::io::ErrorKind;

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
		let f = File::open("hello1.txt").expect("Failed to open hello.txt");
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
	}
}
