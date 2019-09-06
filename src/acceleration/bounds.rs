use crate::algebra::Vec3;
use crate::shape::Ray;

/// A Bounding Box to represent the maximum range of an object, this is useful for Ray intersection
/// checking since it will guarantee that any Ray that can intersect the object, will also
/// intersect with this BoundingBox. Shapes must implement a function that generates this
/// BoundingBox such that they can in general be optimized with the
/// [BVHTree](../bvh/struct.BVHTree.html)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoundingBox {
    /// The vector containing the low values of the bounding box
    pub min_vector: Vec3,
    /// The vector containing the high values of the bounding box
    pub max_vector: Vec3,
}

impl BoundingBox {
    pub const EMPTY: Self = Self {
        min_vector: Vec3 {
            x: std::f64::MAX,
            y: std::f64::MAX,
            z: std::f64::MAX,
        },
        max_vector: Vec3 {
            x: std::f64::MIN,
            y: std::f64::MIN,
            z: std::f64::MIN,
        },
    };

    // TODO: take a look at this
    /// Extract a bound
    fn bounds(&self, sign: f64) -> Vec3 {
        if sign == 1.0 {
            self.min_vector
        } else {
            self.max_vector
        }
    }

    /// Get the centre of the BoundingBox
    pub fn centre(&self) -> Vec3 {
        (self.min_vector + self.max_vector) / 2.0
    }

    /// Check if the BoundingBox intersects with a Ray; used during acceleration
    pub fn intersects_with(&self, ray: &Ray) -> bool {
        let mut tmin = (self.bounds(ray.sign.x).x - ray.origin.x) * ray.inv_dir.x;
        let mut tmax = (self.bounds(1.0 - ray.sign.x).x - ray.origin.x) * ray.inv_dir.x;
        let tymin = (self.bounds(ray.sign.y).y - ray.origin.y) * ray.inv_dir.y;
        let tymax = (self.bounds(1.0 - ray.sign.y).y - ray.origin.y) * ray.inv_dir.y;

        if (tmin > tymax) || (tymin > tmax) {
            return false;
        }

        if tymin > tmin {
            tmin = tymin;
        }
        if tymax < tmax {
            tmax = tymax;
        }

        let tzmin = (self.bounds(ray.sign.z).z - ray.origin.z) * ray.inv_dir.z;
        let tzmax = (self.bounds(1.0 - ray.sign.z).z - ray.origin.z) * ray.inv_dir.z;

        if (tmin > tzmax) || (tzmin > tmax) {
            return false;
        }

        if tzmin > tmin {
            tmin = tzmin;
        }
        if tzmax < tmax {
            tmax = tzmax;
        }

        let mut t = tmin;

        if t < 0.0 {
            t = tmax;
            if t < 0.0 {
                return false;
            }
        }

        true
    }

    /// Get the diagonal vector from min to max
    pub fn diagonal(&self) -> Vec3 {
        self.max_vector - self.min_vector
    }

    /// Returns the dimension for which the box's extent is biggest
    pub fn max_extent(&self) -> u32 {
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
            min_vector: self.min_vector.min(&other.min_vector),
            max_vector: self.max_vector.max(&other.max_vector),
        }
    }

    pub fn merge_with_vec(&self, v: &Vec3) -> Self {
        Self {
            min_vector: self.min_vector.min(v),
            max_vector: self.max_vector.max(v),
        }
    }

    pub fn offset(&self, v: &Vec3) -> Vec3 {
        let mut o = *v - self.min_vector;
        if self.max_vector.x > self.min_vector.x {
            o.x /= self.max_vector.x - self.min_vector.x;
        }
        if self.max_vector.y > self.min_vector.y {
            o.y /= self.max_vector.y - self.min_vector.y;
        }
        if self.max_vector.z > self.min_vector.z {
            o.z /= self.max_vector.z - self.min_vector.z;
        }
        o
    }

    pub fn surface_area(&self) -> f64 {
        let d = self.diagonal();
        2.0 * (d.x * d.y + d.x * d.z + d.y * d.z)
    }
}
