pub mod bsdf;

use crate::algebra::prelude::*;
use crate::core::spectrum::RGBSpectrum;
use enumset::EnumSet;

#[repr(u8)]
#[derive(Debug, EnumSetType)]
pub enum BxDFType {
    Reflection,
    Transmission,
    Diffuse,
    Glossy,
    Specular,
}

pub trait BxDF {
    fn types(&self) -> EnumSet<BxDFType>;

    fn evaluate(&self, wo: &Vec3, wi: &Vec3) -> RGBSpectrum;
}
