use crate::algebra::prelude::*;
use crate::geometry::{geometry_information::GeometryInformation, shape::Shape};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sphere {
    pub origin: Point3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(origin: Point3, radius: f64) -> Self {
        Self { origin, radius }
    }
}

impl Shape for Sphere {
    fn bounds(&self) -> BoundingBox {
        BoundingBox {
            min: self.origin - Vec3::new(1.0, 1.0, 1.0) * self.radius,
            max: self.origin + Vec3::new(1.0, 1.0, 1.0) * self.radius,
        }
    }

    fn intersect(&self, ray: &Ray) -> Option<GeometryInformation> {
        let local_ray = self.origin - ray.origin;
        let tca = comb::dot(&local_ray, &ray.direction);
        if tca < 0.0 {
            return None;
        }
        let d2 = local_ray.length2() - tca * tca;
        if d2 > self.radius * self.radius {
            return None;
        }
        let thc = (self.radius * self.radius - d2).sqrt();
        let t0 = tca - thc;
        let t1 = tca + thc;
        if t0 < 0.0 && t1 < 0.0 {
            return None;
        }
        let t = if t0 < 0.0 {
            t1
        } else if t1 < 0.0 {
            t0
        } else {
            t0.min(t1)
        };
        if t <= 0.0 {
            return None;
        }
        let p = ray.origin + (ray.direction * t);
        let normal = Normal::from((p - self.origin).normalized());
        let uv = Point2::new(
            0.5 + normal.z.atan2(normal.x) / std::f64::consts::PI / 2.0,
            0.5 - normal.y.asin() / std::f64::consts::PI,
        );
        Some(GeometryInformation {
            t,
            origin: p,
            normal,
            uv,
        })
    }
}
