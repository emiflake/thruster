use image::{Pixel, Rgb, Rgba};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Vec2 {
        Vec2 { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
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

trait Clampable {
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
    pub const ORIGIN: Self = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }
    pub fn length2(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f64 {
        self.length2().sqrt()
    }

    pub fn dot(&self, rhs: &Vec3) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn normalized(&self) -> Self {
        let mag = self.length();

        /* Make a copy and divide it by the magnitude*/
        *self / mag
    }

    pub fn distance2(&self, other: &Vec3) -> f64 {
        (self.x - other.x).powf(2.0) + (self.y - other.y).powf(2.0) + (self.z - other.z).powf(2.0)
    }

    pub fn distance(&self, other: &Vec3) -> f64 {
        self.distance2(other).sqrt()
    }

    pub fn clamp_as_color(&self) -> Self {
        self.clamp_to(Vec3::ORIGIN, Vec3::new(255.0, 255.0, 255.0))
    }

    pub fn cross_product(&self, other: &Vec3) -> Vec3 {
        Vec3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

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

    pub fn rotate(self, theta: Vec3) -> Self {
        let v = Vec3 {
            x: self.x,
            y: self.y * theta.x.cos() + self.z * theta.x.sin(),
            z: -self.z * theta.x.sin() + self.z * theta.x.cos(),
        };
        let w = Vec3 {
            x: v.x * theta.y.cos() + v.z * theta.y.sin(),
            y: v.y,
            z: -v.x * theta.y.sin() + theta.y.cos() * v.z,
        };
        w
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

#[derive(Copy, Clone)]
pub struct Vertex {
    pub origin: Vec3,
    pub normal: Vec3,
    pub uv: Vec2,
}

use crate::parser;
impl Vertex {
    #[allow(dead_code)]
    pub fn new(origin: Vec3, normal: Vec3, uv: Vec2) -> Self {
        Self { origin, normal, uv }
    }

    #[allow(dead_code)]
    pub fn from_parsed(vertex: &parser::Vertex3) -> Self {
        Self {
            origin: Vec3::new(vertex.pos.x, vertex.pos.y, vertex.pos.z),
            normal: Vec3::new(vertex.normal.x, vertex.normal.y, vertex.normal.z),
            uv: Vec2::new(vertex.uv.x, vertex.uv.y),
        }
    }
}
