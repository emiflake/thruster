use crate::algebra::prelude::*;
use crate::bxdf::bsdf::BSDF;
use crate::core::material::BRDF;
use crate::core::primitive::Primitive;
use crate::core::spectrum::RGBSpectrum;
use crate::geometry::geometry_information::GeometryInformation;

use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Interaction<'a> {
    pub geom: GeometryInformation,
    pub primitive: Arc<dyn Primitive + Sync + Send + 'a>,
}

impl<'a> Interaction<'a> {
    pub fn nearest(self, other: Interaction<'a>) -> Self {
        if self.geom.t < other.geom.t {
            self
        } else {
            other
        }
    }

    pub fn compute_scattering_functions(&self, ray: &Ray) -> BRDF {
        self.primitive.compute_scattering_functions(self)
    }

    pub fn light_emission(&self) -> RGBSpectrum {
        self.primitive.light_emission()
    }
}
