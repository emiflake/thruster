use crate::algebra::prelude::*;
use image::{Pixel, Rgb, Rgba};
use serde_derive::{Deserialize, Serialize};

/// A f64 2-dimensional Vector struct
/// For use with Pixels and sampling
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub const fn new(x: f64, y: f64) -> Vec2 {
        Vec2 { x, y }
    }

    pub fn has_nans(&self) -> bool {
        self.x.is_nan() || self.y.is_nan()
    }
}

/// A standard f64 3-dimensional Vector struct
/// For use everywhere
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl std::ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl std::ops::Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl std::ops::Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Vec3 {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl std::ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl std::ops::Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3::new(rhs.x * self, rhs.y * self, rhs.z * self)
    }
}

impl std::ops::Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Vec3 {
        Vec3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl From<Point3> for Vec3 {
    fn from(p: Point3) -> Self {
        Self::new(p.x, p.y, p.z)
    }
}

/// Make an object clampable between two instances of itself
/// # Example:
/// ```
/// use thruster::algebra::vectors::Clampable;
/// println!("{}", (5f64).clamp_to(2.0, 10.0));
/// //=>  2.0
/// println!("{}", (14f64).clamp_to(2.0, 10.0));
/// //=> 10.0
/// ```
pub trait Clampable {
    fn clamp_to(self, min: Self, max: Self) -> Self;
}

impl Clampable for f64 {
    fn clamp_to(self, min: f64, max: f64) -> f64 {
        if self > max {
            return max;
        }
        if self < min {
            return min;
        }
        self
    }
}

impl Vec3 {
    /// The origin Vector (0.0, 0.0, 0.0)
    pub const ORIGIN: Self = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    pub fn has_nans(&self) -> bool {
        self.x.is_nan() || self.y.is_nan() || self.z.is_nan()
    }

    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }

