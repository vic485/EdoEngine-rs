//=============================================================================
// matrix.rs
//
// Created by Victor on 2019/11/01
//=============================================================================

use std::ops::Mul;

pub struct Matrix4 {
    pub values: [[f32; 4]; 4],
}

impl Matrix4 {}

impl Mul<Matrix4> for Matrix4 {
    type Output = Matrix4;

    fn mul(self, rhs: Matrix4) -> Self::Output {
        let mut res = Matrix4{ values: [[0.0; 4]; 4] };
        
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
