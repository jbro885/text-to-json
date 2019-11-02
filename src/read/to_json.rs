extern crate serde_json;

use crate::*;
use read::text;
use serde_json::json;
use std::io;

pub fn to_json(file: String) -> io::Result<Vec::<String>> {
  let txt = text(file).unwrap();
  let res = spliting_text_by_space(txt).unwrap();
  Ok(res)
}

fn spliting_text_by_space(text: String) -> io::Result<Vec::<String>> {
  let space_char: &'static str = "\n";
  let mut res = Vec::new();
  for c in text.split(space_char) {
    let get_char = spliting_text_by_key(c).unwrap();
    res.push(get_char);
  }
  Ok(res)
}

fn spliting_text_by_key(text: &str) -> io::Result<String> {
  let equal_char: &'static str = "=";
  let mut res = Vec::new();
  for c in text.split(equal_char) {
    res.push(c);
  }

  let data_json = json!({
    res[0]: res[1],
  });

  Ok(data_json.to_string())
}
