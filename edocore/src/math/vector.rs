//=============================================================================
// vector.rs
//
// Created by Victor on 2019/10/31
//=============================================================================

use std::ops::{Add, Div, Mul, Neg, Sub};
use crate::math::EPSILON;

// TODO: Do we need to implement copy and clone ourselves?
/// A two dimensional vector
#[derive(Copy, Clone)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    /// Computes the dot product of two vectors
    pub fn dot(lhs: Vector2, rhs: Vector2) -> f32 {
        lhs.x * rhs.x + lhs.y * rhs.y
    }

    /// Creates a new Vector2
    #[inline]
    pub fn new(x_val: f32, y_val: f32) -> Vector2 {
        Vector2 { x: x_val, y: y_val }
    }

    /// Linearly interpolates between two vectors
    pub fn lerp(a: Vector2, b: Vector2, t: f32) -> Vector2 {
        Vector2 { x: a.x + (b.x - a.x) * t, y: a.y + (b.y - a.y) * t }
    }

    /// Returns the magnitude of the vector
    pub fn magnitude(self) -> f32 {
        self.square_magnitude().sqrt()
    }

    /// Moves a point "current" toward "target"
    pub fn move_toward(current: Vector2, target: Vector2, max_dist: f32) -> Vector2 {
        let to_x = target.x - current.x;
        let to_y = target.y - current.y;
        let sqr_dist = to_x * to_x + to_y * to_y;

        if sqr_dist == 0.0 || (max_dist >= 0.0 && sqr_dist <= max_dist * max_dist) {
            return target;
        }

        let dist = sqr_dist.sqrt();
        Vector2 { x: current.x + to_x / dist * max_dist, y: current.y + to_y / dist * max_dist }
    }

    /// Makes this vector have a magnitude of 1
    pub fn normalize(mut self) {
        let mag = self.magnitude();
        if mag > EPSILON {
            self.x /= mag;
            self.y /= mag;
        } else {
            self.x = 0.0;
            self.y = 0.0;
        }
    }

    /// Returns this vector with a magnitude of 1 
    pub fn normalized(self) -> Vector2 {
        let v = Vector2 { x: self.x, y: self.y };
        v.normalize();
        v
    }

    /// Multiplies this vector component-wise by another vector
    pub fn scale(mut self, amount: Vector2) {
        self.x *= amount.x;
        self.y *= amount.y;
    }

    /// Returns the square magnitude of the vector
    pub fn square_magnitude(self) -> f32 {
        self.x * self.x + self.y * self.y
    }
}

impl PartialEq for Vector2 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

/// A three dimensional vector
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    /// Computes the cross product of two vectors
    pub fn cross(lhs: Vector3, rhs: Vector3) -> Vector3 {
        Vector3 { x: lhs.y * rhs.z - lhs.z * rhs.y, y: lhs.z * rhs.x - lhs.x * rhs.z, z: lhs.x * rhs.y - lhs.y * rhs.x }
    }

    /// Computes the dot product of two vectors
    pub fn dot(lhs: Vector3, rhs: Vector3) -> f32 {
        lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z
    }

    /// Creates a new Vector3
    #[inline]
    pub fn new(x_val: f32, y_val: f32, z_val: f32) -> Vector3 {
        Vector3 { x: x_val, y: y_val, z: z_val }
    }
}

// Operator implementations
impl Add<Vector2> for Vector2 {
    type Output = Vector2;

    fn add(self, rhs: Vector2) -> Self::Output {
        Vector2 { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl Add<Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Vector3) -> Self::Output {
        Vector3 { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}

impl Div<Vector2> for Vector2 {
    type Output = Vector2;

    fn div(self, rhs: Vector2) -> Self::Output {
        Vector2 { x: self.x / rhs.x, y: self.y / rhs.y }
    }
}

impl Div<f32> for Vector2 {
    type Output = Vector2;

    fn div(self, rhs: f32) -> Self::Output {
        Vector2 { x: self.x / rhs, y: self.y / rhs }
    }
}

impl Div<f32> for Vector3 {
    type Output = Vector3;

    fn div(self, rhs: f32) -> Self::Output {
        Vector3 { x: self.x / rhs, y: self.y / rhs, z: self.z / rhs }
    }
}

impl Mul<Vector2> for Vector2 {
    type Output = Vector2;

    fn mul(self, rhs: Vector2) -> Self::Output {
        Vector2 { x: self.x * rhs.x, y: self.y * rhs.y }
    }
}

impl Mul<f32> for Vector2 {
    type Output = Vector2;

    fn mul(self, rhs: f32) -> Self::Output {
        Vector2 { x: self.x * rhs, y: self.y * rhs }
    }
}

impl Mul<Vector2> for f32 {
    type Output = Vector2;

    fn mul(self, rhs: Vector2) -> Self::Output {
        Vector2 { x: self * rhs.x, y: self * rhs.y }
    }
}

impl Mul<f32> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f32) -> Self::Output {
        Vector3 { x: self.x * rhs, y: self.y * rhs, z: self.z * rhs }
    }
}

impl Mul<Vector3> for f32 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Self::Output {
        Vector3 { x: self * rhs.x, y: self * rhs.y, z: self * rhs.z }
    }
}

impl Neg for Vector2 {
    type Output = Vector2;

    fn neg(self) -> Self::Output {
        Vector2 { x: -self.x, y: -self.y }
    }
}

impl Neg for Vector3 {
    type Output = Vector3;

    fn neg(self) -> Self::Output {
        Vector3 { x: -self.x, y: -self.y, z: -self.z }
    }
}

impl Sub<Vector2> for Vector2 {
    type Output = Vector2;

    fn sub(self, rhs: Vector2) -> Self::Output {
        Vector2 { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl Sub<Vector3> for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Vector3) -> Self::Output {
        Vector3 { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
    }
}
