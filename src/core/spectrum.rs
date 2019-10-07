use crate::algebra::prelude::*;
use std::iter::FromIterator;
use std::slice::{Iter, IterMut};

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct RGBSpectrum {
    pub rgb: [f64; 3],
}

impl RGBSpectrum {
    pub const fn new(rgb: [f64; 3]) -> Self {
        Self { rgb }
    }

    pub const fn from_rgb(r: f64, g: f64, b: f64) -> Self {
        Self::new([r, g, b])
    }

    pub const BLACK: Self = Self::from_rgb(0.0, 0.0, 0.0);

    pub fn iter(&self) -> Iter<f64> {
        self.rgb.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<f64> {
        self.rgb.iter_mut()
    }

    pub fn sqrt(&self) -> Self {
        self.iter().map(|s| s.sqrt()).collect()
    }

    pub fn has_nans(&self) -> bool {
        self.iter().any(|s| s.is_nan())
    }

    pub fn add_contribution(&mut self, rhs: RGBSpectrum, spp: usize) {
        for (s, o) in self.iter_mut().zip(rhs.iter()) {
            *s += o / f64::from(spp as u32);
        }
    }

    pub fn clamp(&mut self) {
        self.rgb.iter_mut().for_each(|s| {
            *s = if *s > 255.0 {
                255.0
            } else if *s < 0.0 {
                0.0
            } else {
                *s
            };
        });
    }

    pub fn mul_with(&self, rhs: RGBSpectrum) -> RGBSpectrum {
        RGBSpectrum::from_rgb(
            (self[0] / 255.0 * rhs[0] / 255.0) * 255.0,
            (self[1] / 255.0 * rhs[1] / 255.0) * 255.0,
            (self[2] / 255.0 * rhs[2] / 255.0) * 255.0,
        )
    }
}

impl FromIterator<f64> for RGBSpectrum {
    fn from_iter<T: IntoIterator<Item = f64>>(it: T) -> RGBSpectrum {
        let mut s = RGBSpectrum::BLACK;
        for (r, x) in s.iter_mut().zip(it.into_iter()) {
            *r = x;
        }
        s
    }
}

impl<'a> FromIterator<&'a f64> for RGBSpectrum {
    fn from_iter<T: IntoIterator<Item = &'a f64>>(it: T) -> RGBSpectrum {
        let mut s = RGBSpectrum::BLACK;
        for (r, x) in s.iter_mut().zip(it.into_iter()) {
            *r = *x;
        }
        s
    }
}

impl std::ops::Index<usize> for RGBSpectrum {
    type Output = f64;
    fn index(&self, idx: usize) -> &Self::Output {
        &self.rgb[idx]
    }
}

impl std::ops::Mul<f64> for RGBSpectrum {
    type Output = RGBSpectrum;
    fn mul(self, rhs: f64) -> Self {
        self.iter().map(|s| s * rhs).collect()
    }
}

impl std::ops::Div<f64> for RGBSpectrum {
    type Output = RGBSpectrum;
    fn div(self, rhs: f64) -> Self {
        self.iter().map(|s| s / rhs).collect()
    }
}

impl std::ops::Add<RGBSpectrum> for RGBSpectrum {
    type Output = RGBSpectrum;
    fn add(self, rhs: Self) -> Self {
        self.iter().zip(rhs.iter()).map(|(a, b)| a + b).collect()
    }
}

impl std::ops::AddAssign for RGBSpectrum {
    fn add_assign(&mut self, rhs: Self) {
        for (s, o) in self.iter_mut().zip(rhs.iter()) {
            *s += o;
        }
    }
}

impl std::ops::Neg for RGBSpectrum {
    type Output = RGBSpectrum;
    fn neg(self) -> Self {
        self.iter().map(|s| -s).collect()
    }
}
