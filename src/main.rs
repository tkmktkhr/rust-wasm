use race::fn_race;
use std::convert::AsRef;
use std::fmt::Debug;

mod race;
mod utils;

use utils::exercises::storing_book::library;

fn main() {
  // dereference
  let mut x: _ = 10; // _ i> i32
  let ref_x: &mut i32 = &mut x;
  *ref_x = 20;
  println!("x: {x}");

  let a: [i32; 6] = [10, 20, 30, 40, 50, 60];

  let slice: &[_] = &a[2..]; // _ i> i32

  println!("s: {slice:?}"); // :? is for debug.
  // utils::traits::sample_trait::main();
  // if you run functions, remove comment out.
  // sample();
  // type_inference();
  // str_string();
  // static_constant();
  // banner();
  // shadowing();
  // stack_memory();
  // copy_clone();
  // life_time_1();
  // life_time_2();
  // library();
  // iterator();
  // into_iterator();
  // sample_struct();
  // new_type_idiom();
  // create_default();
  // sample_basic_enum();
  // variant_payloads();
  // enum_size();
  // method();
  // fn_race();
  // pattern_matching();
  // destructure_enum();
  // destructure_struct();
  // destructure_tuple();
  // closure();
  // destructure_array();
  // match_guards();
  // block();
  // if_let();
  // endless_loop();
  // break_continue();

  // utils::tools::option_result();
  // utils::tools::string_sample();
  // utils::tools::vec_sample();
  // utils::tools::hash_map();
  // utils::tools::box_sample();
  // utils::tools::rc();

  // utils::generics::generics_sample();
  // utils::generics::generic_method();
  // utils::generics::trait_sample();

  // utils::traits::trait_sample();
  // utils::traits::trait_derive();
  // utils::traits::trait_default_method();
  // utils::traits::trait_bounds();
  // utils::traits::trait_impl();
  // utils::traits::iterators();
  // utils::traits::from_iterator();
  // utils::traits::from_sample();
  // utils::traits::into_sample();
  // let _ = utils::traits::read_sample();
  // let _ = utils::traits::write_sample();
  // utils::traits::drop_sample();
  // utils::traits::default_sample();
  // utils::traits::add_sample();
  // utils::traits::closure_sample();

  // utils::results::file_read1();
  // utils::results::file_read2();

  // utils::errors::add_context_error();

  // utils::tests::first_word("abc def ghi");

  // utils::unsafes::unsafe_sample();
  // utils::unsafes::safe_static();
  // utils::unsafes::unsafe_static();
  // utils::unsafes::unsafe_union();
  // utils::unsafes::unsafe_func();
  // utils::unsafes::unsafe_swap();
  // utils::unsafes::unsafe_extern();

  // utils::concurrencies::conc_thread();
  // utils::concurrencies::normal_scoped_thread();
  // utils::concurrencies::channel_thread();
  // utils::concurrencies::unbounded_channel();
  // utils::concurrencies::bounded_channel();
  // utils::concurrencies::shared_state::arc();
  // utils::concurrencies::shared_state::mutex();
  // utils::concurrencies::shared_state::arc_mutex();
  // utils::concurrencies::async_sample();
  // utils::concurrencies::tokio_sample();
  // utils::concurrencies::sample_async_channels::async_channels();
  // utils::concurrencies::sample_join::join();
  // utils::concurrencies::sample_select::select();
  // utils::concurrencies::sample_block_executor::blocking_executor();
  // utils::concurrencies::sample_pin::pin();
  // let _ = utils::concurrencies::sample_cancellation::cancellation();

  // exercises
  // utils::exercises::array_for_loops::array_for_loops::output();
  // utils::exercises::simple_gui_library::simple_gui_library();
  // utils::exercises::polygon_struct::polygon_struct();
  // utils::exercises::dining_philosophers::dining_philosophers();
  utils::exercises::multi_threaded_link_checker::link_checker();
}

// Array, Vec, Slice
fn sample() {
  // matrix practice
  let matrix = [
    [101, 102, 103], // <-- the comment makes rustfmt add a newline
    [201, 202, 203],
    [301, 302, 303],
  ];

  println!("matrix:");
  pretty_print(&matrix);

  let transposed = transpose(&matrix);
  println!("transposed:");
  pretty_print(&transposed);

  // advanced matrix practice
  pretty_print(&[&[1, 2, 3], &[4, 5, 6], &[7, 8, 9]]);
  pretty_print([["a", "b"], ["c", "d"]]);
  pretty_print(vec![vec![1, 2], vec![3, 4]])
}

