use std::sync::{Mutex, Arc};
use std::thread;
// use std::rc::Rc;

fn main() {
	{
		let m = Mutex::new(5);
		{
			let mut num = m.lock().unwrap();
			*num = 6;
		}
		println!("m = {:?}", m);
	}

	{
		// let counter = Mutex::new(0);
		// let mut handles = vec![];

		// for _ in 0..10 {
		// 	let handle = thread::spawn(move || {
		// 		let mut num = counter.lock().unwrap();

		// 		*num += 1;
		// 	});
		// 	handles.push(handle);
		// }

		// for handle in handles {
		// 	handle.join().unwrap();
		// }

		// println!("Result: {}", *counter.lock().unwrap());
	}

	{
		// let counter = Mutex::new(0);
		// let mut handles = vec![];

		// let handle = thread::spawn(move || {
		// 	let mut num = counter.lock().unwrap();

		// 	*num += 1;
		// });
		// handles.push(handle);
		// let handle2 = thread::spawn(move || {
		// 	let mut num2 = counter.lock().unwrap();

		// 	*num2 += 1;
		// });
		// handles.push(handle2);

		// for handle in handles {
		// 	handle.join().unwrap();
		// }

		// println!("Result: {}", *counter.lock().unwrap());
	}

	{
		let counter = Arc::new(Mutex::new(0));
		let mut handles = vec![];

		for _ in 0..10 {
			let counter = Arc::clone(&counter);
			let handle = thread::spawn(move || {
				let mut num = counter.lock().unwrap();

				*num += 1;
			});
			handles.push(handle);
		}

		for handle in handles {
			handle.join().unwrap();
		}

		println!("Result: {}", *counter.lock().unwrap());
	}
}
