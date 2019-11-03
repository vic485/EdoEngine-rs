//=============================================================================
// matrix.rs
//
// Created by Victor on 2019/11/01
//=============================================================================

use std::ops::Mul;
use crate::math::vector::Vector4;

#[derive(Copy, Clone)]
pub struct Matrix4 {
    pub values: [[f32; 4]; 4],
}

impl Matrix4 {
    /// Returns a column of the matrix represented as a Vector4
    pub fn get_column(self, index: usize) -> Vector4 {
        Vector4 {
            x: self.values[0][index],
            y: self.values[1][index],
            z: self.values[2][index],
            w: self.values[3][index],
        }
    }

    /// Returns a row of the matrix represented as a Vector4
    pub fn get_row(self, index: usize) -> Vector4 {
        Vector4 {
            x: self.values[index][0],
            y: self.values[index][1],
            z: self.values[index][2],
            w: self.values[index][3],
        }
    }

    /// Creates a new 4x4 matrix using row vectors
    pub fn new(row0: Vector4, row1: Vector4, row2: Vector4, row3: Vector4) -> Matrix4 {
        Matrix4 {
            values: [[row0.x, row0.y, row0.z, row0.w],
                [row1.x, row1.y, row1.z, row1.w],
                [row2.x, row2.y, row2.z, row2.w],
                [row3.x, row3.y, row3.z, row3.w]]
        }
    }
}

impl PartialEq for Matrix4 {
    fn eq(&self, other: &Self) -> bool {
        self.get_row(0) == other.get_row(0) &&
            self.get_row(1) == other.get_row(1) &&
            self.get_row(2) == other.get_row(2) &&
            self.get_row(3) == other.get_row(3)
    }
}

impl Mul<Matrix4> for Matrix4 {
    type Output = Matrix4;

    fn mul(self, rhs: Matrix4) -> Self::Output {
        let mut res = Matrix4 { values: [[0.0; 4]; 4] };

        // This makes me cry
        res.values[0][0] = self.values[0][0] * rhs.values[0][0] + self.values[0][1] * rhs.values[1][0] + self.values[0][2] * rhs.values[2][0] + self.values[0][3] * rhs.values[3][0];
        res.values[0][1] = self.values[0][0] * rhs.values[0][1] + self.values[0][1] * rhs.values[1][1] + self.values[0][2] * rhs.values[2][1] + self.values[0][3] * rhs.values[3][1];
        res.values[0][2] = self.values[0][0] * rhs.values[0][2] + self.values[0][1] * rhs.values[1][2] + self.values[0][2] * rhs.values[2][2] + self.values[0][3] * rhs.values[3][2];
        res.values[0][3] = self.values[0][0] * rhs.values[0][3] + self.values[0][1] * rhs.values[1][3] + self.values[0][2] * rhs.values[2][3] + self.values[0][3] * rhs.values[3][3];

        res.values[1][0] = self.values[1][0] * rhs.values[0][0] + self.values[1][1] * rhs.values[1][0] + self.values[1][2] * rhs.values[2][0] + self.values[1][3] * rhs.values[3][0];
        res.values[1][1] = self.values[1][0] * rhs.values[0][1] + self.values[1][1] * rhs.values[1][1] + self.values[1][2] * rhs.values[2][1] + self.values[1][3] * rhs.values[3][1];
        res.values[1][2] = self.values[1][0] * rhs.values[0][2] + self.values[1][1] * rhs.values[1][2] + self.values[1][2] * rhs.values[2][2] + self.values[1][3] * rhs.values[3][2];
        res.values[1][3] = self.values[1][0] * rhs.values[0][3] + self.values[1][1] * rhs.values[1][3] + self.values[1][2] * rhs.values[2][3] + self.values[1][3] * rhs.values[3][3];

        res.values[2][0] = self.values[2][0] * rhs.values[0][0] + self.values[2][1] * rhs.values[1][0] + self.values[2][2] * rhs.values[2][0] + self.values[2][3] * rhs.values[3][0];
        res.values[2][1] = self.values[2][0] * rhs.values[0][1] + self.values[2][1] * rhs.values[1][1] + self.values[2][2] * rhs.values[2][1] + self.values[2][3] * rhs.values[3][1];
        res.values[2][2] = self.values[2][0] * rhs.values[0][2] + self.values[2][1] * rhs.values[1][2] + self.values[2][2] * rhs.values[2][2] + self.values[2][3] * rhs.values[3][2];
        res.values[2][3] = self.values[2][0] * rhs.values[0][3] + self.values[2][1] * rhs.values[1][3] + self.values[2][2] * rhs.values[2][3] + self.values[2][3] * rhs.values[3][3];

        res.values[3][0] = self.values[3][0] * rhs.values[0][0] + self.values[3][1] * rhs.values[1][0] + self.values[3][2] * rhs.values[2][0] + self.values[3][3] * rhs.values[3][0];
        res.values[3][1] = self.values[3][0] * rhs.values[0][1] + self.values[3][1] * rhs.values[1][1] + self.values[3][2] * rhs.values[2][1] + self.values[3][3] * rhs.values[3][1];
        res.values[3][2] = self.values[3][0] * rhs.values[0][2] + self.values[3][1] * rhs.values[1][2] + self.values[3][2] * rhs.values[2][2] + self.values[3][3] * rhs.values[3][2];
        res.values[3][3] = self.values[3][0] * rhs.values[0][3] + self.values[3][1] * rhs.values[1][3] + self.values[3][2] * rhs.values[2][3] + self.values[3][3] * rhs.values[3][3];

        res
    }
}
