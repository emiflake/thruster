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
use crate::camera::{Camera, PerspectiveCamera};
use crate::image::{ImageBuffer, Rgba};
use crate::lightsource::Lightsource;
use crate::shape::Shape;
use crate::skybox::Skybox;
use crate::texture_map::TextureMap;

use scoped_threadpool::Pool;

pub struct Scene<'a> {
    pub camera: PerspectiveCamera,
    pub shapes: Vec<Shape<'a>>,
    pub lights: Vec<Box<dyn Lightsource + Sync + 'a>>,
    pub texture_map: TextureMap,
    pub skybox: Skybox,
}

impl Scene<'_> {
    pub fn screenshot(&self, filename: &'static str, w: f64, h: f64) -> Result<(), String> {
        let before = SystemTime::now();
        let screenshot = self.render_to_buffer(w, h);

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

    pub fn render_to_buffer(&self, w: f64, h: f64) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let mut buf = ImageBuffer::new(w as u32, h as u32);
        let mut pool = Pool::new(12);
        pool.scoped(|scoped| {
            for (_, row) in buf.enumerate_rows_mut() {
                scoped.execute(move || {
                    for (x, y, pix) in row {
                        let ray = self
                            .camera
                            .project_ray((f64::from(x), f64::from(y)), (w, h));
                        let intersections = ray.cast(&self);
                        if let Some(color) = ray.color_function(intersections, self) {
                            *pix = Rgba([color.x as u8, color.y as u8, color.z as u8, 255]);
                        } else {
                            let color = self
                                .skybox
                                .calc_color(self, ray.direction)
                                .unwrap_or(Vec3::ORIGIN);
                            *pix = Rgba([color.x as u8, color.y as u8, color.z as u8, 255]);
                        }
                    }
                })
            }
        });

        //use crate::dither::{ColorPalette, Dither};
        //let dither = Dither {
        //palette: ColorPalette::vga_palette(),
        //};

        //dither.dither_image(buf)
        buf
    }
}
