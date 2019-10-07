use std::sync::Arc;

use crate::algebra::prelude::*;

use crate::core::camera::CameraSample;

pub trait Sampler {
    fn start_pixel(&mut self, p: Pixel) {}

    fn sample_count(&self) -> usize;

    fn spp(&self) -> usize;

    fn get_1d(&mut self) -> f64;

    fn get_2d(&mut self) -> Point2 {
        Point2::new(self.get_1d(), self.get_1d())
    }

    fn get_camera_sample(&mut self) -> CameraSample {
        CameraSample {
            film_pos: self.get_2d(),
            time: self.get_1d(),
        }
    }

    fn start_next_sample(&mut self) -> bool;
}

use rand::prelude::*;

pub struct RandomSamplerConstructor {
    spp: usize,
}

impl RandomSamplerConstructor {
    pub fn new(spp: usize) -> Self {
        Self { spp }
    }

    pub fn construct(&self) -> RandomSampler {
        RandomSampler::new(self.spp)
    }
}

pub struct RandomSampler {
    rng: ThreadRng,
    spp: usize,
    sample_count: usize,
}

impl RandomSampler {
    pub fn new(spp: usize) -> Self {
        let rng = rand::thread_rng();
        Self {
            rng,
            spp,
            sample_count: 1,
        }
    }
}

impl Sampler for RandomSampler {
    fn spp(&self) -> usize {
        self.spp
    }

    fn sample_count(&self) -> usize {
        self.sample_count
    }

    fn get_1d(&mut self) -> f64 {
        self.rng.gen::<f64>()
    }

    fn start_next_sample(&mut self) -> bool {
        self.sample_count += 1;
        self.sample_count < self.spp
    }
}
