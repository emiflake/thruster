use std::iter::FromIterator;
use std::slice::{Iter, IterMut};

#[derive(PartialEq, Debug, Clone)]
pub struct Mat4x4 {
    pub m: [f64; 16],
}

impl Default for Mat4x4 {
    /// Identity matrix
    fn default() -> Self {
        Self::new([
            1.0, 0.0, 0.0, 0.0, //
            0.0, 1.0, 0.0, 0.0, //
            0.0, 0.0, 1.0, 0.0, //
            0.0, 0.0, 0.0, 1.0, //
        ])
    }
}

impl Mat4x4 {
    pub const EMPTY: Self = Self { m: [0.0; 16] };
    pub const IDENTITY: Self = Self::new([
        1.0, 0.0, 0.0, 0.0, //
        0.0, 1.0, 0.0, 0.0, //
        0.0, 0.0, 1.0, 0.0, //
        0.0, 0.0, 0.0, 1.0, //
    ]);

    pub const fn new(m: [f64; 16]) -> Self {
        Self { m }
    }

    pub fn transpose(&self) -> Mat4x4 {
        let mut res = Mat4x4::EMPTY;
        for i in 0..4 {
            for j in 0..4 {
                *res.at_mut(i, j) = *self.at(j, i);
            }
        }
        res
    }

    pub fn at(&self, i: usize, j: usize) -> &f64 {
        &self.m[i * 4 + j]
    }

    pub fn at_mut(&mut self, i: usize, j: usize) -> &mut f64 {
        &mut self.m[i * 4 + j]
    }

