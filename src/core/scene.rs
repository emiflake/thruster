use crate::algebra::prelude::*;
use crate::core::intersection::Intersection;
use crate::core::primitive::Primitive;
use crate::light::Light;
use std::sync::Arc;

/// Represents a scene to be rendered
#[derive(Debug, Clone)]
pub struct Scene<'a> {
    pub lights: Vec<Arc<dyn Light + 'a>>,
    pub aggregate: Arc<dyn Primitive + 'a>,
    pub bounds: BoundingBox,
}

impl<'a> Scene<'a> {
    pub fn new(aggregate: Arc<dyn Primitive + 'a>, lights: Vec<Arc<dyn Light + 'a>>) -> Self {
        let bounds = aggregate.bounds();
        Self {
            bounds,
            lights,
            aggregate,
        }
    }

    pub fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        self.aggregate.intersect(ray)
    }

    pub fn does_intersect(&self, ray: &Ray) -> bool {
        self.aggregate.does_intersect(ray)
    }
}
