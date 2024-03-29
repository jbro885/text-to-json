extern crate text_to_json;

use text_to_json::read;

fn main() {
  // Example: to read::to_json(file.txt).
  match read::to_json(String::from("example.txt")) {
    Ok(content) => println!("{:?}", content),
    Err(_) => panic!("Error"),
  }
}
