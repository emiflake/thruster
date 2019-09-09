use crate::algebra::prelude::*;
use crate::core::intersection::Intersection;
use crate::core::material::Material;
use crate::core::medium::MediumInterface;
use crate::geometry::shape::Shape;
use crate::light::area_light::AreaLight;
use std::sync::Arc;

pub trait Primitive: std::fmt::Debug {
    fn bounds(&self) -> BoundingBox;
    fn intersect(&self, ray: &Ray) -> Option<Intersection>;
    fn does_intersect(&self, ray: &Ray) -> bool {
        self.intersect(ray).is_some()
    }

    fn mat<'a>(&'a self) -> Option<Arc<dyn Material + 'a>>;
    fn area_light(&self) -> Option<Arc<AreaLight>>;

    // TODO: needs thinking
    //fn compute_scattering_functions(
    //&self,
    //intersection: &Intersection,
    //mode: TransportMode,
    //allow_multiple_lobes: bool,
    //) -> ScatteringFunction;
}

#[derive(Debug)]
pub struct GeometricPrimitive<'a> {
    pub shape: Arc<dyn Shape + 'a>,
    pub material: Arc<dyn Material + 'a>,
    pub area_light: Arc<AreaLight>,
    pub medium_interface: MediumInterface<'a>,
}

impl<'a> Primitive for GeometricPrimitive<'a> {
    fn bounds(&self) -> BoundingBox {
        self.shape.bounds()
    }

    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let geom = self.shape.intersect(ray)?;

        Some(Intersection { geom })
    }

    fn does_intersect(&self, ray: &Ray) -> bool {
        self.shape.does_intersect(ray)
    }

    fn mat<'b>(&'b self) -> Option<Arc<dyn Material + 'b>> {
        Some(Arc::clone(&self.material))
    }

    fn area_light(&self) -> Option<Arc<AreaLight>> {
        Some(Arc::clone(&self.area_light))
    }
}
