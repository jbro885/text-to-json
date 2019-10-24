extern crate text_to_json;

use text_to_json::read::{text, to_array};

fn main() {
  // Example: Using text.
  let text = match text(String::from("example.txt")) {
    Ok(t) => t,
    Err(_) => panic!("Error"),
  };

  // Example: Using to_array.
  match to_array(text) {
    Ok(t) => println!("{:?}", t),
    Err(_) => panic!("Error"),
  }
}
