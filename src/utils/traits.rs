// Trait
// Rust lets you abstract over types with traits. They’re similar to interfaces:

trait Pet {
  fn name(&self) -> String;
}

struct Dog {
  name: String,
}

struct Cat;

// pub fn trait() {

impl Pet for Dog {
  fn name(&self) -> String {
    self.name.clone()
  }
}

impl Pet for Cat {
  fn name(&self) -> String {
    String::from("The cat") // No name, cats won't respond to it anyway.
  }
}

fn greet<P: Pet>(pet: &P) {
  println!("Who's a cutie? {} is!", pet.name());
}

pub fn trait_sample() {
  let fido = Dog {
    name: "Fido".into(),
  };
  greet(&fido);

  let captain_floof = Cat;
  greet(&captain_floof);

  let pets: Vec<Box<dyn Pet>> = vec![
    Box::new(Dog {
      name: String::from("Fido"),
    }),
    Box::new(Cat),
  ];
  for pet in pets {
    println!("Hello {}!", pet.name());
  }

  // Types that implement a given trait may be of different sizes. This makes it impossible to have things like Vec<Pet> in the example above.
  // dyn Pet is a way to tell the compiler about a dynamically sized type that implements Pet.
  // In the example, pets holds fat pointers to objects that implement Pet. The fat pointer consists of two components, a pointer to the actual object and a pointer to the virtual method table for the Pet implementation of that particular object.
  println!(
    "{} {}",
    std::mem::size_of::<Dog>(),
    std::mem::size_of::<Cat>()
  );
  println!(
    "{} {}",
    std::mem::size_of::<&Dog>(),
    std::mem::size_of::<&Cat>()
  );
  println!("{}", std::mem::size_of::<&dyn Pet>());
  println!("{}", std::mem::size_of::<Box<dyn Pet>>());
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct Player {
  name: String,
  strength: u8,
  hit_points: u8,
}

pub fn trait_derive() {
  let p1 = Player::default();
  let p2 = p1.clone();
  println!(
    "Is {:?}\nequal to {:?}?\nThe answer is {}!",
    &p1,
    &p2,
    if p1 == p2 { "yes" } else { "no" }
  );
  println!("pointer1: {:p}, pointer2: {:p}", &p1, &p2)
}

// Default Methods
trait Equals {
  fn equal(&self, other: &Self) -> bool;
  fn not_equal(&self, other: &Self) -> bool {
    !self.equal(other)
  }
}

#[derive(Debug)]
struct Centimeter(i16);

impl Equals for Centimeter {
  fn equal(&self, other: &Self) -> bool {
    self.0 == other.0
  }
  // can be overwrite.
  // fn not_equal(&self, other: &Self) -> bool {
  //     !!self.equal(other)
  // }
}

pub fn trait_default_method() {
  let a = Centimeter(10);
  let b = Centimeter(20);
  println!("{a:?} equals {b:?}: {}", a.equal(&b));
  println!("{a:?} not_equals {b:?}: {}", a.not_equal(&b));
}

// trait bounds
// When working with generics, you often want to require the types to implement some trait, so that you can call this trait’s methods.
fn duplicate<T: Clone>(a: T) -> (T, T) {
  (a.clone(), a.clone())
}

// Syntactic sugar for:
//   fn add_42_millions<T: Into<i32>>(x: T) -> i32 {
//   fn add_42_millions(x: impl Into<i32>) -> i32 {
fn add_42_millions<T>(x: T) -> i32
where
  T: Into<i32>,
{
  x.into() + 42_000_000
}

pub fn trait_bounds() {
  let foo = String::from("foo");
  let pair = duplicate(foo); // String contains a Clone data type so foo can be arg.
  println!("{pair:?}");

  let many = add_42_millions(42_i8);
  println!("{many:?}");
  let many_more = add_42_millions(10_000_000);
  println!("{many_more:?}");
}

// impl trait
use std::fmt::Display;

fn get_x(name: impl Display) -> impl Display {
  // For a return type, it means that the return type is some concrete type that implements the trait, without naming the type. This can be useful when you don’t want to expose the concrete type in a public API.
  format!("HEllo {name}")
}

pub fn trait_impl() {
  let x = get_x(1);
  println!("{x}");

  // NG: `traits::Dog` doesn't implement `std::fmt::Display`
  // let fido = Dog {
  //   name: "Fido".into(),
  // };
  // let dog = get_x(fido);
  // println!("{dog}");
}

// Important traits

// Iterators

struct Fibonacci {
  curr: u32,
  next: u32,
}

impl Iterator for Fibonacci {
  type Item = u32;

  fn next(&mut self) -> Option<Self::Item> {
    let new_next = self.curr + self.next;
    self.curr = self.next;
    self.next = new_next;
    Some(self.curr)
  }
}

pub fn iterators() {
  let fib = Fibonacci { curr: 0, next: 1 };
  for (i, n) in fib.enumerate().take(6) {
    // enumerate returns tuple.
    println!("fib({i}): {n}");
  }
}

// FromIterator
pub fn from_iterator() {
  let primes = vec![2, 3, 5, 7];
  let prime_squares = primes
    .into_iter()
    .map(|prime| prime * prime)
    .collect::<Vec<_>>();
  println!("{prime_squares:?}");
}

// From
pub fn from_sample() {
  let s = String::from("hello");
  let addr = std::net::Ipv4Addr::from([127, 0, 0, 1]);
  let one = i16::from(true);
  let bigger = i32::from(123i16);
  println!("from: {s}, {addr}, {one}, {bigger}");
}

// Into
pub fn into_sample() {
  let s: String = "hello".into();
  let addr: std::net::Ipv4Addr = [127, 0, 0, 1].into();
  let one: i16 = true.into();
  let bigger: i32 = 123i16.into();
  println!("into: {s}, {addr}, {one}, {bigger}");
}

// Read and Write
use std::io::{BufRead, BufReader, Read, Result, Write};

fn count_lines<R: Read>(reader: R) -> usize {
  let buf_reader = BufReader::new(reader);
  buf_reader.lines().count()
}

pub fn read_sample() -> Result<()> {
  let slice: &[u8] = b"foo\nbar\nbaz\n";
  println!("lines in slice: {}", count_lines(slice));

  let file = std::fs::File::open(std::env::current_exe()?)?;
  println!("lines in file: {}", count_lines(file));
  Ok(())
}

// write
// use std::io::{Result, Write};

fn log<W: Write>(writer: &mut W, msg: &str) -> Result<()> {
  println!("as bytes: {:?}", msg.as_bytes());
  println!("as bytes pointer: {:p}", msg.as_bytes());
  writer.write_all(msg.as_bytes())?;
  writer.write_all("\n".as_bytes())
}

pub fn write_sample() -> Result<()> {
  let mut buffer = Vec::new();
  log(&mut buffer, "Hello")?;
  log(&mut buffer, "Worldあ")?;
  println!("Logged: {:?}", buffer);
  Ok(())
}

// drop trait
struct Droppable {
  name: &'static str,
}

impl Drop for Droppable {
  fn drop(&mut self) {
    println!("Dropping {}", self.name)
  }
}

pub fn drop_sample() {
  let a = Droppable { name: "a" };
  {
    let _b = Droppable { name: "b" };
    {
      let _c = Droppable { name: "c" };
      let _d = Droppable { name: "d" };
      println!("Exiting block B");
    }
    println!("Exiting block A");
  }
  drop(a);
  // a.drop(); // This method is called implicitly when the value goes out of scope, and cannot be called explicitly (this is compiler error E0040).
  println!("Exiting main");
}

// Default

#[derive(Debug, Default)]
struct Derived {
  x: u32,
  y: String,
  z: Implemented,
}

#[derive(Debug)]
struct Implemented(String);

impl Default for Implemented {
  fn default() -> Self {
    Self("John Smith".into())
  }
}

// Standard Rust types often implement Default with reasonable values (e.g. 0, "", etc).
pub fn default_sample() {
  let default_struct: Derived = Default::default();
  println!("{default_struct:#?}");

  let almost_default_struct = Derived {
    y: " Y is set!".into(),
    ..Default::default()
  };
  println!("{almost_default_struct:#?}");

  let nothing: Option<Derived> = None;
  println!("{:#?}", nothing.unwrap_or_default());
}

// Add, Mul, ...
#[derive(Debug, Copy, Clone)]
struct Point {
  x: i32,
  y: i32,
}

impl std::ops::Add for Point {
  type Output = Self; // alias.

  fn add(self, other: Self) -> Self {
    // work if Self is Point
    Self {
      // work if Self is Point
      x: self.x + other.x,
      y: self.y + other.y,
    }
  }
}

pub fn add_sample() {
  let p1 = Point { x: 10, y: 20 };
  let p2 = Point { x: 100, y: 200 };
  let plus_res = p1 + p2;
  println!("{:?} + {:?} = {:?}", p1, p2, p1 + p2);
  println!("pointer 1: {:p}", &p1);
  println!("pointer 2: {:p}", &p2);
  println!("pointer 1+2: {:p},{:?}", &plus_res, plus_res);
}

// Closures
fn apply_with_log(func: impl FnOnce(i32) -> i32, input: i32) -> i32 {
  println!("Calling function on {input}");
  func(input)
}

pub fn closure_sample() {
  // closure must be Fn(&self), FnMut(&mut self), FnOnce(self) instance.
  // FnMut and FnOnce extends Fn trait.
  let add_3 = |x| x + 3; // |x| {x + 3} is also fine.
  let mul_5 = |x| x + 5;

  println!("add_3: {}", apply_with_log(add_3, 10));
  println!("mul_5: {}", apply_with_log(mul_5, 20));

  let outer_var = 42;
  let closure_annotated = |i: i32| -> i32 { i + outer_var };
  println!("closure_annotated: {}", closure_annotated(1));
}

pub mod sample_trait {
  use reqwest::StatusCode;

  #[derive(Debug, Clone, Default)]
  pub struct User {
    id: u32,
    name: String,
  }

  #[derive(Debug, Clone, Copy, Default)]
  pub struct Json<T>(pub T);

  // #[derive(Debug, Clone, Default)]
  // struct CustomError { msg: String }

  // #[derive(Debug)]
  // pub enum CtmError {
  //   NotFoundError { msg: String },
  //   InternalError { msg: String },
  //   // NotFoundError(NotFoundError),
  //   // InternalError(InternalError),
  // }

  // type ApiResponse<T> = (StatusCode, Json<JsonResponse<T>>);

  // #[derive(Debug)]
  // pub enum JsonResponse<T> {
  //   OkResponse(T),
  //   CustomError(CtmError),
  // }

  pub trait ResponseJson<T> {
    fn response(code: StatusCode, body: T) -> (StatusCode, Json<T>);
  }

  impl<T, U> ResponseJson<T> for U
  where
    T: std::fmt::Debug,
  {
    fn response(code: StatusCode, body: T) -> (StatusCode, Json<T>) {
      (code, Json(body))
    }
  }

  #[derive(Debug)]
  pub struct NotFoundError {
    msg: String,
  }

  #[derive(Debug)]
  pub struct InternalError {
    msg: String,
  }

  fn res(
    bool: bool,
  ) -> impl ResponseJson<User> + ResponseJson<NotFoundError> + ResponseJson<InternalError> {
    let user = User {
      id: 1,
      name: "foo".to_string(),
    };
    let err_res = NotFoundError {
      msg: "not found".to_string(),
    };

    let backend = match bool {
      true => Ok(user),
      false => Err(err_res),
    };
    let response = match backend {
      Ok(user) => Ok::<(StatusCode, User), _>((StatusCode::OK, user)),
      Err(err) => Err::<_, (StatusCode, NotFoundError)>((StatusCode::NOT_FOUND, err)),
    };

    println!("response : {:?}", response);
    response
  }

  pub fn main(
  ) -> impl ResponseJson<User> + ResponseJson<NotFoundError> + ResponseJson<InternalError> {
    let bool = true;
    // let bool = false;
    let obj = res(bool);
    obj
  }
}