fn transpose(matrix: &[[i32; 3]; 3]) -> [[i32; 3]; 3] {
  let mut result = [[0; 3]; 3];
  for i in 0..result[0].len() {
    for j in 0..result[0].len() {
      result[j][i] = matrix[i][j];
    }
  }
  return result;
}

fn pretty_print<T, Line, Matrix>(matrix: Matrix)
where
  // easier to read Generics.
  T: Debug,
  // A line references a slice of items
  Line: AsRef<[T]>,
  // A matrix references a slice of lines
  Matrix: AsRef<[Line]>,
{
  for row in matrix.as_ref() {
    println!("{:?}", row.as_ref())
  }
}

#[test]
fn test_transpose() {
  let matrix = [
    [101, 102, 103], //
    [201, 202, 203],
    [301, 302, 303],
  ];
  let transposed = transpose(&matrix);

  assert_eq!(
    transposed,
    [
      [101, 201, 301], //
      [102, 202, 302],
      [103, 203, 303],
    ]
  )
}

fn type_inference() {
  let mut v = Vec::new();
  let abc = "abc";
  let def = "def";
  v.push((String::from(abc), false));
  v.push((String::from(def), true));
  println!(" v: {v:?}");

  // pointer
  println!("pointer1: {:p}", &v[0].0);
  println!("pointer2: {:p}", &v[0].1);
  println!("pointer3: {:p}", &v[0]);
  println!("pointer3: {:p}", &v);

  // let vv = v.iter().collect::<std::collections::HashSet<&(i32, bool)>>(); // don't need to write this.
  let vv = v.iter().collect::<std::collections::HashSet<_>>();
  println!("vv: {vv:?}");

  let not_cloned = vec![1, 2, 3];
  let cloned = &not_cloned.iter().copied();
  println!("{:?}", not_cloned);
  println!("pointer {:p}", not_cloned.as_ptr());
  println!("{:?}", cloned);
  println!("pointer {:p}", cloned);

  println!("---------vector")
}

fn str_string() {
  println!("str_string---------------------------------------");
  let str: &str = "test";
  let string: String = "test1".to_string();
  println!("str: {}", str);
  println!("{:p}", str);

  println!("str.to_owned: {}", str.to_owned());
  println!("{:p}", str.to_owned().as_ptr());

  println!("str.to_string: {}", str.to_string());
  println!("{:p}", str.to_string().as_ptr());

  println!("string: {}", string);
  println!("{:p}", string.as_ptr());

  println!("string.as_str: {}", string.as_str());
  println!("{:p}", string.as_str());

  let s = "a b c \nd e f g h i".to_owned();
  let line = s.lines();
  println!("{:?}", line);
  let mapped = line.map(|line| line.chars().count()).max().unwrap_or(0);
  println!("{:?}", mapped);
  println!("{:?}", s.len());
  println!("{:^1$}", str.to_owned(), str.len());
}

const DIGEST_SIZE: usize = 3; // usize is u32 or u64. // inlined upon use.
const ZERO: Option<u8> = Some(42);

fn compute_digest(test_str: &str) -> [u8; DIGEST_SIZE] {
  let mut digest = [ZERO.unwrap_or(0); DIGEST_SIZE];
  for (idx, &b) in test_str.as_bytes().iter().enumerate() {
    digest[idx % DIGEST_SIZE] = digest[idx % DIGEST_SIZE].wrapping_add(b);
  }
  digest
}

fn static_constant() {
  let digest = compute_digest("Hello");
  println!("Digest: {digest:?}");
}

static BANNER: &str = "Welcome to RustOS 3.14"; // not inlined upon use and have an actual associated memory location.

fn banner() {
  println!("{BANNER}");
}

fn shadowing() {
  let a = 10;
  println!("before: {a}");

  {
    let a = "hello";
    println!("inner scope: {a}");

    let a = true;
    println!("shadowed in inner scope: {a}");
  }

  println!("after: {a}");
}

