//=============================================================================
// vector.rs
//
// Created by Victor on 2019/10/31
//=============================================================================

use std::ops::{Add, Div, Mul, Neg, Sub};
use crate::math::{EPSILON, clamp};
use std::fmt::{Display, Formatter, Error};

// TODO: Do we need to implement copy and clone ourselves?
/// A two dimensional vector
#[derive(Copy, Clone)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    /// Returns the angle in degrees between two vectors
    pub fn angle(from: Vector2, to: Vector2) -> f32 {
        let denominator = (from.square_magnitude() * to.square_magnitude()).sqrt();
        let dot = clamp(Vector2::dot(from, to) / denominator, -1.0, 1.0);
        dot.acos().to_degrees()
    }

    /// Returns the distance between a and b
    pub fn distance(a: Vector2, b: Vector2) -> f32 {
        let x_diff = a.x - b.x;
        let y_diff = a.x - b.x;
        (x_diff * x_diff + y_diff * y_diff).sqrt()
    }

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
        let v = self.clone();
        v.normalize();
        v
    }

    /// Returns a vector2 perpendicular to the provided
    pub fn perpendicular(in_dir: Vector2) -> Vector2 {
        Vector2 { x: -in_dir.y, y: in_dir.x }
    }

    /// Reflects a vector off the provided normal
    pub fn reflect(in_dir: Vector2, in_norm: Vector2) -> Vector2 {
        let factor = -2.0 * Vector2::dot(in_norm, in_dir);
        Vector2 { x: factor * in_norm.x + in_dir.x, y: factor * in_norm.y * in_dir.y }
    }

    /// Multiplies this vector component-wise by another vector
    pub fn scale(mut self, amount: Vector2) {
        self.x *= amount.x;
        self.y *= amount.y;
    }

    /// Returns the signed angle between two vectors.
    /// Always returns the smallest possible angle
    pub fn signed_angle(from: Vector2, to: Vector2) -> f32 {
        let unsigned_angle = Vector2::angle(from, to);
        if (from.x * to.y - from.y * to.x).is_sign_positive() {
            unsigned_angle
        } else {
            -unsigned_angle
        }
    }

    /// Returns the square magnitude of the vector
    pub fn square_magnitude(self) -> f32 {
        self.x * self.x + self.y * self.y
    }
}

/// A three dimensional vector
#[derive(Copy, Clone)]
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

/// A three-dimensional vector of integers
pub struct IVector3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl IVector3 {
    /// Creates a new three-dimensional vector
    pub fn new(x_val: f32, y_val: f32, z_val: f32) -> Vector3 {
        Vector3 { x: x_val, y: y_val, z: z_val }
    }
}

