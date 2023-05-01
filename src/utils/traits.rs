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
  println!("{s}, {addr}, {one}, {bigger}");
}
