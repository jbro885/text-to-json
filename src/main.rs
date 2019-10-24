extern crate text_to_json;

use text_to_json::read::text;

fn main() {
  // Example: Using text.
  match text(String::from("example.txt")) {
    Ok(text) => println!("{:?}", text),
    Err(_) => println!("Error"),
  }
}
