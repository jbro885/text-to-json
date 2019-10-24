pub fn to_array(text: String) -> std::io::Result<Vec<String>> {
  let split_by_break: &'static str = "/n";
  let mut result = Vec::new();
  for t in text.split(split_by_break) {
    result.push(String::from(t));
  }

  Ok(result)
}
