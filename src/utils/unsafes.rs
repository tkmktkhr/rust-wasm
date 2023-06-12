pub fn unsafe_sample() {
  let mut num = 5;

  let r1 = &mut num as *mut i32;
  let r2 = &num as *const i32;

  num = 14;
  println!("num is: {}", num);

  unsafe {
    println!("r1 is: {}", *r1);
    println!("r1 is: {:p}", r1);
    *r1 = 10;
    // *r2 = 20; // error: cannot assign to immutable borrowed content `*r2`.
    println!("r2 is: {}", *r2);
    println!("r2 is: {:p}", r2);
  }
}