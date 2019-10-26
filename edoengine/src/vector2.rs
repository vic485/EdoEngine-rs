use std::ops::{Add, Div, Mul, Neg, Sub};

pub struct Vector2 {
    x: f32,
    y: f32,
}

impl Vector2 {
    pub fn new(x_val: f32, y_val: f32) -> Vector2 {
        Vector2 { x: x_val, y: y_val }
    }

    pub fn distance(a: Vector2, b: Vector2) -> f32 {
        ((a.x - b.x) * (a.x - b.x) + (a.y - b.y) * (a.y - b.y)).sqrt()
    }

    pub fn dot(a: Vector2, b: Vector2) -> f32 {
        a.x * b.x + a.y + b.y
    }

    pub fn normalize(&mut self) {
        // this = this / Magnitude
    }
}

// Vector addition
impl Add<Vector2> for Vector2 {
    type Output = Vector2;

    fn add(self, v: Vector2) -> Vector2 {
        Vector2::new(self.x + v.x, self.y + v.y)
    }
}

// Vector division
impl Div<Vector2> for Vector2 {
    type Output = Vector2;

    fn div(self, v: Vector2) -> Vector2 {
        Vector2::new(self.x / v.x, self.y / v.y)
    }
}

impl Div<f32> for Vector2 {
    type Output = Vector2;

    fn div(self, f: f32) -> Vector2 {
        Vector2::new(self.x / f, self.y / f)
    }
}
