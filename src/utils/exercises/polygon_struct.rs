pub struct Point {
  x: i32,
  y: i32,
}

impl Point {
  // add methods
  fn new(x: i32, y: i32) -> Self {
    Self { x, y }
  }

  fn magnitude(&self) -> f64 {
    let pow_x = self.x.pow(2) as f64;
    let pow_y = self.y.pow(2) as f64;
    (pow_x + pow_y).sqrt()
  }
}

pub struct Polygon {
  // add fields
}

impl Polygon {
  // add methods
}

pub struct Circle {
  // add fields
}

impl Circle {
  // add methods
}

pub enum Shape {
  Polygon(Polygon),
  Circle(Circle),
}

#[cfg(test)]
mod tests {
  use super::*;

  fn round_two_digits(x: f64) -> f64 {
    (x * 100.0).round() / 100.0 // Returns the nearest integer to self. If a value is half-way between two integers, round away from 0.0.
  }

  #[test]
  fn test_point_magnitude() {
      let p1 = Point::new(12, 13);
      assert_eq!(round_two_digits(p1.magnitude()), 17.69);
  }

}

pub fn polygon_struct() {
  println!("called polygon_struct().")
}
