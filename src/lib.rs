pub mod read;

#[cfg(test)]
mod tests {
    use super::*;
    use read::{to_json, text};

    #[test]
    fn test_read_text() {
        let txt = text(String::from("example.txt")).unwrap();
        assert_eq!(txt, "title=hello world\ndescription=lorem ipsume sit amet dolor\ndate=1211121")
    }

    #[test]
    fn test_text_to_json() {
        let content = to_json("example.txt".to_string()).unwrap();
        println!("{:?}", content);
        assert_eq!(content, ["{\"title\":\"hello world\"}", "{\"description\":\"lorem ipsume sit amet dolor\"}", "{\"date\":\"1211121\"}"])
    }
}
