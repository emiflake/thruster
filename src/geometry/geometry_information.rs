use crate::algebra::prelude::*;

#[derive(Debug, Clone)]
pub struct GeometryInformation {
    /// The 'distance' the ray hit at. This is derived from `p = rO + t * rD`
    pub t: f64,

    /// The normal from the shape at the intersection
    pub normal: Normal,

    /// The position the ray hit the object at
    pub origin: Point3,

    /// The texture position in UV-space that the ray intersects
    pub uv: Point2,
}
