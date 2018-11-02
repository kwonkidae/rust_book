#[cfg(test)]
mod tests {
	// use super::client;
	#[test]
	fn it_works() {
		super::client::connect();
	}
}
pub mod client;
pub mod network;

mod outermost {
	pub fn middle_function() {}

	fn middle_secret_function() {}

	mod inside {
		pub fn inner_function() {}

		fn secret_function() {}
	}
}

fn try_me() {
	outermost::middle_function();
	// outermost::middle_secret_function();
	// outermost::inside::inner_function();
	// outermost::inside::secret_function();
}
