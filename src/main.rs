use std::convert::AsRef;
use std::fmt::Debug;

mod library_mod;

fn main() {
  // dereference
  let mut x: _ = 10; // _ i> i32
  let ref_x: &mut i32 = &mut x;
  *ref_x = 20;
  println!("x: {x}");

  let a: [i32; 6] = [10, 20, 30, 40, 50, 60];

  let slice: &[_] = &a[2..]; // _ i> i32
  println!("s: {slice:?}"); // :? is for debug.
  sample();
  type_inference();
  static_constant();
  banner();
  shadowing();
  stack_memory();
  copy_clone();
  life_time_1();
  life_time_2();
  library();
  iterator();
  into_iterator();
  sample_struct();
  new_type_idiom()
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
  println!("vv: {vv:?}")
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

fn library() {
  let mut library = library_mod::library::Library::new();

  let favorite_book = library_mod::library::Book::new("A sample book", 1985);
  println!(
    "Our favorite book {} should go in our library.",
    favorite_book
  );
  println!("1: {:p}", &library); // same pointer
  library.add_book(favorite_book);
  println!("3: {:p}", &library); // same pointer
  println!("Our library: {library:?}.");

  let favorite_book_2 = library_mod::library::Book::new("A sample book 2", 1986);
  library.add_book(favorite_book_2); // same pointer
  println!("Our library 2: {library:?}.");

  for book in library.books {
    println!("{book}");
  }
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
