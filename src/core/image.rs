use image::{ImageBuffer, Rgb};
use std::sync::Arc;

use crate::algebra::prelude::*;
use crate::core::spectrum::RGBSpectrum;
use crate::core::texture::Texture;

#[derive(Debug)]
pub struct ImageTexture {
    pub image: Arc<ImageBuffer<Rgb<u8>, Vec<u8>>>,
}

impl Texture<RGBSpectrum> for ImageTexture {
    fn sample(&self, _uv: &Point2) -> RGBSpectrum {
        RGBSpectrum::BLACK
    }
}
