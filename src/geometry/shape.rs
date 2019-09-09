use crate::algebra::prelude::*;
use crate::geometry::geometry_information::GeometryInformation;

pub trait Shape: std::fmt::Debug {
    fn intersect(&self, ray: &Ray) -> Option<GeometryInformation>;
    fn does_intersect(&self, ray: &Ray) -> bool {
        self.intersect(ray).is_some()
    }

    fn bounds(&self) -> BoundingBox;
}
