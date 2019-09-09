use crate::algebra::prelude::*;

pub trait Texture<T: std::fmt::Debug>: std::fmt::Debug {
    fn sample(&self, uv: &Point2) -> T;
}

#[derive(Debug)]
pub struct ConstantTexture<T> {
    pub t: T,
}

impl<T> ConstantTexture<T> {
    pub const fn new(t: T) -> Self {
        Self { t }
    }
}

impl<T: std::fmt::Debug + std::clone::Clone + Copy> Texture<T> for ConstantTexture<T> {
    fn sample(&self, _uv: &Point2) -> T {
        self.t.clone()
    }
}
