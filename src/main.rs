fn main() {
    // dereference
    let mut x: i32 = 10;
    let ref_x: &mut i32 = &mut x;
    *ref_x = 20;
    println!("x: {x}");

    let a: [i32; 6] = [10, 20, 30, 40, 50, 60];

    let slice: &[i32] = &a[2..];
    println!("s: {slice:?}"); // :? is for debug.
    sample();
}

fn sample() {
  let array = [10, 20, 30];
  print!("Iterating over array:");
  for n in array {
    print!(" {n}");
  }
  println!();

  print!("Iterating over range:");
  for i in 0..array.len() {
    print!(" {}", array[i])
  }
  println!();
}
