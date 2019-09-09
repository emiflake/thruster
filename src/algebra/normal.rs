use crate::algebra::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Normal {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Normal {
    pub const ORIGIN: Self = Normal {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    pub fn has_nans(&self) -> bool {
        self.x.is_nan() || self.y.is_nan() || self.z.is_nan()
    }

    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Normal { x, y, z }
    }

    pub fn length2(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// The magnitude/length of a Vector
    pub fn length(&self) -> f64 {
        self.length2().sqrt()
    }

    pub fn normalized(&self) -> Self {
        let mag = self.length();

        /* Make a copy and divide it by the magnitude*/
        *self / mag
    }

    pub fn dot(&self, rhs: &Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl From<Vec3> for Normal {
    fn from(v: Vec3) -> Self {
        Self::new(v.x, v.y, v.z)
    }
}

impl From<Point3> for Normal {
    fn from(p: Point3) -> Self {
        Self::new(p.x, p.y, p.z)
    }
}

impl std::ops::Sub<Normal> for Normal {
    type Output = Normal;
    fn sub(self, rhs: Normal) -> Normal {
        Normal::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl std::ops::Add<Normal> for Normal {
    type Output = Normal;
    fn add(self, rhs: Normal) -> Normal {
        Normal::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl std::ops::Mul<f64> for Normal {
    type Output = Normal;
    fn mul(self, rhs: f64) -> Normal {
        Normal::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl std::ops::Mul<Normal> for Normal {
    type Output = Normal;
    fn mul(self, rhs: Normal) -> Normal {
        Normal::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl std::ops::Mul<Normal> for f64 {
    type Output = Normal;
    fn mul(self, rhs: Normal) -> Normal {
        Normal::new(rhs.x * self, rhs.y * self, rhs.z * self)
    }
}

impl std::ops::Div<f64> for Normal {
    type Output = Normal;
    fn div(self, rhs: f64) -> Normal {
        Normal::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl std::ops::Neg for Normal {
    type Output = Normal;
    fn neg(self) -> Normal {
        Normal::new(-self.x, -self.y, -self.z)
    }
}

impl std::ops::Index<usize> for Normal {
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

impl std::ops::IndexMut<usize> for Normal {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Dimension {} invalid while indexing 3D type", i),
        }
    }
}

impl Transformable for Normal {
    fn apply_t(&self, trans: &Transform) -> Self {
        let Self { x, y, z } = self;
        let Transform { mat, inv_mat } = trans;
        Self::new(
            inv_mat.at(0, 0) * x + inv_mat.at(1, 0) * y + inv_mat.at(2, 0) * z,
            inv_mat.at(0, 1) * x + inv_mat.at(1, 1) * y + inv_mat.at(2, 1) * z,
            inv_mat.at(0, 2) * x + inv_mat.at(1, 2) * y + inv_mat.at(2, 2) * z,
        )
    }
}
