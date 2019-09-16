pub mod area_light;

use crate::algebra::prelude::*;
use crate::core::spectrum::RGBSpectrum;

pub trait Light: std::fmt::Debug {
    fn le(&self, ray: &Ray) -> RGBSpectrum {
        RGBSpectrum::BLACK
    }
}
