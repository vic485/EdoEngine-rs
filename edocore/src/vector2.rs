use std::ops::{Add, Div, Mul, Neg, Sub};

/// A two-dimensional vector
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    /// Creates a new two-dimensional vector
    pub fn new(x_val: f32, y_val: f32) -> Vector2 {
        Vector2 { x: x_val, y: y_val }
    }

    /// Returns the magnitude of the vector
    pub fn magnitude(self) -> f32 {
        self.square_magnitude()
    }

    /// Returns the square magnitude of the vector
    pub fn square_magnitude(self) -> f32 {
        self.x * self.x + self.y * self.y
    }
}

// TODO: Do these need docs?
// Vector addition
impl Add<Vector2> for Vector2 {
    type Output = Vector2;

    fn add(self, rhs: Vector2) -> Self::Output {
        Vector2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

// Vector Division
impl Div<Vector2> for Vector2 {
    type Output = Vector2;

    fn div(self, rhs: Vector2) -> Self::Output {
        Vector2::new(self.x / rhs.x, self.y / rhs.y)
    }
}

impl Div<f32> for Vector2 {
    type Output = Vector2;

    fn div(self, rhs: f32) -> Self::Output {
        Vector2::new(self.x / rhs, self.y / rhs)
    }
}

// Vector Multiplication
impl Mul<Vector2> for Vector2 {
    type Output = Vector2;

    fn mul(self, rhs: Vector2) -> Self::Output {
        Vector2::new(self.x * rhs.x, self.y * rhs.y)
    }
}

impl Mul<f32> for Vector2 {
    type Output = Vector2;

    fn mul(self, rhs: f32) -> Self::Output {
        Vector2::new(self.x * rhs, self.y * rhs)
    }
}

impl Mul<Vector2> for f32 {
    type Output = Vector2;

    fn mul(self, rhs: Vector2) -> Self::Output {
        Vector2::new(self * rhs.x, self * rhs.y)
    }
}

// Negate vector
impl Neg for Vector2 {
    type Output = Vector2;

    fn neg(self) -> Self::Output {
        Vector2::new(-self.x, -self.y)
    }
}

// Vector subtraction
impl Sub<Vector2> for Vector2 {
    type Output = Vector2;

    fn sub(self, rhs: Vector2) -> Self::Output {
        Vector2::new(self.x - rhs.x, self.y - rhs.y)
    }
}
