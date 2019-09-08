use crate::algebra::prelude::*;

pub struct Transform {
    pub mat: Mat4x4,
    pub inv_mat: Mat4x4,
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
        let right = comb::cross(&up.normalized(), &dir.normalized());
        let new_up = comb::cross(&dir, &right);
        let mat = Mat4x4::new([
            right.x, new_up.x, dir.x, pos.x, //
            right.y, new_up.y, dir.y, pos.y, //
            right.z, new_up.z, dir.z, pos.z, //
            0.0, 0.0, 0.0, 1.0, //
        ]);
        let inv_mat = mat.clone().inverse();
        Transform::new(inv_mat, mat)
    }

    pub fn compose(self, rhs: Self) -> Self {
        Self::new(self.mat * rhs.mat, self.inv_mat * rhs.inv_mat)
    }
}

impl std::ops::Mul<Transform> for Transform {
    type Output = Transform;
    fn mul(self, rhs: Transform) -> Self {
        self.compose(rhs)
    }
}

/// Describes the ability to apply a `Transform` on particular type
pub trait Transformable {
    fn apply_t(&self, trans: &Transform) -> Self;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn composition() {
        let a = Point3::ORIGIN;
        let go_up = Transform::translation(&Vec3::new(0.0, 1.0, 0.0));
        let go_left = Transform::translation(&Vec3::new(-1.0, 0.0, 0.0));

        let both = go_left * go_up;

        let transformed = a.apply_t(&both);
        assert_eq!(transformed, Point3::new(-1.0, 1.0, 0.0));
    }
}
