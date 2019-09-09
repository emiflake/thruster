use crate::acceleration::bvh::{BVHAccel, BVHConstructionAlgorithm, BVHLinearTree};
use crate::algebra::prelude::*;
use crate::core::intersection::Intersection;
use crate::core::material::Material;
use crate::core::primitive::Primitive;
use crate::light::area_light::AreaLight;

use std::sync::Arc;

#[derive(Debug)]
pub struct Aggregate {
    pub tree: BVHLinearTree,
}

impl Aggregate {
    pub fn from_primitives(primitives: Vec<Arc<dyn Primitive>>) -> Self {
        let mut accel = BVHAccel::new(BVHConstructionAlgorithm::SAH, primitives);
        let (total, node) = accel.construct().expect("Could not construct BVHTree");
        let flat_bvh = accel.flatten(Box::new(node), total);

        Self { tree: flat_bvh }
    }
}

impl Primitive for Aggregate {
    fn bounds(&self) -> BoundingBox {
        self.tree.bounds.clone()
    }

    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        self.tree.intersect(ray)
    }

    fn does_intersect(&self, ray: &Ray) -> bool {
        self.tree.does_intersect(ray)
    }

    fn mat<'a>(&'a self) -> Option<Arc<dyn Material + 'a>> {
        unimplemented!("Don't get mat from an Aggregate")
    }
    fn area_light(&self) -> Option<Arc<AreaLight>> {
        unimplemented!("Don't get area light from an Aggregate")
    }
}
