# text-to-json

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
["{\"title\":\"hello world\"}", "{\"description\":\"lorem ipsume sit amet dolor\"}", "{\"date\":\"1211121\"}"]
```
