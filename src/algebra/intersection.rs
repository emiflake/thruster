use crate::algebra::prelude::*;
/// Represents a found intersection between a Ray and an Object
#[derive(Debug, Clone, PartialEq)]
pub struct Intersection {
    /// The 'distance' the ray hit at. This is derived from ```p = rO + t * rD```
    pub t: f64,

    /// The normal from the shape at the intersection
    pub normal: Vec3,

    /// The position the ray hit the object at
    pub origin: Vec3,

    /// The texture position in UV-space that the ray intersects
    pub text_pos: Vec2,
}
