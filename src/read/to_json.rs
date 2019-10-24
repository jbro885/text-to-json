extern crate serde_json;

use crate::*;
use read::text;
use serde_json::json;

pub fn to_json(file: String) -> std::io::Result<Vec::<String>> {
  let t = match text(file) {
    Err(_) => panic!("Oops, can't read: <file>.txt"),
    Ok(t) => t,
  };

  let f = match spliting_text_by_space(t) {
    Ok(f) => f,
    Err(_) => panic!("Oops, can't read: <file>.txt"),
  };

  Ok(f)
}

fn spliting_text_by_space(text: String) -> std::io::Result<Vec::<String>> {
  let space_char: &'static str = "\n";
  let mut res = Vec::new();
  for c in text.split(space_char) {
    let get_char = match spliting_text_by_key(c) {
      Ok(f) => f,
      Err(_) => panic!("Error"),
    };
    res.push(get_char);
  }
  Ok(res)
}

fn spliting_text_by_key(text: &str) -> std::io::Result<String> {
  let equal_char: &'static str = "=";
  let mut result = Vec::new();
  for c in text.split(equal_char) {
    result.push(c);
  }
  let data_json = json!({
    result[0]: result[1],
  });
  Ok(data_json.to_string())
}
