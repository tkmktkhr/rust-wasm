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
    let pow_x = self.x.pow(2);
    let pow_y = self.y.pow(2);
    f64::from(pow_x + pow_y).sqrt()
    // let pow_x = self.x.pow(2) as f64;
    // let pow_y = self.y.pow(2) as f64;
    // (pow_x + pow_y).sqrt()
  }

  fn dist(&self, p: Point) -> f64 {
    let dist_x = (p.x - self.x).abs() as f64;
    let dist_y = (p.y - self.y).abs() as f64;
    (dist_x.powi(2) + dist_y.powi(2)).sqrt()
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

  #[test]
  fn test_point_dist() {
    let p1 = Point::new(10, 10);
    let p2 = Point::new(14, 13);
    assert_eq!(round_two_digits(p1.dist(p2)), 5.00);
  }
}

pub fn polygon_struct() {
  println!("called polygon_struct().")
}