    pub fn inverse(&self) -> Self {
        let mut i = Self::EMPTY;
        i.m[0] = self.m[5] * self.m[10] * self.m[15]
            - self.m[5] * self.m[11] * self.m[14]
            - self.m[9] * self.m[6] * self.m[15]
            + self.m[9] * self.m[7] * self.m[14]
            + self.m[13] * self.m[6] * self.m[11]
            - self.m[13] * self.m[7] * self.m[10];

        i.m[4] = -self.m[4] * self.m[10] * self.m[15]
            + self.m[4] * self.m[11] * self.m[14]
            + self.m[8] * self.m[6] * self.m[15]
            - self.m[8] * self.m[7] * self.m[14]
            - self.m[12] * self.m[6] * self.m[11]
            + self.m[12] * self.m[7] * self.m[10];

        i.m[8] = self.m[4] * self.m[9] * self.m[15]
            - self.m[4] * self.m[11] * self.m[13]
            - self.m[8] * self.m[5] * self.m[15]
            + self.m[8] * self.m[7] * self.m[13]
            + self.m[12] * self.m[5] * self.m[11]
            - self.m[12] * self.m[7] * self.m[9];

        i.m[12] = -self.m[4] * self.m[9] * self.m[14]
            + self.m[4] * self.m[10] * self.m[13]
            + self.m[8] * self.m[5] * self.m[14]
            - self.m[8] * self.m[6] * self.m[13]
            - self.m[12] * self.m[5] * self.m[10]
            + self.m[12] * self.m[6] * self.m[9];

        i.m[1] = -self.m[1] * self.m[10] * self.m[15]
            + self.m[1] * self.m[11] * self.m[14]
            + self.m[9] * self.m[2] * self.m[15]
            - self.m[9] * self.m[3] * self.m[14]
            - self.m[13] * self.m[2] * self.m[11]
            + self.m[13] * self.m[3] * self.m[10];

        i.m[5] = self.m[0] * self.m[10] * self.m[15]
            - self.m[0] * self.m[11] * self.m[14]
            - self.m[8] * self.m[2] * self.m[15]
            + self.m[8] * self.m[3] * self.m[14]
            + self.m[12] * self.m[2] * self.m[11]
            - self.m[12] * self.m[3] * self.m[10];

        i.m[9] = -self.m[0] * self.m[9] * self.m[15]
            + self.m[0] * self.m[11] * self.m[13]
            + self.m[8] * self.m[1] * self.m[15]
            - self.m[8] * self.m[3] * self.m[13]
            - self.m[12] * self.m[1] * self.m[11]
            + self.m[12] * self.m[3] * self.m[9];

        i.m[13] = self.m[0] * self.m[9] * self.m[14]
            - self.m[0] * self.m[10] * self.m[13]
            - self.m[8] * self.m[1] * self.m[14]
            + self.m[8] * self.m[2] * self.m[13]
            + self.m[12] * self.m[1] * self.m[10]
            - self.m[12] * self.m[2] * self.m[9];

        i.m[2] = self.m[1] * self.m[6] * self.m[15]
            - self.m[1] * self.m[7] * self.m[14]
            - self.m[5] * self.m[2] * self.m[15]
            + self.m[5] * self.m[3] * self.m[14]
            + self.m[13] * self.m[2] * self.m[7]
            - self.m[13] * self.m[3] * self.m[6];

        i.m[6] = -self.m[0] * self.m[6] * self.m[15]
            + self.m[0] * self.m[7] * self.m[14]
            + self.m[4] * self.m[2] * self.m[15]
            - self.m[4] * self.m[3] * self.m[14]
            - self.m[12] * self.m[2] * self.m[7]
            + self.m[12] * self.m[3] * self.m[6];

        i.m[10] = self.m[0] * self.m[5] * self.m[15]
            - self.m[0] * self.m[7] * self.m[13]
            - self.m[4] * self.m[1] * self.m[15]
            + self.m[4] * self.m[3] * self.m[13]
            + self.m[12] * self.m[1] * self.m[7]
            - self.m[12] * self.m[3] * self.m[5];

        i.m[14] = -self.m[0] * self.m[5] * self.m[14]
            + self.m[0] * self.m[6] * self.m[13]
            + self.m[4] * self.m[1] * self.m[14]
            - self.m[4] * self.m[2] * self.m[13]
            - self.m[12] * self.m[1] * self.m[6]
            + self.m[12] * self.m[2] * self.m[5];

        i.m[3] = -self.m[1] * self.m[6] * self.m[11]
            + self.m[1] * self.m[7] * self.m[10]
            + self.m[5] * self.m[2] * self.m[11]
            - self.m[5] * self.m[3] * self.m[10]
            - self.m[9] * self.m[2] * self.m[7]
            + self.m[9] * self.m[3] * self.m[6];

        i.m[7] = self.m[0] * self.m[6] * self.m[11]
            - self.m[0] * self.m[7] * self.m[10]
            - self.m[4] * self.m[2] * self.m[11]
            + self.m[4] * self.m[3] * self.m[10]
            + self.m[8] * self.m[2] * self.m[7]
            - self.m[8] * self.m[3] * self.m[6];

        i.m[11] = -self.m[0] * self.m[5] * self.m[11]
            + self.m[0] * self.m[7] * self.m[9]
            + self.m[4] * self.m[1] * self.m[11]
            - self.m[4] * self.m[3] * self.m[9]
            - self.m[8] * self.m[1] * self.m[7]
            + self.m[8] * self.m[3] * self.m[5];

        i.m[15] = self.m[0] * self.m[5] * self.m[10]
            - self.m[0] * self.m[6] * self.m[9]
            - self.m[4] * self.m[1] * self.m[10]
            + self.m[4] * self.m[2] * self.m[9]
            + self.m[8] * self.m[1] * self.m[6]
            - self.m[8] * self.m[2] * self.m[5];

        let mut det =
            self.m[0] * i.m[0] + self.m[1] * i.m[4] + self.m[2] * i.m[8] + self.m[3] * i.m[12];
        assert!(det != 0.0);
        det = 1.0 / det;

        for x in &mut i.m {
            *x *= det;
        }
        i
    }

    /// Get iterator over mat
    pub fn iter(&self) -> Iter<f64> {
        self.m.iter()
    }

    /// Get mutable iterator over mat
    pub fn iter_mut(&mut self) -> IterMut<f64> {
        self.m.iter_mut()
    }
}

impl FromIterator<f64> for Mat4x4 {
    fn from_iter<T: IntoIterator<Item = f64>>(it: T) -> Mat4x4 {
        let mut m = Mat4x4::EMPTY;
        for (r, x) in m.iter_mut().zip(it.into_iter()) {
            *r = x;
        }
        m
    }
}

