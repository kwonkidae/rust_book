type Kilometers = i32;

fn main() {
	let x: i32 = 5;
	let y: Kilometers = 5;

	println!("x + y = {}", x + y);

	let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));


	type Thunk = Box<dyn Fn() + Send + 'static>;

	let f: Thunk = Box::new(|| println!("hi"));

	fn bar() -> ! {
		panic!("kkdosk");
	}

	// let s1: str = "Hello there!";

	
}

fn generic<T: Sized>(t: &T) {

}

struct Foo<T>(T);
struct Bar<T: ?Sized>(T);
// struct FooUse(Foo<[i32]>);
struct BarUse(Bar<[i32]>);