fn stack_memory() {
  let mut s1 = String::from("Hello");
  s1.push(' ');
  s1.push_str("world");
  // DON'T DO THIS AT HOME! For educational purposes only.
  // String provides no guarantees about its layout, so this could lead to
  // undefined behavior.
  unsafe {
    let (capacity, ptr, len): (usize, usize, usize) = std::mem::transmute(s1);
    println!("ptr = {ptr:#x}, len = {len}, capacity = {capacity}");
  }
  let a = String::from("Hello");
  let b = a.clone();
  println!("{:?}, {:p}", a, &a);
  println!("{:?}, {:p}", b, &b);
}

#[derive(Copy, Clone, Debug)]
struct Point(i32, i32); // Tuple struct

fn copy_clone() {
  let x = 42;
  let y = x; // certain types have copy traits.
  println!("x: {x}");
  println!("y: {y}");

  let p1 = Point(3, 4);
  let p2 = p1;
  println!("p1: {p1:?}, {:p}", &p1);
  println!("p2: {p2:?}, {:p}", &p2);
}

// already exists in line 169.
// #[derive(Debug)]
// struct Point(i32, i32);

fn left_most<'a>(p1: &'a Point, p2: &'a Point) -> &'a Point {
  if p1.0 > p2.0 {
    p1
  } else {
    p2
  }
}

fn life_time_1() {
  let p1: Point = Point(10, 10);
  let p2: Point = Point(20, 20);
  let p3: &Point = left_most(&p1, &p2);
  println!("left-most point: {:?}", p3);
  println!("pointer: {:p},{:p},{:p}", &p1, &p2, &p3);
}

#[derive(Debug)]
struct Highlight<'doc>(&'doc str);

fn erase(text: String) {
  println!("Bye {text}!");
}

fn life_time_2() {
  let text = String::from("The quick brown fox jumps over the lazy dog.");
  let fox = Highlight(&text[4..19]);
  let dog = Highlight(&text[35..43]);
  // erase(text); // not ok
  println!("{fox:?}");
  println!("{dog:?}");
  erase(text) // ok
}

fn library() -> () {
  println!("start library---------------------------------------");
  let mut library = library::Library::new();
  println!("0 books?: {}", &library.get_is_empty());
  let favorite_book = library::Book::new("A sample book", 1985);
  println!(
    "Our favorite book {} should go in our library.",
    favorite_book
  );
  println!("1: {:p}", &library); // same pointer
  library.add_book(favorite_book);
  println!("3: {:p}", &library); // same pointer
  println!("Our library: {library:?}.");

  let book0 = library::Book::new("A sample book 1", 1912);
  library.add_book(book0); // same pointer
  let favorite_book_2 = library::Book::new("A sample book 2", 1986);
  library.add_book(favorite_book_2); // same pointer
  println!("Our library 2: {library:?}.");

  for book in &library.books {
    println!("{book}");
  }
  println!("0 books?: {}", &library.get_is_empty());
  println!("Return the length of `self.books`: {}", &library.get_len());
  println!("{:?}", &library.get_oldest_book());
  let _ = &library.print_books();
}

fn iterator() {
  let v: Vec<i8> = vec![10, 20, 30];
  let mut iter = v.iter();

  let v0: Option<&i8> = iter.next(); // return reference.
  println!("v0: {v0:?}");
  println!("v[1]: {:?}", iter.next());
  println!("v[2]: {:?}", iter.next());
  println!("v[3]: {:?}", iter.next())
}

fn into_iterator() {
  let v: Vec<String> = vec![String::from("foo"), String::from("bar")];
  let mut iter = v.into_iter(); // return value.
  let v0: Option<String> = iter.next();
  println!("v0: {v0:?}");
  println!("v1: {:?}", iter.next());
  println!("v2: {:?}", iter.next());
}
#[derive(Debug)]
struct Person {
  name: String,
  age: u8,
}

fn sample_struct() {
  let mut peter = Person {
    name: String::from("peter"),
    age: 27,
  };
  println!("{} is {} years old", peter.name, peter.age);

  peter.age = 28;
  println!("{} is {} years old", peter.name, peter.age);

  let jackie = Person {
    name: String::from("jackie"),
    ..peter
  };
  println!("{} is {} years old", jackie.name, jackie.age);
}

struct Years(i64);
struct Days(i64);

impl Years {
  pub fn to_days(&self) -> Days {
    Days(self.0 * 365)
  }
}