// A four dimensional vector
#[derive(Copy, Clone)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vector4 {
    /// Returns the distance between two vectors
    pub fn distance(a: Vector4, b: Vector4) -> f32 {
        Vector4::magnitude(a - b)
    }

    /// Computes the dot product of two vectors
    pub fn dot(a: Vector4, b: Vector4) -> f32 {
        a.x * b.x + a.y * b.y + a.z * b.z + a.w * b.w
    }

    /// Linearly interpolates between two vectors
    pub fn lerp(a: Vector4, b: Vector4, t: f32) -> Vector4 {
        Vector4 {
            x: a.x + (b.x - a.x) * t,
            y: a.y + (b.y - a.y) * t,
            z: a.z + (b.z - a.z) * t,
            w: a.w + (b.w - a.w) * t,
        }
    }

    /// Returns the magnitude of the vector
    pub fn magnitude(self) -> f32 {
        Vector4::dot(self, self).sqrt()
    }

    /// Moves point "current" toward "target"
    pub fn move_toward(current: Vector4, target: Vector4, max_dist: f32) -> Vector4 {
        let to_x = target.x - current.x;
        let to_y = target.y - current.y;
        let to_z = target.z - current.z;
        let to_w = target.w - current.w;

        let sqr_dist = to_x * to_x + to_y * to_y + to_z * to_z + to_w * to_w;
        if sqr_dist == 0.0 || (max_dist >= 0.0 && sqr_dist <= max_dist * max_dist) {
            return target;
        }

        let dist = sqr_dist.sqrt();
        Vector4 {
            x: current.x + target.x / dist * max_dist,
            y: current.y + target.y / dist * max_dist,
            z: current.z + target.z / dist * max_dist,
            w: current.w + target.w / dist * max_dist,
        }
    }

    /// Creates a new Vector4
    #[inline]
    pub fn new(x_val: f32, y_val: f32, z_val: f32, w_val: f32) -> Vector4 {
        Vector4 { x: x_val, y: y_val, z: z_val, w: w_val }
    }

    /// Makes this vector have a magnitude of 1
    pub fn normalize(mut self) {
        let mag = self.magnitude();
        if mag > EPSILON {
            self.x /= mag;
            self.y /= mag;
            self.z /= mag;
            self.w /= mag;
        } else {
            self.x = 0.0;
            self.y = 0.0;
            self.z = 0.0;
            self.w = 0.0;
        }
    }

    /// Returns this vector with a magnitude of 1
    pub fn normalized(self) -> Vector4 {
        let v = self.clone();
        v.normalize();
        v
    }

    /// Projects vector "a" onto "b"
    pub fn project(a: Vector4, b: Vector4) -> Vector4 {
        b * Vector4::dot(a, b) / Vector4::dot(b, b)
    }

    /// Multiplies this vector component-wise by another vector
    pub fn scale(mut self, amount: Vector4) {
        self.x *= amount.x;
        self.y *= amount.y;
        self.z *= amount.z;
        self.w *= amount.w;
    }

    /// Returns the square magnitude of the vector
    pub fn square_magnitude(self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }
}

impl Display for Vector2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Display for Vector3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Display for IVector3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "(i{}, i{}, i{})", self.x, self.y, self.z)
    }
}

impl Display for Vector4 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "({}, {}, {}, {})", self.x, self.y, self.z, self.w)
    }
}

impl PartialEq for Vector2 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl PartialEq for Vector3 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl PartialEq for IVector3 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl PartialEq for Vector4 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z && self.w == other.w
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

impl Add<IVector3> for IVector3 {
    type Output = IVector3;

    fn add(self, rhs: IVector3) -> Self::Output {
        IVector3 { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}

impl Add<Vector4> for Vector4 {
    type Output = Vector4;

    fn add(self, rhs: Vector4) -> Self::Output {
        Vector4 { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z, w: self.w + rhs.w }
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

impl Div<i32> for IVector3 {
    type Output = IVector3;

    fn div(self, rhs: i32) -> Self::Output {
        IVector3 { x: self.x / rhs, y: self.y / rhs, z: self.z / rhs }
    }
}

impl Div<f32> for Vector4 {
    type Output = Vector4;

    fn div(self, rhs: f32) -> Self::Output {
        Vector4 { x: self.x / rhs, y: self.y / rhs, z: self.z / rhs, w: self.w / rhs }
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

impl Mul<f32> for Vector4 {
    type Output = Vector4;

    fn mul(self, rhs: f32) -> Self::Output {
        Vector4 { x: self.x * rhs, y: self.y * rhs, z: self.z * rhs, w: self.w * rhs }
    }
}

impl Mul<Vector4> for f32 {
    type Output = Vector4;

    fn mul(self, rhs: Vector4) -> Self::Output {
        Vector4 { x: self * rhs.x, y: self * rhs.y, z: self * rhs.z, w: self * rhs.w }
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

impl Neg for Vector4 {
    type Output = Vector4;

    fn neg(self) -> Self::Output {
        Vector4 { x: -self.x, y: -self.y, z: -self.z, w: -self.w }
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

impl Sub<Vector4> for Vector4 {
    type Output = Vector4;

    fn sub(self, rhs: Vector4) -> Self::Output {
        Vector4 { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z, w: self.w - rhs.w }
    }
}
