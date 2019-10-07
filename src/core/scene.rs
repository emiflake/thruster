use crate::algebra::prelude::*;
use crate::core::aggregate::Aggregate;
use crate::core::interaction::Interaction;
use crate::core::primitive::Primitive;
use crate::light::Light;
use std::sync::Arc;

/// Represents a scene to be rendered
#[derive(Debug, Clone)]
pub struct Scene<'a> {
    pub lights: Vec<Arc<dyn Light + Sync + Send + 'a>>,
    pub aggregate: Arc<Aggregate>,
    pub bounds: BoundingBox,
}

impl<'a> Scene<'a> {
    pub fn new(aggregate: Arc<Aggregate>, lights: Vec<Arc<dyn Light + Send + Sync + 'a>>) -> Self {
        let bounds = aggregate.bounds();
        Self {
            bounds,
            lights,
            aggregate,
        }
    }

    pub fn intersect(&self, ray: &Ray) -> Option<Interaction> {
        self.aggregate.intersect(ray)
    }

    pub fn does_intersect(&self, ray: &Ray) -> bool {
        self.aggregate.does_intersect(ray)
    }
}
