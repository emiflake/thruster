use crate::bxdf::bsdf::BSDF;
use crate::core::interaction::Interaction;
use crate::core::spectrum::RGBSpectrum;
use crate::core::texture::Texture;
use crate::core::transport::TransportMode;
use std::sync::Arc;

pub trait Material: std::fmt::Debug {
    fn compute_scattering_functions(&self, interaction: &Interaction) -> BSDF;
}

#[derive(Debug)]
pub struct Matte<'a> {
    pub kd: Arc<dyn Texture<RGBSpectrum> + 'a>,
}
impl<'a> Material for Matte<'a> {
    fn compute_scattering_functions(&self, interaction: &Interaction) -> BSDF {
        unimplemented!("!!!");
    }
}
