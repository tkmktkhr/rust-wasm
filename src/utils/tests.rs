// unit test

pub fn first_word(text: &str) -> &str {
  let res = match text.find(' ') {
    Some(idx) => &text[..idx],
    None => &text,
  };
  println!("{:?}", &res);
  &res
}

// #[test]
// fn test_empty() {
//   assert_eq!(first_word(""), "");
// }

// #[test]
// fn test_single_word() {
//   assert_eq!(first_word("hello"), "Hello");
// }

// #[test]
// fn test_multiple_words() {
//   assert_eq!("Hello World", "Hello");
// }
