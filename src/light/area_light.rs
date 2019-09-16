use crate::algebra::prelude::*;
use crate::core::spectrum::RGBSpectrum;
use crate::core::texture::Texture;

#[derive(Debug)]
pub struct AreaLight;

pub struct InfiniteAreaLight {
    pub lmap: Box<dyn Texture<RGBSpectrum>>,
    pub world_centre: Point3,
    pub world_radius: f64,
}