impl<'a> FromIterator<&'a f64> for Mat4x4 {
    fn from_iter<T: IntoIterator<Item = &'a f64>>(it: T) -> Mat4x4 {
        let mut m = Mat4x4::EMPTY;
        for (r, x) in m.iter_mut().zip(it.into_iter()) {
            *r = *x;
        }
        m
    }
}

impl std::ops::Add<Mat4x4> for Mat4x4 {
    type Output = Mat4x4;
    fn add(self, rhs: Self) -> Self {
        self.iter().zip(rhs.iter()).map(|(&a, &b)| a + b).collect()
    }
}

impl std::ops::Sub<Mat4x4> for Mat4x4 {
    type Output = Mat4x4;
    fn sub(self, rhs: Self) -> Self {
        self.iter().zip(rhs.iter()).map(|(&a, &b)| a - b).collect()
    }
}

impl std::ops::Mul<Mat4x4> for Mat4x4 {
    type Output = Mat4x4;

    fn mul(self, rhs: Mat4x4) -> Mat4x4 {
        let mut res = Mat4x4::EMPTY;
        for i in 0..4 {
            for j in 0..4 {
                *res.at_mut(i, j) = *self.at(i, 0) * *rhs.at(0, j)
                    + *self.at(i, 1) * *rhs.at(1, j)
                    + *self.at(i, 2) * *rhs.at(2, j)
                    + *self.at(i, 3) * *rhs.at(3, j);
            }
        }
        res
    }
}

impl std::ops::Mul<f64> for Mat4x4 {
    type Output = Mat4x4;
    fn mul(self, rhs: f64) -> Self {
        self.iter().map(|a| a * rhs).collect()
    }
}

impl std::ops::Mul<Mat4x4> for f64 {
    type Output = Mat4x4;
    fn mul(self, rhs: Mat4x4) -> Mat4x4 {
        rhs.iter().map(|a| a * self).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let mut a = Mat4x4::default();
        *a.at_mut(0, 1) = 1.0;
        let mut b = Mat4x4::IDENTITY;
        *b.at_mut(2, 3) = 3.0;
        let c = Mat4x4::new([
            2.0, 1.0, 0.0, 0.0, 0.0, 2.0, 0.0, 0.0, 0.0, 0.0, 2.0, 3.0, 0.0, 0.0, 0.0, 2.0,
        ]);
        assert!(a + b == c);
    }
    #[test]
    fn test_sub() {
        let mut a = Mat4x4::IDENTITY;
        *a.at_mut(0, 1) = 1.0;
        let mut b = Mat4x4::IDENTITY;
        *b.at_mut(2, 3) = 3.0;
        let c = Mat4x4::new([
            0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -3.0, 0.0, 0.0, 0.0, 0.0,
        ]);
        assert!(a - b == c);
    }
    #[test]
    fn test_mul() {
        assert!(Mat4x4::IDENTITY * Mat4x4::IDENTITY == Mat4x4::IDENTITY);
        let a = Mat4x4::new([
            1.0, 2.0, 1.0, 0.0, 3.0, 1.0, 4.0, 2.0, 1.0, 2.0, -5.0, 4.0, 3.0, 2.0, 4.0, 1.0,
        ]);
        let b = Mat4x4::new([
            8.0, 0.0, 2.0, 3.0, -2.0, 1.0, 0.0, 1.0, 5.0, -2.0, 3.0, 1.0, 0.0, 0.0, 4.0, 1.0,
        ]);
        let c = Mat4x4::new([
            9.0, 0.0, 5.0, 6.0, 42.0, -7.0, 26.0, 16.0, -21.0, 12.0, 3.0, 4.0, 40.0, -6.0, 22.0,
            16.0,
        ]);
        assert!(a * b == c);
    }

    #[test]
    fn tranpose() {
        let a = Mat4x4::new([
            1.0, 2.0, 3.0, 4.0, //
            5.0, 6.0, 7.0, 8.0, //
            9.0, 10.0, 11.0, 12.0, //
            13.0, 14.0, 15.0, 16.0, //
        ]);

        assert_eq!(
            a.transpose(),
            Mat4x4::new([
                1.0, 5.0, 9.0, 13.0, //
                2.0, 6.0, 10.0, 14.0, //
                3.0, 7.0, 11.0, 15.0, //
                4.0, 8.0, 12.0, 16.0, //
            ])
        );
    }
}
