//=============================================================================
// ivector3.rs
//
// Created by Luuk on 2019/12/04
//=============================================================================

use std::ops::{Add, Div};

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

impl Add<IVector3> for IVector3 {
    type Output = IVector3;

    fn add(self, rhs: IVector3) -> Self::Output {
        IVector3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Div<i32> for IVector3 {
    type Output = IVector3;

    fn div(self, rhs: i32) -> Self::Output {
        IVector3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}
