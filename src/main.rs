extern crate text_to_json;

use text_to_json::read;

fn main() {
  match read::to_json(String::from("example.txt")) {
    Ok(n) => println!("{:?}", n),
    Err(_) => panic!("Error"),
  }
}
