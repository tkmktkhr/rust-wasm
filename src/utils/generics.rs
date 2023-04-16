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
