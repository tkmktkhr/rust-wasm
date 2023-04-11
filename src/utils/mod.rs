pub mod tools {
  use std::rc::Rc;

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
    let mut v3 = vec![0, 0, 1, 2, 3, 4];

    v3.retain(|x| x % 2 == 0); // if the content is true, pick it up.
    println!("{v3:?}");

    // Remove consecutive duplicates.
    v3.dedup();
    println!("{v3:?}");
  }

  pub fn hash_map() {
    use std::collections::HashMap;

    let mut page_counts = HashMap::new();
    page_counts.insert("Adventures of Huckleberry Finn".to_string(), 207);
    page_counts.insert("Grimms' Fairy Tales".to_string(), 751);
    page_counts.insert("Pride and Prejudice".to_string(), 303);

    if !page_counts.contains_key("Les Misérables") {
      println!(
        "We know about {} books, but not Les Misérables.",
        page_counts.len()
      );
    }

    for book in ["Pride and Prejudice", "Alice's Adventure in Wonderland"] {
      match page_counts.get(book) {
        Some(count) => println!("{book}: {count} pages"),
        None => println!("{book} is unknown."),
      }
    }

    // Use the .entry() method to insert a value if nothing is found.
    for book in ["Pride and Prejudice", "Alice's Adventure in Wonderland"] {
      let page_count: &mut i32 = page_counts.entry(book.to_string()).or_insert(0);
      *page_count += 1;
    }

    println!("{page_counts:#?}");

    let page_counts = HashMap::from([
      ("Harry Potter and the Sorcerer's Stone".to_string(), 336),
      ("The Hunger Games".to_string(), 374),
    ]);
    println!("{page_counts:#?}");
  }

  // Box is an owned pointer to data on the heap. All values in Rust are stack allocated by default.
  #[derive(Debug)]
  enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
  }

  pub fn box_sample() {
    let five = Box::new(5);
    println!("five: {}", *five);

    let child1_list = Box::new(List::Nil);
    let child2_list = List::Cons(2, child1_list);
    let list: List<i32> = List::Cons(1, Box::new(child2_list));
    println!("{list:?}");
  }

  // Box enables (im)mutable borrows that are enforced at compile time.
  // RefCell enables (im)mutable borrows that are enforced at run time and will panic if it fails at runtime.
  pub fn rc() {
    let s: Rc<String> = Rc::new("Live".to_string());
    let t = s.clone();
    let u = s.clone();
    println!("{:p}, {:p}, {:p}", s, t, u); // same pointer
    println!("{:?}", Rc::strong_count(&s));
  }
}
