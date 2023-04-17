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
  println!("p.x = {}, p.y = {}", p.x(), p.y())
}
