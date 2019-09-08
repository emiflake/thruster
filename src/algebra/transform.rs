use crate::algebra::prelude::*;

pub struct Transform {
    mat: Mat4x4,
    inv_mat: Mat4x4,
}

impl Transform {
    pub const fn new(mat: Mat4x4, inv_mat: Mat4x4) -> Self {
        Self { mat, inv_mat }
    }

    pub fn from_mat(mat: Mat4x4) -> Self {
        let inverse = mat.inverse();
        Self::new(mat, inverse)
    }

    pub fn inverse(self) -> Self {
        Self::new(self.inv_mat, self.mat)
    }

    pub fn transpose(self) -> Self {
        Self::new(self.mat.transpose(), self.inv_mat.transpose())
    }

    pub fn translation(delta: &Vec3) -> Self {
        let mat = Mat4x4::new([
            1.0, 0.0, 0.0, delta.x, //
            0.0, 1.0, 0.0, delta.y, //
            0.0, 0.0, 1.0, delta.z, //
            0.0, 0.0, 0.0, 1.0, //
        ]);
        let inv_mat = Mat4x4::new([
            1.0, 0.0, 0.0, -delta.x, //
            0.0, 1.0, 0.0, -delta.y, //
            0.0, 0.0, 1.0, -delta.z, //
            0.0, 0.0, 0.0, 1.0, //
        ]);

        Self::new(mat, inv_mat)
    }

    pub fn scaling(x: f64, y: f64, z: f64) -> Self {
        let mat = Mat4x4::new([
            x, 0.0, 0.0, 0.0, //
            0.0, y, 0.0, 0.0, //
            0.0, 0.0, z, 0.0, //
            0.0, 0.0, 0.0, 1.0, //
        ]);
        let inv_mat = Mat4x4::new([
            1.0 / x,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0 / y,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0 / z,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
        ]);

        Self::new(mat, inv_mat)
    }

    pub fn rotate_x(theta: f64) -> Self {
        let sin_theta = theta.sin();
        let cos_theta = theta.cos();

        let mat = Mat4x4::new([
            1.0, 0.0, 0.0, 0.0, //
            0.0, cos_theta, -sin_theta, 0.0, //
            0.0, sin_theta, cos_theta, 0.0, //
            0.0, 0.0, 0.0, 1.0, //
        ]);
        let inv_mat = mat.clone().transpose();
        Self::new(mat, inv_mat)
    }

    pub fn rotate_y(theta: f64) -> Self {
        let sin_theta = theta.sin();
        let cos_theta = theta.cos();

        let mat = Mat4x4::new([
            cos_theta, 0.0, sin_theta, 0.0, //
            0.0, 1.0, 0.0, 0.0, //
            -sin_theta, 0.0, cos_theta, 0.0, //
            0.0, 0.0, 0.0, 1.0, //
        ]);
        let inv_mat = mat.clone().transpose();
        Self::new(mat, inv_mat)
    }

    pub fn rotate_z(theta: f64) -> Self {
        let sin_theta = theta.sin();
        let cos_theta = theta.cos();

        let mat = Mat4x4::new([
            cos_theta, -sin_theta, 0.0, 0.0, //
            sin_theta, cos_theta, 0.0, 0.0, //
            0.0, 0.0, 1.0, 0.0, //
            0.0, 0.0, 0.0, 1.0, //
        ]);
        let inv_mat = mat.clone().transpose();
        Self::new(mat, inv_mat)
    }

    pub fn look_at(pos: &Vec3, look: &Vec3, up: &Vec3) -> Self {
        let dir = (*look - *pos).normalized();
        let right = up.normalized().cross_product(&dir).normalized();
        let new_up = dir.cross_product(&right);
        let mat = Mat4x4::new([
            right.x, new_up.x, dir.x, pos.x, //
            right.y, new_up.y, dir.y, pos.y, //
            right.z, new_up.z, dir.z, pos.z, //
            0.0, 0.0, 0.0, 1.0, //
        ]);
        let inv_mat = mat.clone().inverse();
        Transform::new(inv_mat, mat)
    }
}
