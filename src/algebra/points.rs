use crate::algebra::prelude::*;

use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub struct Point2 {
    pub x: f64,
    pub y: f64,
}

impl Point2 {
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    /// Check if Point has NaN (floating point)
    /// # Example:
    /// ```
    /// use thruster::algebra::prelude::*;
    /// assert!(Point2::new(std::f64::NAN, 0.0).has_nans());
    /// assert!(!Point2::new(0.0, 5.0).has_nans());
    /// ```
    pub fn has_nans(&self) -> bool {
        self.x.is_nan() || self.y.is_nan()
    }
}

impl std::ops::Add<Point2> for Point2 {
    type Output = Point2;
    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl std::ops::Mul<Point2> for Point2 {
    type Output = Point2;
    fn mul(self, rhs: Self) -> Self {
        Self::new(self.x * rhs.x, self.y * rhs.y)
    }
}

impl From<Vec2> for Point2 {
    fn from(v: Vec2) -> Self {
        Self::new(v.x, v.y)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub struct Point3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point3 {
    pub const ORIGIN: Self = Self::new(0.0, 0.0, 0.0);

    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    /// Check if Point has NaN (floating point)
    /// # Example:
    /// ```
    /// use thruster::algebra::prelude::*;
    /// assert!(Point3::new(0.0, std::f64::NAN, 0.0).has_nans());
    /// assert!(!Point3::new(4.0, 0.0, 5.0).has_nans());
    /// ```
    pub fn has_nans(&self) -> bool {
        self.x.is_nan() || self.y.is_nan() || self.z.is_nan()
    }

    pub fn distance2(&self, rhs: &Self) -> f64 {
        (*self - *rhs).length2()
    }

    /// Get the distance between two points
    pub fn distance(&self, rhs: &Self) -> f64 {
        (*self - *rhs).length()
    }

    /// Element-wise minimum over two Points
    pub fn min(self, rhs: &Self) -> Self {
        Point3 {
            x: self.x.min(rhs.x),
            y: self.y.min(rhs.y),
            z: self.z.min(rhs.z),
        }
    }

    /// Element-wise maximum over two Points
    pub fn max(self, rhs: &Self) -> Self {
        Point3 {
            x: self.x.max(rhs.x),
            y: self.y.max(rhs.y),
            z: self.z.max(rhs.z),
        }
    }
}

impl From<Vec3> for Point3 {
    fn from(v: Vec3) -> Self {
        Self::new(v.x, v.y, v.z)
    }
}

impl std::ops::Add<Point3> for Point3 {
    type Output = Point3;
    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl std::ops::Mul<Point3> for Point3 {
    type Output = Point3;
    fn mul(self, rhs: Self) -> Self {
        Self::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl std::ops::Mul<f64> for Point3 {
    type Output = Point3;
    fn mul(self, rhs: f64) -> Self {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl std::ops::Div<f64> for Point3 {
    type Output = Point3;
    fn div(self, rhs: f64) -> Self {
        Self::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl std::ops::Add<Vec3> for Point3 {
    type Output = Point3;
    fn add(self, rhs: Vec3) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl std::ops::Sub<Vec3> for Point3 {
    type Output = Point3;
    fn sub(self, rhs: Vec3) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl std::ops::Sub<Point3> for Point3 {
    type Output = Vec3;
    fn sub(self, rhs: Self) -> Vec3 {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl std::ops::SubAssign<Vec3> for Point3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl std::ops::AddAssign<Vec3> for Point3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl std::ops::MulAssign<Vec3> for Point3 {
    fn mul_assign(&mut self, rhs: Vec3) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl std::ops::Index<usize> for Point3 {
    type Output = f64;
    fn index(&self, i: usize) -> &Self::Output {
        match i {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Dimension {} invalid while indexing 3D type", i),
        }
    }
}

impl std::ops::IndexMut<usize> for Point3 {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Dimension {} invalid while indexing 3D type", i),
        }
    }
}

impl Transformable for Point3 {
    fn apply_t(&self, trans: &Transform) -> Self {
        let Self { x, y, z } = self;
        let m = &trans.mat;
        let xp = m.at(0, 0) * x + m.at(0, 1) * y + m.at(0, 2) * z + m.at(0, 3);
        let yp = m.at(1, 0) * x + m.at(1, 1) * y + m.at(1, 2) * z + m.at(1, 3);
        let zp = m.at(2, 0) * x + m.at(2, 1) * y + m.at(2, 2) * z + m.at(2, 3);
        let wp = m.at(3, 0) * x + m.at(3, 1) * y + m.at(3, 2) * z + m.at(3, 3);
        if wp == 1.0 {
            Self::new(xp, yp, zp)
        } else {
            Self::new(xp, yp, zp) / wp
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transform() {
        let my_point = Point3::ORIGIN;
        let translation = Transform::translation(&Vec3::new(0.0, 1.0, 0.0));

        let translated = my_point.apply_t(&translation);
        assert_eq!(my_point + Vec3::new(0.0, 1.0, 0.0), translated);
    }
}
