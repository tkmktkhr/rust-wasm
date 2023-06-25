// It is good practice (and required by the Android Rust style guide) to write a comment for each unsafe block
// explaining how the code inside it satisfies the safety requirements of the unsafe operations it is doing.

// dereference a raw pointer
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

// mutable static variables
// safe
static HELLO_WORLD: &str = "Hello, world!";

pub fn safe_static() {
  println!("name is: {}", HELLO_WORLD);
}

static mut COUNTER: u32 = 0;

fn add_to_counter(inc: u32) {
  unsafe {
    COUNTER += inc;
  } // Potential data race!
}

// Using a mutable static is generally a bad idea
pub fn unsafe_static() {
  add_to_counter(42);

  unsafe {
    println!("COUNTER: {COUNTER}");
  } // Potential data race!
}

// Union
// Unions are like enums, but you need to track the active field yourself:
#[repr(C)]
union MyUnion {
  i: u8,
  b: bool,
}

pub fn unsafe_union() {
  let u = MyUnion { i: 42 };
  println!("int: {}", unsafe { u.i });
  println!("bool: {}", unsafe { u.b }); // Undefined behavior!
}

// unsafe function
pub fn unsafe_func() {
  let emojis = "🗻∈🌏";

  // Safe because the indices are in the correct order, within the bounds of
  // the string slice, and lie on UTF-8 sequence boundaries.
  unsafe {
    println!("emoji: {}", emojis.get_unchecked(0..4));
    println!("emoji: {}", emojis.get_unchecked(4..7));
    println!("emoji: {}", emojis.get_unchecked(7..11));
  }

  println!(
    "char count: {}",
    count_chars(unsafe { emojis.get_unchecked(0..7) })
  );

  // Not upholding the UTF-8 encoding requirement breaks memory safety!
  // println!("emoji: {}", unsafe { emojis.get_unchecked(0..3) });
  // println!("char count: {}", count_chars(unsafe { emojis.get_unchecked(0..3) }));
}

fn count_chars(s: &str) -> usize {
  s.chars().map(|_| 1).sum()
}

// unsafe swap func
/// Swaps the values pointed to by the given pointers.
///
/// # Safety
///
/// The pointers must be valid and properly aligned.
unsafe fn swap(a: *mut u8, b: *mut u8) {
  let temp = *a;
  *a = *b;
  *b = temp;
}

pub fn unsafe_swap() {
  let mut a = 42;
  let mut b = 66;
  // Safe because ...
  unsafe {
    swap(&mut a, &mut b);
  }
  println!("a: {}, b: {}", a, b);
}

// Calling External Code
extern "C" {
  fn abs(input: i32) -> i32;
}

pub fn unsafe_extern() {
  unsafe {
      // Undefined behavior if abs misbehaves.
      println!("Absolute value of -3 according to C: {}", abs(-3));
  }
}

// Unsafe trait