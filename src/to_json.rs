use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::error::Error;

pub fn to_json() -> std::io::Result<String> {
  let path = Path::new("example.txt");
  let display = path.display();

  let mut file = match File::open(&path) {
    Err(why) => panic!("couldn't read {}: {}", display, why.description()),
    Ok(file) => file,
  };

  let mut buffer = String::new();
  file.read_to_string(&mut buffer)?;
  Ok(buffer)
}
