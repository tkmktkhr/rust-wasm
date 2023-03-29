
pub mod tools {
  pub fn tool() {
    println!("2");
  }

  pub fn option_result() {
    let nubmers = vec![10,20,30];
    let first: Option<&i8> = nubmers.first();
    println!("first: {first:?}");

    let idx: Result<usize, usize> = nubmers.binary_search(&10);
    println!("idx: {idx:?}")
  }
}