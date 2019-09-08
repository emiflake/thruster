use crate::algebra::prelude::*;

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
pub struct Point3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point3 {
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
