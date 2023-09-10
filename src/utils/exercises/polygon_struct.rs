#![allow(unused_variables, dead_code)]

pub struct Point {
    // add fields
}

impl Point {
    // add methods
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

#[allow(dead_code)]
fn polygon_struct() {
    use super::*;
    
    fn round_two_digits(x: f64) -> f64 {
        (x * 100.0).round() / 100.0
    }
}