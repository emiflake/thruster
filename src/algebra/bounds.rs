use crate::algebra::prelude::*;
use serde::{Deserialize, Serialize};

/// A Bounding Box to represent the maximum range of an object, this is useful for Ray intersection
/// checking since it will guarantee that any Ray that can intersect the object, will also
/// intersect with this BoundingBox. Shapes must implement a function that generates this
/// BoundingBox such that they can in general be optimized with the
/// [BVHTree](../bvh/struct.BVHTree.html)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoundingBox {
    /// The vector containing the low values of the bounding box
    pub min: Point3,
    /// The vector containing the high values of the bounding box
    pub max: Point3,
}

impl BoundingBox {
    pub const EMPTY: Self = Self {
        min: Point3 {
            x: std::f64::MAX,
            y: std::f64::MAX,
            z: std::f64::MAX,
        },
        max: Point3 {
            x: std::f64::MIN,
            y: std::f64::MIN,
            z: std::f64::MIN,
        },
    };

    // TODO: take a look at this
    /// Extract a bound
    fn bounds(&self, sign: f64) -> Point3 {
        if sign >= 1.0 {
            self.min
        } else {
            self.max
        }
    }

    /// Get the centre of the BoundingBox
    pub fn centre(&self) -> Point3 {
        (self.min + self.max) / 2.0
    }

    /// Get the diagonal vector from min to max
    pub fn diagonal(&self) -> Vec3 {
        self.max - self.min
    }

    /// Returns the dimension for which the box's extent is biggest
    pub fn max_extent(&self) -> usize {
        let diag = self.diagonal();
        if diag.x > diag.y && diag.x > diag.z {
            0
        } else if diag.y > diag.z {
            1
        } else {
            2
        }
    }

    /// Merge two BoundingBox by joining them
    pub fn merge(&self, other: &BoundingBox) -> Self {
        Self {
            min: self.min.min(&other.min),
            max: self.max.max(&other.max),
        }
    }

    pub fn merge_with_vec(&self, v: &Vec3) -> Self {
        let p = Point3::from(*v);
        Self {
            min: self.min.min(&p),
            max: self.max.max(&p),
        }
    }

    pub fn merge_with_point(&self, v: &Point3) -> Self {
        Self {
            min: self.min.min(v),
            max: self.max.max(v),
        }
    }

    pub fn offset(&self, v: &Point3) -> Vec3 {
        let mut o = *v - self.min;
        if self.max.x > self.min.x {
            o.x /= self.max.x - self.min.x;
        }
        if self.max.y > self.min.y {
            o.y /= self.max.y - self.min.y;
        }
        if self.max.z > self.min.z {
            o.z /= self.max.z - self.min.z;
        }
        o
    }

    pub fn surface_area(&self) -> f64 {
        let d = self.diagonal();
        2.0 * (d.x * d.y + d.x * d.z + d.y * d.z)
    }
}

use crate::geometry::geometry_information::GeometryInformation;
use crate::geometry::shape::Shape;

impl Shape for BoundingBox {
    fn bounds(&self) -> Self {
        self.clone()
    }

    fn intersect(&self, ray: &Ray) -> Option<GeometryInformation> {
        // Leave unimplemented
        // TODO: come up with better solution
        unimplemented!("Intersection is not valid for Bounds");
    }

    fn does_intersect(&self, ray: &Ray) -> bool {
        // TODO: take a look at inv_dir infinites
        let inv_dir = Vec3::new(
            1.0 / ray.direction.x,
            1.0 / ray.direction.y,
            1.0 / ray.direction.z,
        );
        let sign = Vec3::new(
            if ray.direction.x > 0.0 { 1.0 } else { 0.0 },
            if ray.direction.y > 0.0 { 1.0 } else { 0.0 },
            if ray.direction.z > 0.0 { 1.0 } else { 0.0 },
        );

        let mut tmin = (self.bounds(sign.x).x - ray.origin.x) * inv_dir.x;
        let mut tmax = (self.bounds(1.0 - sign.x).x - ray.origin.x) * inv_dir.x;
        let tymin = (self.bounds(sign.y).y - ray.origin.y) * inv_dir.y;
        let tymax = (self.bounds(1.0 - sign.y).y - ray.origin.y) * inv_dir.y;

        if (tmin > tymax) || (tymin > tmax) {
            return false;
        }

        if tymin > tmin {
            tmin = tymin;
        }
        if tymax < tmax {
            tmax = tymax;
        }

        let tzmin = (self.bounds(sign.z).z - ray.origin.z) * inv_dir.z;
        let tzmax = (self.bounds(1.0 - sign.z).z - ray.origin.z) * inv_dir.z;

        if (tmin > tzmax) || (tzmin > tmax) {
            return false;
        }

        if tzmin > tmin {
            tmin = tzmin;
        }
        if tzmax < tmax {
            tmax = tzmax;
        }

        tmin >= 0.0 || tmax >= 0.0
    }
}
