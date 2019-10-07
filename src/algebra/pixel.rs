use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub struct Pixel {
    pub x: usize,
    pub y: usize,
}

impl Pixel {
    pub const fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

impl std::ops::Add<Pixel> for Pixel {
    type Output = Pixel;
    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl std::ops::Mul<Pixel> for Pixel {
    type Output = Pixel;
    fn mul(self, rhs: Self) -> Self {
        Self::new(self.x * rhs.x, self.y * rhs.y)
    }
}