    /// The square of the magniture/length of the Vector
    /// x^2 + y^2 + z^2
    pub fn length2(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// The magnitude/length of a Vector
    pub fn length(&self) -> f64 {
        self.length2().sqrt()
    }

    /// The dot product between two Vectors
    /// Represents the 'difference' in angle.
    pub fn dot(&self, rhs: &Vec3) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    /// Normalize the Vector towards a unit Vector.
    /// This is achieved by dividing the Vector's elements by its length
    pub fn normalized(&self) -> Self {
        let mag = self.length();

        /* Make a copy and divide it by the magnitude*/
        *self / mag
    }

    /// The square of the distance between two Vectors
    pub fn distance2(&self, other: &Vec3) -> f64 {
        (self.x - other.x).powf(2.0) + (self.y - other.y).powf(2.0) + (self.z - other.z).powf(2.0)
    }

    /// The distance between two vectors
    pub fn distance(&self, other: &Vec3) -> f64 {
        self.distance2(other).sqrt()
    }

    /// Clamp the Vector's values to [0.0, 255.0]
    pub fn clamp_as_color(&self) -> Self {
        self.clamp_to(Vec3::ORIGIN, Vec3::new(255.0, 255.0, 255.0))
    }

    /// The cross product between two vectors
    pub fn cross_product(&self, other: &Vec3) -> Vec3 {
        Vec3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    /// Map a function over each of the Vector's values
    pub fn map_all(self, f: &impl Fn(f64) -> f64) -> Self {
        Vec3::new(f(self.x), f(self.y), f(self.z))
    }

    pub fn from_rgb(rgb: Rgb<u8>) -> Self {
        if let [r, g, b] = rgb.channels() {
            Vec3::new(f64::from(*r), f64::from(*g), f64::from(*b))
        } else {
            Vec3::ORIGIN
        }
    }

    pub fn to_rgb(self) -> Rgb<u8> {
        Rgb([self.x as u8, self.y as u8, self.z as u8])
    }

    pub fn from_rgba(rgba: Rgba<u8>) -> Self {
        if let [r, g, b, _] = rgba.channels() {
            Vec3::new(f64::from(*r), f64::from(*g), f64::from(*b))
        } else {
            Vec3::ORIGIN
        }
    }

    pub fn to_rgba(self) -> Rgba<u8> {
        Rgba([self.x as u8, self.y as u8, self.z as u8, 255])
    }

    /// Rotate a Vector with the angles of another Vector
    ///
    /// **Warning, incomplete, and possibly faulty.**
    pub fn rotate(self, theta: Vec3) -> Self {
        let v = Vec3 {
            x: self.x,
            y: self.y * theta.x.cos() + self.z * theta.x.sin(),
            z: -self.z * theta.x.sin() + self.z * theta.x.cos(),
        };
        Vec3 {
            x: v.x * theta.y.cos() + v.z * theta.y.sin(),
            y: v.y,
            z: -v.x * theta.y.sin() + theta.y.cos() * v.z,
        }
    }

    /// Element-wise minimum over two Vectors
    pub fn min(self, rhs: &Self) -> Self {
        Vec3 {
            x: self.x.min(rhs.x),
            y: self.y.min(rhs.y),
            z: self.z.min(rhs.z),
        }
    }

    /// Element-wise maximum over two Vectors
    pub fn max(self, rhs: &Self) -> Self {
        Vec3 {
            x: self.x.max(rhs.x),
            y: self.y.max(rhs.y),
            z: self.z.max(rhs.z),
        }
    }

    /// Extract a dimension from a Vector by it's number 0-2
    pub fn dim(self, dimension: u32) -> f64 {
        match dimension {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => panic!("Dimension greater than 2"),
        }
    }

    /// Flip a dimension from a Vector by it's number 0-2
    pub fn flip(self, dimension: u32) -> Self {
        match dimension % 3 {
            0 => Vec3::new(-self.x, self.y, self.z),
            1 => Vec3::new(self.x, -self.y, self.z),
            2 => Vec3::new(self.x, self.y, -self.z),
            _ => Vec3::new(-self.x, self.y, self.z),
        }
    }

    /// Rotate a dimension from a Vector by it's number 0-2
    /// This function is particularly useful since it's very versatile in conjunction with flipping
    /// for some transformations
    pub fn rotate_around(self, axis: u32, theta: f64) -> Self {
        match axis % 3 {
            0 => Vec3::new(
                self.x,
                self.y * theta.cos() - self.z * theta.sin(),
                self.y * theta.sin() + self.z * theta.cos(),
            ),
            1 => Vec3::new(
                self.x * theta.cos() + self.z * theta.sin(),
                self.y,
                -self.x * theta.sin() + self.z * theta.cos(),
            ),
            2 => Vec3::new(
                self.x * theta.cos() - self.y * theta.sin(),
                self.y * theta.sin() + self.y * theta.cos(),
                self.z,
            ),
            _ => self.rotate_around(0, theta),
        }
    }

    pub fn max_component(self) -> f64 {
        if self.x > self.y && self.x > self.z {
            self.x
        } else if self.y > self.z {
            self.y
        } else {
            self.z
        }
    }

    pub fn min_component(self) -> f64 {
        if self.x < self.y && self.x < self.z {
            self.x
        } else if self.y < self.z {
            self.y
        } else {
            self.z
        }
    }
}

impl Clampable for Vec3 {
    fn clamp_to(self, min: Vec3, max: Vec3) -> Vec3 {
        Vec3::new(
            self.x.clamp_to(min.x, max.x),
            self.y.clamp_to(min.y, max.y),
            self.z.clamp_to(min.z, max.z),
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Vertex {
    pub origin: Vec3,
    pub normal: Vec3,
    pub uv: Vec2,
}

impl Vertex {
    #[allow(dead_code)]
    pub fn new(origin: Vec3, normal: Vec3, uv: Vec2) -> Self {
        Self { origin, normal, uv }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn vector_maths() {
        assert_eq!(
            Vec3::new(0.0, 5.0, 1.0) + Vec3::new(1.0, 0.0, 5.0),
            Vec3::new(1.0, 5.0, 6.0)
        );

        assert_eq!(
            Vec3::new(0.0, 5.0, 1.0) - Vec3::new(1.0, 0.0, 5.0),
            Vec3::new(-1.0, 5.0, -4.0)
        );

        assert_eq!(Vec3::new(0.5, 2.5, 0.0) * 2.0, Vec3::new(1.0, 5.0, 0.0));

        assert_eq!(5.0 * Vec3::new(1.0, 2.0, 3.0), Vec3::new(5.0, 10.0, 15.0));
    }

    #[test]
    fn dot_product() {
        assert_eq!(Vec3::new(1.0, 5.0, 3.0).dot(&Vec3::new(2.0, 0.0, 0.0)), 2.0);
        assert_eq!(Vec3::new(0.0, 1.0, 0.0).dot(&Vec3::new(0.0, 1.0, 0.0)), 1.0);
        assert_eq!(Vec3::new(0.0, 0.0, 1.0).dot(&Vec3::new(0.0, 1.0, 0.0)), 0.0);
        assert_eq!(
            Vec3::new(0.0, -1.0, 0.0).dot(&Vec3::new(0.0, 1.0, 0.0)),
            -1.0
        );
    }

    #[test]
    fn length2() {
        assert_eq!(Vec3::new(0.0, 0.0, 0.0).length2(), 0.0);
        assert_eq!(Vec3::new(5.0, 3.0, 0.0).length2(), 25.0 + 9.0);
        assert_eq!(Vec3::new(5.0, 9.0, 1.0).length2(), 107.0);
    }

    #[test]
    fn length() {
        assert_eq!(Vec3::new(0.0, 0.0, 0.0).length(), 0.0);
        assert!((Vec3::new(5.0, 9.0, 1.0).length() - 10.3441).abs() <= 0.0001);
    }

    #[test]
    fn clamp_as_color() {
        assert_eq!(Vec3::ORIGIN.clamp_as_color(), Vec3::ORIGIN);
        assert_eq!(
            Vec3::new(5.0, 5.0, 5.0).clamp_as_color(),
            Vec3::new(5.0, 5.0, 5.0)
        );
        assert_eq!(
            Vec3::new(256.0, 0.0, 0.0).clamp_as_color(),
            Vec3::new(255.0, 0.0, 0.0)
        );
        assert_eq!(
            Vec3::new(255.1, 0.0, 0.0).clamp_as_color(),
            Vec3::new(255.0, 0.0, 0.0)
        );
        assert_eq!(
            Vec3::new(-1.0, 0.0, 0.0).clamp_as_color(),
            Vec3::new(0.0, 0.0, 0.0)
        );
    }

    #[test]
    fn normalized() {
        assert_eq!(
            Vec3::new(1.0, 0.0, 0.0).normalized(),
            Vec3::new(1.0, 0.0, 0.0)
        );
        assert!(
            (Vec3::new(1.0, 1.0, 0.0).normalized()
                - Vec3::new(2f64.sqrt() / 2.0, 2f64.sqrt() / 2.0, 0.0))
            .length()
                <= 0.0001
        );

        assert!(
            (Vec3::new(5.0, 94.0, 1.4).normalized().length2()
                - Vec3::new(5.0, 94.0, 1.4).length2())
                <= 0.0001
        );
    }

    #[test]
    fn min_max() {
        let a = Vec3::new(0.0, 1.0, 2.0);
        let b = Vec3::new(5.0, 2.0, 1.0);
        let c = Vec3::ORIGIN;

        assert_eq!(a.min(&b), Vec3::new(0.0, 1.0, 1.0));
        assert_eq!(b.min(&c), c);
        assert_eq!(a.max(&c), a);
        assert_eq!(a.max(&b), Vec3::new(5.0, 2.0, 2.0));
    }

    #[test]
    fn dim() {
        let a = Vec3::new(3.0, 2.0, 1.0);

        assert_eq!(a.dim(0), 3.0);
        assert_eq!(a.dim(1), 2.0);
        assert_eq!(a.dim(2), 1.0);
    }

}