impl Days {
  pub fn to_years(&self) -> Years {
    Years(self.0 / 365)
  }
}

fn old_enough(age: &Years) -> bool {
  age.0 >= 18
}

fn new_type_idiom() {
  let age = Years(5);
  let age_days = age.to_days();

  println!("Old enough {}", old_enough(&age));
  println!("Old enough {}", old_enough(&age_days.to_years()));
}

// This is unnecessary for the bellow.
trait Default {
  fn default() -> Self;
}

impl Default for Person {
  fn default() -> Self {
    Person {
      name: "Default_Name".to_string(),
      age: 0,
    }
  }
}

fn create_default() {
  let tmp = Person {
    ..Default::default()
  };
  println!("{tmp:#?}");

  let tmp = Person {
    name: "sample_name".to_string(),
    ..Default::default()
  };
  println!("{tmp:#?}");
}

#[derive(Debug)]
enum CoinFlip {
  Heads, // 0
  Tails, // 1
}

fn flip_coin() -> CoinFlip {
  let random_number = 4; // should be random number.
  if random_number % 2 == 0 {
    return CoinFlip::Heads;
  } else {
    return CoinFlip::Tails;
  }
}

fn sample_basic_enum() {
  println!("{:?}", flip_coin())
}

#[derive(Debug)]
enum WebEvent {
  PageLoad,                 // Variant without payload
  KeyPress(char),           // Tuple struct variant
  Click { x: i64, y: i64 }, // Full struct variant
}

// #[rustfmt::skip]
fn inspect(event: WebEvent) {
  println!("{event:#?}");
  match event {
    WebEvent::PageLoad => println!("page loaded"),
    WebEvent::KeyPress(c) => println!("pressed '{c}'"),
    WebEvent::Click { x, y } => println!("clicked at x={x}, y={y}"),
  }
}

fn variant_payloads() {
  let load = WebEvent::PageLoad;
  let press = WebEvent::KeyPress('x');
  let click = WebEvent::Click { x: 20, y: 80 };

  inspect(load);
  inspect(press);
  inspect(click);
}

use std::mem::{align_of, size_of};

macro_rules! dbg_size {
  ($t:ty) => {
    println!(
      "{}: size {} bytes, align: {} bytes",
      stringify!($t),
      size_of::<$t>(),
      align_of::<$t>()
    )
  };
}

enum FooEnum {
  A,
  B,
}

#[repr(u32)]
enum Bar {
  A,
  B = 1000,
  C,
}

fn enum_size() {
  dbg_size!(FooEnum);
  println!("A: {}", Bar::A as u32);
  println!("B: {}", Bar::B as u32);
  println!("C: {}", Bar::C as u32);
}

impl Person {
  fn say_hello(&self) {
    println!("Hello, my name is {}", self.name)
  }
}

fn method() {
  let peter = Person {
    name: String::from("Peter"),
    age: 27,
  };
  peter.say_hello();
}

fn pattern_matching() {
  let input = '9';

  match input {
    'q' => println!("Quitting"),
    'a' | 's' | 'w' | 'd' => println!("Moving around"),
    '0'..='9' => println!("Number input"),
    _ => println!("Something else"),
  }
}

enum Result {
  Ok(i32),
  Err(String),
}

fn divide_in_two(n: i32) -> Result {
  if n % 2 == 0 {
    Result::Ok(n / 2)
  } else {
    Result::Err(format!("cannot divide {n} into two equal parts"))
  }
}

fn destructure_enum() {
  let n = 101;
  match divide_in_two(n) {
    Result::Ok(half) => println!("{n} divided in two is {half}"),
    Result::Err(msg) => println!("sorry, an error happened: {msg}"),
  }
}

struct Foo {
  x: (u32, u32),
  y: u32,
}

fn destructure_struct() {
  let foo = Foo { x: (1, 2), y: 4 };
  let res = match foo {
    Foo { x: (1, b), y } => {
      println!("x.0 = 1, b = {b}, y = {y}");
      y
    }
    Foo { y: 2, x: i } => {
      println!("y = 2, x = {i:?}");
      i.1
    }
    Foo { y, .. } => {
      println!("y = {y}, other fields were ignored");
      y
    }
  };
  println!("result is {res:?}")
}

