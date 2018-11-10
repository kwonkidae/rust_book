use std::slice;

static HELLO_WOLRD: &str = "hello, world!";

static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
	unsafe {
		COUNTER += inc;
	}
}

fn main() {
	{

		add_to_count(3);

		println!("COUNTER: {}", COUNTER);
		println!("name is: {}", HELLO_WOLRD);
		let mut num = 5;

		let r1 = &num as *const i32;
		let r2 = &mut num as *mut i32;

		unsafe {
			println!("r1 is: {}", *r1);
			println!("r2 is: {}", *r2);
		}

		// let address = 0x012345usize;
		// let r = address as *const i32;

		// unsafe {
		// 	println!("r is : {}", *r);
		// }
		{
			let mut v = vec![1,2,3,4,5,6];
			let r = &mut v[..];

			let (a, b) = split_at_mut(r, 3);
			println!("{:?} {:?}",a ,b);
			assert_eq!(a, &mut [1,2,3]);
			assert_eq!(b, &mut [4,5,6]);
		}
	}

	unsafe {
		println!("Absolute value of -3 according to C: {}", abs(-3));
	}
}

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
	println!("split_at_mut");
	let len = slice.len();
	let ptr = slice.as_mut_ptr();

	assert!(mid <= len);
	// isize The pointer-sized signed integer type.
	unsafe {
		(slice::from_raw_parts_mut(ptr, mid),
		 slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid))
	}
}

extern "C" {
	fn abs(input: i32) -> i32;
}

