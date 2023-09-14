#[derive(Debug, PartialEq, Copy, Clone, Eq)]
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
    // (self - other).magnitude()
  }
}

impl std::ops::Add for Point {
  type Output = Self;

  fn add(self, other: Self) -> Self::Output {
    Self {
      x: self.x + other.x,
      y: self.y + other.y,
    }
  }
}

impl std::ops::Sub for Point {
  type Output = Self;

  fn sub(self, other: Self) -> Self::Output {
    Self {
      x: self.x - other.x,
      y: self.y - other.y,
    }
  }
}

pub struct Polygon {
  // add fields
  points: Vec<Point>,
}

impl Polygon {
  fn new() -> Self {
    Polygon { points: Vec::new() }
  }

  fn add_point(&mut self, point: Point) {
    self.points.push(point)
  }

  fn left_most_point(&self) -> Option<Point> {
    self.points.iter().min_by_key(|p| p.x).copied()
  }

  fn iter(&self) -> impl Iterator<Item = &Point> {
    self.points.iter()
  }
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

  #[test]
  fn test_point_add() {
    let p1 = Point::new(16, 16);
    let p2 = p1 + Point::new(-4, 3);
    assert_eq!(p2, Point::new(12, 19));
  }

  #[test]
  fn test_polygon_left_most_point() {
    let p1 = Point::new(12, 13);
    let p2 = Point::new(16, 16);

    let mut poly = Polygon::new();
    assert_eq!(poly.left_most_point(), None);
    poly.add_point(p1);
    poly.add_point(p2);
    assert_eq!(poly.left_most_point(), Some(p1));
  }

  #[test]
  fn test_polygon_iter() {
    let p1 = Point::new(12, 13);
    let p2 = Point::new(16, 16);

    let mut poly = Polygon::new();
    poly.add_point(p1);
    poly.add_point(p2);

    // let points = poly.points.iter().cloned().collect::<Vec<_>>();
    let points = poly.iter().cloned().collect::<Vec<_>>();
    assert_eq!(points, vec![Point::new(12, 13), Point::new(16, 16)]);
  }
}

pub fn polygon_struct() {
  println!("called polygon_struct().")
}
