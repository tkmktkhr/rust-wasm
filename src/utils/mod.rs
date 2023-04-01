pub mod tools {
  pub fn tool() {
    println!("2");
  }

  pub fn option_result() {
    let nubmers = vec![10, 20, 30];
    let first: Option<&i8> = nubmers.first();
    println!("first: {first:?}");

    let idx: Result<usize, usize> = nubmers.binary_search(&10);
    println!("idx: {idx:?}")
  }

  // String is the standard heap-allocated growable UTF-8 string buffer:
  pub fn string_sample() {
    let mut s1 = String::new();
    s1.push_str("Hello");
    // String::len returns the size of the String in bytes
    println!("s1: len = {}, capacity = {}", s1.len(), s1.capacity());

    let mut s2 = String::with_capacity(s1.len() + 1);
    s2.push_str(&s1);
    s2.push('!');
    println!("s2: len = {}, capacity = {}", s2.len(), s2.capacity());

    let s3 = String::from("🇨🇭");
    println!(
      "s3: len = {}, number of chars = {}",
      s3.len(),
      s3.chars().count()
    );
  }
}
