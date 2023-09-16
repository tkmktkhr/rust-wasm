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

  fn length(&self) -> f64 {
    let mut res = 0.0;
    if self.points.is_empty() {
      return res;
    };
    let mut last_point = self.points[0];
    for point in &self.points[1..] {
      res += last_point.dist(*point);
      last_point = *point;
    }
    res += last_point.dist(self.points[0]);
    res
  }
}

pub struct Circle {
  center: Point,
  radius: i32,
}

impl Circle {
  // add methods
  fn new(center: Point, radius: i32) -> Self {
    Circle { center, radius }
  }

  pub fn circumference(&self) -> f64 {
    2.0 * std::f64::consts::PI * f64::from(self.radius)
  }
}

pub enum Shape {
  Polygon(Polygon),
  Circle(Circle),
}

impl From<Polygon> for Shape {
  fn from(polygon: Polygon) -> Self {
    Shape::Polygon(polygon)
  }
}

impl From<Circle> for Shape {
  fn from(circle: Circle) -> Self {
    Shape::Circle(circle)
  }
}

impl Shape {
  pub fn perimeter(&self) -> f64 {
    match self {
      Shape::Polygon(poly) => poly.length(),
      Shape::Circle(circle) => circle.circumference(),
    }
  }
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

  #[test]
  fn test_shape_perimeters() {
    let mut poly = Polygon::new();
    poly.add_point(Point::new(12, 13));
    poly.add_point(Point::new(17, 11));
    poly.add_point(Point::new(16, 16));
    let shapes = vec![
      Shape::Polygon(poly),
      // Shape::from(poly),
      Shape::Circle(Circle::new(Point::new(10, 20), 5)),
      // Shape::from(Circle::new(Point::new(10, 20), 5)),
    ];
    let perimeters = shapes
      .iter()
      .map(Shape::perimeter)
      .map(round_two_digits)
      .collect::<Vec<_>>();
    assert_eq!(perimeters, vec![15.48, 31.42]);
  }
}

pub fn polygon_struct() {
  println!("called polygon_struct().")
}
