use crate::algebra::prelude::*;
use crate::geometry::{geometry_information::GeometryInformation, shape::Shape};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Plane {
    pub origin: Point3,
    pub normal: Normal,
}

impl Plane {
    pub fn new(origin: Point3, normal: Normal) -> Self {
        Self { origin, normal }
    }
}

impl Shape for Plane {
    fn intersect(&self, ray: &Ray) -> Option<GeometryInformation> {
        let t = comb::dot(&(self.origin - ray.origin), &self.normal)
            / comb::dot(&self.normal, &ray.direction);
        if t < 0.001 {
            None
        } else {
            let p = ray.origin + (ray.direction * t);
            let uv = Point2::new(p.x, p.z);
            Some(GeometryInformation {
                origin: p,
                t,
                normal: self.normal,
                uv,
            })
        }
    }

    fn bounds(&self) -> BoundingBox {
        let far = Vec3::new(100_000.0, 100_000.0, 100_000.0);

        BoundingBox {
            min: Point3::from(self.origin - far + Vec3::from(self.normal) * far),
            max: Point3::from(self.origin + far - Vec3::from(self.normal) * far),
        }
    }
}
