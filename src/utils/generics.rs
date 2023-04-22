#[derive(Debug)]
struct Point<T> {
  x: T,
  y: T, // warning.
}

pub fn generics_sample() {
  let integer = Point { x: 5, y: 10 };
  let float = Point { x: 1.0, y: 4.0 };
  let test = Point {
    x: "x".to_string(),
    y: "y".to_string(),
  };
  let x = &test.x;
  println!("{x:?}");
  println!("{integer:?} and {float:?} and {test:?}");
}

// Methods
#[derive(Debug)]
struct PointA<T>(T, T);

impl<T> PointA<T> {
  fn x(&self) -> &T {
    &self.0
  }

  fn y(&self) -> &T {
    &self.1
  }

  fn set_x(&mut self, x: T) {
    self.0 = x
  }
}

pub fn generic_method() {
  let mut p = PointA(5, 10);
  println!("p.x = {}, p.y = {}", p.x(), p.y());
  p.set_x(8);
  println!("p.x = {}, p.y = {}", p.x(), p.y());

  // Monomorphization
  // let float = Some(5.0);
  // // means, as if you wrote
  // enum Option_f64 {
  //   Some(f64),
  //   None,
  // }
  // let float = Option_f64::Some(5.0);
}

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

// impl Pet for Cat {
//   fn name(&self) -> String {
//       String::from("The cat") // No name, cats won't respond to it anyway.
//   }
// }

// fn greet<P: Pet>(pet: &P) {
//   println!("Who's a cutie? {} is!", pet.name());
// }

// fn main() {
//   let fido = Dog { name: "Fido".into() };
//   greet(&fido);

//   let captain_floof = Cat;
//   greet(&captain_floof);
// }
