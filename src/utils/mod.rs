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
    // String::len returns the size of the String in bytes (which can be different from its length in characters).
    println!("s1: len = {}, capacity = {}", s1.len(), s1.capacity());

    let mut s2 = String::with_capacity(s1.len() + 1);
    s2.push_str(&s1); // push_str: add string literal.
    s2.push('!'); // push: add only 1 character.
    // s2.push('?'); // s2: len = 7, capacity = 12
    println!("s2: len = {}, capacity = {}", s2.len(), s2.capacity());

    let s3 = String::from("🇨🇭");
    println!(
      "s3: len = {}, number of chars = {}",
      s3.len(),
      s3.chars().count()
    );
  }

  pub fn vec_sample() {
    // Vec is a type of collection, along with String and HashMap.
    // The data it contains is stored on the heap. This means the amount of data doesn’t need to be known at compile time. It can grow or shrink at runtime.
    let mut v1 = Vec::new();
    v1.push(42);
    v1.push(43);
    println!("v1: len = {}, capacity = {}", v1.len(), v1.capacity());

    let mut v2 = Vec::with_capacity(v1.len() + 1);
    v2.extend(v1.iter());
    v2.push(9999);
    println!("v2: len = {}, capacity = {}", v2.len(), v2.capacity());

    // Canonical macro to initialize a vector with elements.
    let mut v3 = vec![0,0,1,2,3,4];

    v3.retain(|x| x % 2 == 0); // if the content is true, pick it up.
    println!("{v3:?}");

    // Remove consecutive duplicates.
    v3.dedup();
    println!("{v3:?}");
  }
}
