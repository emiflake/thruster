use crate::acceleration::bvh::{BVHAccel, BVHConstructionAlgorithm, BVHLinearTree};
use crate::algebra::prelude::*;
use crate::bxdf::bsdf::BSDF;
use crate::core::interaction::Interaction;
use crate::core::material::Material;
use crate::core::primitive::Primitive;
use crate::light::area_light::AreaLight;

use std::sync::Arc;

#[derive(Debug)]
pub struct Aggregate {
    pub tree: BVHLinearTree,
}

impl Aggregate {
    pub fn from_primitives(primitives: Vec<Arc<dyn Primitive + Sync + Send>>) -> Self {
        let mut accel = BVHAccel::new(BVHConstructionAlgorithm::SAH, primitives);
        let (total, node) = accel.construct().expect("Could not construct BVHTree");
        let flat_bvh = accel.flatten(Box::new(node), total);

        Self { tree: flat_bvh }
    }
}

impl Aggregate {
    pub fn bounds(&self) -> BoundingBox {
        self.tree.bounds.clone()
    }

    pub fn intersect(&self, ray: &Ray) -> Option<Interaction> {
        self.tree.intersect(ray)
    }

    pub fn does_intersect(&self, ray: &Ray) -> bool {
        self.tree.does_intersect(ray)
    }
}
