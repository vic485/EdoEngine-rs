//=============================================================================
// vector3.rs
//
// Created by Victor on 2019/10/27
//=============================================================================

use std::ops::{Add, Div};

/// A three-dimensional vector
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    /// Computes the cross product of two vectors
    pub fn cross(lhs: Vector3, rhs: Vector3) -> Vector3 {
        Vector3::new(lhs.y * rhs.z - lhs.z * rhs.y, lhs.z * rhs.x - lhs.x * rhs.z, lhs.x * rhs.y - lhs.y * rhs.x)
    }

    /// Computes the dot product of two vectors
    pub fn dot(lhs: Vector3, rhs: Vector3) -> f32 {
        lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z
    }

    /// Creates a new three-dimensional vector
    pub fn new(x_val: f32, y_val: f32, z_val: f32) -> Vector3 {
        Vector3 { x: x_val, y: y_val, z: z_val }
    }
}

impl Add<Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Vector3) -> Self::Output {
        Vector3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Div<f32> for Vector3 {
    type Output = Vector3;

    fn div(self, rhs: f32) -> Self::Output {
        Vector3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}