fn destructure_tuple() {
  let triple = [0, -2, -3];
  println!("Tell me about {triple:?}");

  match triple {
    [0, y, z] => println!("First is 0, y = {y}, and z = {z}"),
    [1, ..] => println!("First is 1 and the rest were ignored"),
    _ => println!("All elements were ignored"),
  }
}
// multi thread
fn closure() {
  const N_THREADS: usize = 3;

  let series_range = 0..30;
  let add = 1;
  // let chunks: Vec<_> = (0..3).map(|ii| (0..30).skip(ii).step_by(3)).collect();
  let chunks = (0..N_THREADS).map(|ii| series_range.clone().skip(ii).step_by(N_THREADS)); // executed until it needs to be.
  println!("{chunks:?}");
  // println!("{:?}", chunks.next()); // first element of array is skipped.

  // Iterator
  let a = [1, 2, 3];
  let mut a_iter = a.iter();
  println!("{a_iter:?}");
  let a_iter_next = a_iter.next();
  println!("{a_iter_next:?}"); // Some(1)

  let handles: Vec<_> = chunks
    .map(|vv| {
      std::thread::spawn(move || {
        vv.for_each(|nn| print! {"{}+{}={}; ", nn, add, nn + add});
      })
    })
    .collect();

  handles.into_iter().for_each(|hh| hh.join().unwrap());
  println!("---END");
}

fn destructure_array() {
  let triple = [2, -2, 3, 1];
  println!("Tell me about {triple:?}");

  match triple {
    [0, y, z, ..] => println!("First is 0, y = {y}, and z = {z}"),
    [1, ..] => println!("First is 1 and the rest were ignored"),
    // [.., 1, ..] => println!("Middle is 1 and the rest were ignored"), // can only be used once per slice pattern
    [.., 1] => println!("Last is 1 and the rest were ignored"),
    _ => println!("All elements were ignored"),
  }
}

fn match_guards() {
  let pair = (2, -2);
  println!("Tell me about {pair:?}");

  match pair {
    (x, y) if x == y => println!("These are twins"),
    (x, y) if x + y == 0 => println!("Antimatter, kaboom!"),
    (x, _) if x % 2 == 1 => println!("The first one is odd"),
    _ => println!("No correlation..."),
  }
}

fn block() {
  let x = {
    let y = 10;
    println!("y: {y}");
    let z = {
      let w = { 3 + 4 };
      println!("y: {y}");
      y * w
    };
    z - y
  };
  println!("x: {x}");
}

fn if_let() {
  // All have type `Option<i32>`
  let number = Some(7);
  // let letter: Option<i32> = None;
  let letter: Option<i32> = Some(10);
  let emoticon: Option<i32> = None;

  // The `if let` construct reads: "if `let` destructures `number` into `Some(i)`, evaluate the block (`{}`).
  if let Some(i) = number {
    println!("Matched {:?}", i);
  }

  // If you need to specify a failure, use an else:
  if let Some(i) = letter {
    println!("Matched {:?}!", i);
    // letter // try adding a return type: `-> Option<i32> `
  } else {
    // Destructure failed. Change to the failure case.
    println!("Didn't match a number. Let's go with a letter!");
    // Some(1)// try adding a return type: `-> Option<i32> `
  }

  // // Provide an altered failing condition.
  let i_like_letters = false;

  if let Some(i) = emoticon {
    println!("Matched {:?}!", i);
  // Destructure failed. Evaluate an `else if` condition to see if the alternate failure branch should be taken:
  } else if i_like_letters {
    println!("Didn't match a number. Let's go with a letter!");
  } else {
    // The condition evaluated false. This branch is the default:
    println!("I don't like letters. Let's go with an emoticon :)!");
  }
}

fn endless_loop() {
  let mut x = 10;

  loop {
    x = if x % 2 == 0 { x / 2 } else { 3 * x + 1 };
    if x == 1 {
      break;
    }
  }

  println!("Final x: {x}");
}

fn break_continue() {
  let v = vec![10, 20, 30];
  let mut iter = v.into_iter();
  // 'outer -> label
  'outer: while let Some(x) = iter.next() {
    println!("x: {x}");
    let mut i = 0;
    while i < x {
      println!("x: {x}, i: {i}");
      i += 1;
      if i == 3 {
        break 'outer;
      }
    }
  }
}
