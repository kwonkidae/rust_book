
extern crate communicator;

pub mod a {
  pub mod series {
    pub mod of {
      pub fn nested_modules() {
        println!("nested module")
      }
    }
  }
}

use a::series::of;

fn main() {
  communicator::client::connect();
  of::nested_modules();
}
