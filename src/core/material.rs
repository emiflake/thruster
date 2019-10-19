use crate::algebra::prelude::*;
use crate::bxdf::bsdf::BSDF;
use crate::core::interaction::Interaction;
use crate::core::spectrum::RGBSpectrum;
use crate::core::texture::Texture;
use crate::core::transport::TransportMode;
use std::sync::Arc;

pub enum BRDF {
    Matte,
    Reflective,
}

pub trait Material: std::fmt::Debug + Send + Sync {
    fn compute_scattering_functions(&self, interaction: &Interaction) -> BRDF;
    fn albedo(&self, uv: &Point2) -> RGBSpectrum;
}

#[derive(Debug)]
pub struct Matte<'a> {
    pub kd: Arc<dyn Texture<RGBSpectrum> + 'a>,
}
impl<'a> Material for Matte<'a> {
    fn albedo(&self, uv: &Point2) -> RGBSpectrum {
        self.kd.sample(uv)
    }

    fn compute_scattering_functions(&self, interaction: &Interaction) -> BRDF {
        BRDF::Matte
    }
}

#[derive(Debug)]
pub struct Glossy<'a> {
    pub kd: Arc<dyn Texture<RGBSpectrum> + 'a>,
}

impl<'a> Material for Glossy<'a> {
    fn albedo(&self, uv: &Point2) -> RGBSpectrum {
        self.kd.sample(uv)
    }

    fn compute_scattering_functions(&self, interaction: &Interaction) -> BRDF {
        BRDF::Reflective
    }
}
