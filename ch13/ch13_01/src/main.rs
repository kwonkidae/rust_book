use std::thread;
use std::time::Duration;

fn main() {
	let simulated_user_specified_value = 10;
	let simulated_random_number = 7;

	// generate_workout(
	// 	simulated_user_specified_value,
	// 	simulated_random_number
	// );

	generate_workout_cacher(
		simulated_user_specified_value,
		simulated_random_number
	);

	let x = 4;
	let equal_to_x = |z| z == x;

	let y = 4;

	assert!(equal_to_x(y));

	{
		let x = 4;
		// fn equal_to_x(z: i32) -> bool { z == x }
		let _x = x;
		println!("{}", x);
		let y = 4;

		// assert!(equal_to_x(y));
	}

	{
		let x = vec![1,2,3];
		println!("{:?}", x);
		let equal_to_x = |z| {
			z == x;
		};

		println!("can't use x here: {:?}", x);

		let y = vec![1,2,3];

		assert!(equal_to_x(y));

		let v = vec![1,2,3,4,5];

		let third: &i32 = &v[2];

		println!("{}", *third);
	}
}

fn simulated_expensive_caculation(intensity: u32) -> u32 {
	println!("caculationg slowly...");
	thread::sleep(Duration::from_secs(2));
	intensity
}

fn generate_workout(intensity: u32, random_number: u32) {
	let add_one_v2 = |x: u32| -> u32 { x + 1};
	// let add_one_v3 = |x| {x + 1}; cannot infer type
	// let add_on3_v4 = |x| x + 1;

	let example_closure = |x| x;

	let s = example_closure(String::from("hello"));
	// let n = example_closure(5);
	let expensive_closure = |num: u32| -> u32 {
		println!("calculating slowly...");
		thread::sleep(Duration::from_secs(2));
		num
	};

	if intensity < 25 {
		println!(
			"Today, do {} pushups!",
			expensive_closure(intensity)
		);
		println!(
			"Next, do {} situps!",
			expensive_closure(intensity)
		);
	} else {
		if random_number == 3 {
			println!("Take a break today! Remember to stay hudrated");
		} else {
			println!(
				"Today, run for {} minutes",
				expensive_closure(intensity)
			);
		}
	}
}

fn generate_workout_cacher(intensity: u32, random_number: u32) {
	let mut expensive_result = Cacher::new(|num| {
		println!("calculating slowly...");
		thread::sleep(Duration::from_secs(2));
		num
	});

	if intensity < 25 {
		println!(
			"Today, do {} pushups!",
			expensive_result.value(intensity)
		);
		println!(
			"Next, do {} situps",
			expensive_result.value(intensity)
		);
	} else {
		if random_number == 3 {
			println!("Take a break today! Remember to stay hydrated!");
		} else {
			println!(
				"Today, run for {} minutes!",
				expensive_result.value(intensity)
			);
		}
	}
}

struct Cacher<T>
	where T: Fn(u32) -> u32
{
	calculation: T,
	value: Option<u32>,
}

impl<T> Cacher<T>
	where T: Fn(u32) -> u32
{
	fn new(calculation: T) -> Cacher<T> {
		Cacher {
			calculation,
			value: None,
		}
	}

	fn value(&mut self, arg: u32) -> u32 {
		match self.value {
			Some(v) => v,
			None => {
				let v = (self.calculation)(arg);
				self.value = Some(v);
				v
			},
		}
	}
}

#[test]
fn call_with_different_values() {
	let mut c = Cacher::new(|a| a);

	let v1 = c.value(1);
	let v2 = c.value(2);

	assert_eq!(v2, 2);

}
