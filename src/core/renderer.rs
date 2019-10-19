use crate::algebra::prelude::*;
use crate::core::camera::Camera;
use crate::core::material::{Material, BRDF};
use crate::core::scene::Scene;
use crate::core::spectrum::RGBSpectrum;
use crate::denoise::Denoiser;
use crate::sampler::{RandomSamplerConstructor, Sampler};

use scoped_threadpool::Pool;

use image::{ImageBuffer, Rgba};
use std::sync::{Arc, Mutex};

pub struct RenderOutput {
    pub buf: ImageBuffer<Rgba<u8>, Vec<u8>>,
}

impl RenderOutput {
    pub fn save(&self, path: &str) {
        self.buf.save(path).expect("Could not save to file");
    }

    pub fn denoise(&mut self) {
        Denoiser.denoise(&mut self.buf);
    }
}

pub trait Renderer {
    type ConfigurationType;
    fn render_scene(
        &self,
        scene: &Scene,
        configuration: Self::ConfigurationType,
        out: &mut RenderOutput,
    );
}

pub struct BasicRenderer<'a> {
    pub sampler_const: RandomSamplerConstructor,
    pub camera: Arc<dyn Camera + 'a>,
}

// TODO: move this
fn create_coordinate_system(n: &Vec3) -> (Vec3, Vec3) {
    let nt = if n.x.abs() > n.y.abs() {
        Vec3::new(n.z, 0.0, -n.x) / (n.x * n.x + n.z * n.z).sqrt()
    } else {
        Vec3::new(0.0, -n.z, n.y) / (n.y * n.y + n.z * n.z).sqrt()
    };
    (nt, comb::cross(&nt, n))
}

impl<'a> BasicRenderer<'a> {
    pub fn new(sampler_const: RandomSamplerConstructor, camera: Arc<dyn Camera + 'a>) -> Self {
        Self {
            sampler_const,
            camera,
        }
    }

    pub fn li(
        &self,
        ray: &Ray,
        scene: &Scene,
        depth: usize,
        samp: &mut dyn Sampler,
    ) -> RGBSpectrum {
        if depth == 0 {
            return RGBSpectrum::from_rgb(0.0, 0.0, 0.0);
        }
        match scene.intersect(ray) {
            Some(isect) => {
                let mut col = RGBSpectrum::BLACK;
                col += isect.light_emission();

                match isect.compute_scattering_functions(ray) {
                    BRDF::Matte => {
                        let (nt, nb) = create_coordinate_system(&Vec3::from(isect.geom.normal));
                        let uv = samp.get_2d();
                        let hemi = Vec3::hemisphere(uv.x, uv.y);
                        let sample_world = Vec3::new(
                            hemi.x * nb.x + hemi.y * isect.geom.normal.x + hemi.z * nt.x,
                            hemi.x * nb.y + hemi.y * isect.geom.normal.y + hemi.z * nt.y,
                            hemi.x * nb.z + hemi.y * isect.geom.normal.z + hemi.z * nt.z,
                        );
                        let ray = Ray::new(isect.geom.origin, sample_world);
                        col += self
                            .li(&ray, scene, depth - 1, samp)
                            .mul_with(isect.primitive.mat().albedo(&Point2::new(0.0, 0.0)));
                    }
                    BRDF::Reflective => {
                        let refl = ray.direction.reflect(&Vec3::from(isect.geom.normal));
                        let specularity = 0.1;
                        let random_p = Vec3::new(samp.get_1d(), samp.get_1d(), samp.get_1d())
                            - Vec3::new(0.5, 0.5, 0.5);
                        let dir = refl + (random_p * specularity);

                        let ray = Ray::new(isect.geom.origin, dir.normalized());

                        col += self
                            .li(&ray, scene, depth - 1, samp)
                            .mul_with(isect.primitive.mat().albedo(&Point2::new(0.0, 0.0)));
                    }
                }

                col
            }
            None => RGBSpectrum::BLACK,
        }
    }
}

impl<'a> Renderer for BasicRenderer<'a> {
    type ConfigurationType = ();

    fn render_scene(&self, scene: &Scene, _: (), out: &mut RenderOutput) {
        let (w, h) = out.buf.dimensions();

        let mut pool = Pool::new(12);
        pool.scoped(|scoped| {
            for (i, row) in out.buf.enumerate_rows_mut() {
                scoped.execute(move || {
                    println!("{}/{}", i, h);
                    for (x, y, pix) in row {
                        let mut samp = self.sampler_const.construct();
                        let mut contribution = RGBSpectrum::BLACK;
                        samp.start_pixel(Pixel::new(x as usize, y as usize));
                        'sample_loop: loop {
                            let mut camera_sample = samp.get_camera_sample();
                            camera_sample.film_pos =
                                Point2::new(f64::from(x), f64::from(y)) + camera_sample.film_pos;
                            let ray = self.camera.generate_ray(&camera_sample);
                            let l = self.li(&ray, scene, 5, &mut samp);
                            contribution.add_contribution(l, samp.spp());
                            if !samp.start_next_sample() {
                                break 'sample_loop;
                            }
                        }
                        contribution.clamp();
                        *pix = Rgba([
                            contribution[0].floor() as u8,
                            contribution[1].floor() as u8,
                            contribution[2].floor() as u8,
                            255,
                        ]);
                    }
                });
            }
        });
    }
}
