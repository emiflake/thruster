use crate::algebra::prelude::*;
use crate::bxdf::bsdf::BSDF;
use crate::core::interaction::Interaction;
use crate::core::material::Material;
use crate::core::medium::MediumInterface;
use crate::core::transport::TransportMode;
use crate::geometry::geometry_information::GeometryInformation;
use crate::geometry::shape::Shape;
use crate::light::area_light::AreaLight;
use std::sync::Arc;

pub trait Primitive: std::fmt::Debug {
    fn bounds(&self) -> BoundingBox;
    fn intersect(&self, ray: &Ray) -> Option<Interaction>;
    fn does_intersect(&self, ray: &Ray) -> bool {
        self.intersect(ray).is_some()
    }

    fn mat<'a>(&'a self) -> Option<Arc<dyn Material + 'a>>;
    fn area_light(&self) -> Option<Arc<AreaLight>>;

    fn compute_scattering_functions(&self, interaction: &Interaction) -> BSDF;
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

    fn intersect(&self, ray: &Ray) -> Option<Interaction> {
        self.shape.intersect(ray).map(|geom| Interaction {
            geom,
            primitive: Arc::new(self),
        })
    }

    fn compute_scattering_functions(&self, interaction: &Interaction) -> BSDF {
        self.material.compute_scattering_functions(interaction)
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
