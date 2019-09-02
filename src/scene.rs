/* ************************************************************************** */
/*                                                                            */
/*                                                        ::::::::            */
/*   thruster.rs                                        :+:    :+:            */
/*                                                     +:+                    */
/*   By: nmartins <nmartins@student.codam.nl>         +#+                     */
/*                                                   +#+                      */
/*   Created: 2019/07/21 17:25:15 by nmartins       #+#    #+#                */
/*   Updated: 2019/08/12 15:49:43 by nmartins      ########   odam.nl         */
/*                                                                            */
/* ************************************************************************** */

use std::time::SystemTime;

use crate::algebra::Vec3;
use crate::bvh::BVHTree;
use crate::camera::{Camera, PerspectiveCamera};
use crate::image::{ImageBuffer, Rgba};
use crate::lightsource::Lightsource;
use crate::render_config::RenderConfig;
use crate::shape::Shape;
use crate::skybox::Skybox;
use crate::texture_map::TextureMap;

use scoped_threadpool::Pool;

pub struct Scene<'a> {
    pub camera: PerspectiveCamera,
    //pub shapes: Vec<Shape<'a>>,
    pub bvh: BVHTree,
    pub lights: Vec<Box<dyn Lightsource + Sync + 'a>>,
    pub texture_map: TextureMap,
    pub skybox: Skybox,
    pub config: RenderConfig,
}

impl Scene<'_> {
    pub fn with_config(self, config: RenderConfig) -> Self {
        Self { config, ..self }
    }

    pub fn screenshot(&self, filename: &'static str, w: f64, h: f64) -> Result<(), String> {
        let before = SystemTime::now();
        let screenshot = self.new_render(w, h);

        screenshot
            .save_with_format(filename, image::ImageFormat::PNG)
            .map_err(|_| "Could not save screenshot")?;
        let after = SystemTime::now();
        let delta = after
            .duration_since(before)
            .expect("Could not get delta time");
        println!(
            "Took {}s to render image",
            delta.as_micros() as f64 / 1_000_000f64
        );
        Ok(())
    }

    pub fn new_render(&self, w: f64, h: f64) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let mut buf = ImageBuffer::new(w as u32, h as u32);

        self.render_to_buffer(w, h, &mut buf);

        buf
    }

    pub fn render_to_buffer(&self, w: f64, h: f64, buf: &mut ImageBuffer<Rgba<u8>, Vec<u8>>) {
        if self.config.multi_thread {
            let mut pool = Pool::new(12);
            pool.scoped(|scoped| {
                for (_, row) in buf.enumerate_rows_mut() {
                    scoped.execute(move || {
                        for (x, y, pix) in row {
                            let mut col = Vec3::ORIGIN;
                            let rays = self.camera.project_rays(
                                (f64::from(x), f64::from(y)),
                                (w, h),
                                &self,
                            );
                            for ray in rays {
                                let intersections = ray.cast(&self);
                                if let Some(color) = ray.color_function(intersections, self) {
                                    col = col + color;
                                } else {
                                    col = col
                                        + if self.config.skybox {
                                            self.skybox
                                                .calc_color(self, ray.direction)
                                                .unwrap_or(Vec3::ORIGIN)
                                        } else {
                                            Vec3::ORIGIN
                                        };
                                }
                            }
                            *pix = Rgba([col.x as u8, col.y as u8, col.z as u8, 255]);
                        }
                    })
                }
            });
        } else {
            for (_, row) in buf.enumerate_rows_mut() {
                for (x, y, pix) in row {
                    let mut col = Vec3::ORIGIN;
                    let rays =
                        self.camera
                            .project_rays((f64::from(x), f64::from(y)), (w, h), &self);
                    for ray in rays {
                        let intersections = ray.cast(&self);
                        if let Some(color) = ray.color_function(intersections, self) {
                            col = col + color;
                        } else {
                            col = col
                                + self
                                    .skybox
                                    .calc_color(self, ray.direction)
                                    .unwrap_or(Vec3::ORIGIN);
                        }
                    }
                    *pix = Rgba([col.x as u8, col.y as u8, col.z as u8, 255]);
                }
            }
        }

        let denoiser = crate::denoise::Denoiser;

        if self.config.denoise {
            denoiser.denoise(buf);
        }
    }
}
