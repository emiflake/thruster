use crate::algebra::prelude::*;
use crate::core::spectrum::RGBSpectrum;
use crate::sampler::Sampler;

#[derive(Debug, Clone)]
pub struct MediumInteraction;

pub trait Medium: std::fmt::Debug {
    fn tr(&self, ray: &Ray, sampler: &dyn Sampler) -> RGBSpectrum;
    fn sample(&self, ray: &Ray, sampler: &dyn Sampler) -> MediumInteraction;
}

#[derive(Debug, Clone)]
pub struct HomogeneousMedium {
    pub sigma_a: RGBSpectrum,
    pub sigma_s: RGBSpectrum,
    pub sigma_t: RGBSpectrum,
    pub g: f64,
}

impl Default for HomogeneousMedium {
    fn default() -> Self {
        Self {
            sigma_a: RGBSpectrum::from_rgb(255.0, 255.0, 255.0),
            sigma_s: RGBSpectrum::from_rgb(255.0, 255.0, 255.0),
            sigma_t: RGBSpectrum::from_rgb(255.0, 255.0, 255.0),
            g: 1.0,
        }
    }
}

impl Medium for HomogeneousMedium {
    fn tr(&self, ray: &Ray, sampler: &dyn Sampler) -> RGBSpectrum {
        RGBSpectrum::BLACK
        //(-self.sigma_t * (ray.max_t * ray.direction.length())).exp()
    }

    fn sample(&self, ray: &Ray, sampler: &dyn Sampler) -> MediumInteraction {
        MediumInteraction
    }
}

#[derive(Debug)]
pub struct MediumInterface<'a> {
    pub inside: Box<dyn Medium + 'a>,
    pub outside: Box<dyn Medium + 'a>,
}
