# text-to-json

Convert text to json in rust.

## Install

```
[dependencies]
text-to-json = "0.1.0"
```

## How to use?

Please check [main.rs](./src/main.rs).

```rust
extern crate text_to_json;

use text_to_json::read;

fn main() {
  match read::to_json(String::from("example.txt")) {
    Ok(content) => println!("{:?}", content),
    Err(_) => panic!("Error"),
  }
}
```

Please check [example.txt](./example.txt).
```
// output
[Object({"title": String("hello world")}), Object({"description": String("lorem ipsume sit amet dolor")}), Obje
ct({"date": String("1211121")})]
```
