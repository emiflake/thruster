use crate::algebra::prelude::*;
use crate::core::camera::Camera;
use crate::core::scene::Scene;
use crate::core::spectrum::RGBSpectrum;
use crate::sampler::Sampler;

use image::{ImageBuffer, Rgb};
use std::sync::{Arc, Mutex};

pub struct RenderOutput {
    pub buf: ImageBuffer<Rgb<u8>, Vec<u8>>,
}

impl RenderOutput {
    pub fn save(&self, path: &str) {
        self.buf.save(path).expect("Could not save to file");
    }
}

pub trait Renderer {
    type ConfigurationType;
    fn render_scene(
        &mut self,
        scene: &Scene,
        configuration: Self::ConfigurationType,
        out: &mut RenderOutput,
    );
}

pub struct BasicRenderer<'a> {
    pub sampler: Arc<Mutex<dyn Sampler + 'a>>,
    pub camera: Arc<dyn Camera + 'a>,
}

impl<'a> BasicRenderer<'a> {
    pub fn new(sampler: Arc<Mutex<dyn Sampler + 'a>>, camera: Arc<dyn Camera + 'a>) -> Self {
        Self { sampler, camera }
    }

    pub fn li(&self, ray: &Ray, scene: &Scene, depth: usize) -> RGBSpectrum {
        if depth == 0 {
            return RGBSpectrum::from_rgb(0.0, 0.0, 0.0);
        }
        match scene.intersect(ray) {
            Some(isect) => {
                let bsdf = isect.compute_scattering_functions(ray);
                RGBSpectrum::from_rgb(255., 255., 255.)
            }
            None => RGBSpectrum::BLACK,
        }
    }
}

impl<'a> Renderer for BasicRenderer<'a> {
    type ConfigurationType = ();

    fn render_scene(&mut self, scene: &Scene, _: (), out: &mut RenderOutput) {
        let (w, h) = out.buf.dimensions();
        let mut samp = self
            .sampler
            .lock()
            .expect("Could not lock Mutex for Sampler");

        for (i, row) in out.buf.enumerate_rows_mut() {
            for (x, y, pix) in row {
                let mut contribution = RGBSpectrum::BLACK;
                samp.start_pixel(Pixel::new(x as usize, y as usize));
                'sample_loop: loop {
                    let mut camera_sample = samp.get_camera_sample();
                    camera_sample.film_pos = Point2::new(f64::from(x), f64::from(y));
                    let ray = self.camera.generate_ray(&camera_sample);
                    let l = self.li(&ray, scene, 5);
                    contribution.add_contribution(l, samp.spp());
                    if !samp.start_next_sample() {
                        break 'sample_loop;
                    }
                }
                *pix = Rgb([
                    contribution[0].floor() as u8,
                    contribution[1].floor() as u8,
                    contribution[2].floor() as u8,
                ]);
            }
        }
    }
}
