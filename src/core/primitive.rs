use crate::algebra::prelude::*;
use crate::bxdf::bsdf::BSDF;
use crate::core::interaction::Interaction;
use crate::core::material::{Material, BRDF};
use crate::core::medium::MediumInterface;
use crate::core::spectrum::RGBSpectrum;
use crate::geometry::geometry_information::GeometryInformation;
use crate::geometry::shape::Shape;
use std::sync::Arc;

pub trait Primitive: std::fmt::Debug {
    fn bounds(&self) -> BoundingBox;
    fn intersect(&self, ray: &Ray) -> Option<GeometryInformation>;
    fn does_intersect(&self, ray: &Ray) -> bool {
        self.intersect(ray).is_some()
    }

    fn mat<'a>(&'a self) -> Arc<dyn Material + 'a>;
    fn light_emission(&self) -> RGBSpectrum;

    fn compute_scattering_functions(&self, interaction: &Interaction) -> BRDF;
}

#[derive(Debug)]
pub struct GeometricPrimitive<'a> {
    pub shape: Arc<dyn Shape + 'a>,
    pub material: Arc<dyn Material + 'a>,
    pub emission: RGBSpectrum,
    pub medium_interface: MediumInterface<'a>,
}

impl<'a> Primitive for GeometricPrimitive<'a> {
    fn bounds(&self) -> BoundingBox {
        self.shape.bounds()
    }

    fn intersect(&self, ray: &Ray) -> Option<GeometryInformation> {
        self.shape.intersect(ray)
    }

    fn compute_scattering_functions(&self, interaction: &Interaction) -> BRDF {
        self.material.compute_scattering_functions(interaction)
    }

    fn does_intersect(&self, ray: &Ray) -> bool {
        self.shape.does_intersect(ray)
    }

    fn mat<'b>(&'b self) -> Arc<dyn Material + 'b> {
        Arc::clone(&self.material)
    }

    fn light_emission(&self) -> RGBSpectrum {
        self.emission
    }
}
