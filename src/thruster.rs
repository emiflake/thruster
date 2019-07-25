/* ************************************************************************** */
/*                                                                            */
/*                                                        ::::::::            */
/*   thruster.rs                                        :+:    :+:            */
/*                                                     +:+                    */
/*   By: nmartins <nmartins@student.codam.nl>         +#+                     */
/*                                                   +#+                      */
/*   Created: 2019/07/21 17:25:15 by nmartins       #+#    #+#                */
/*   Updated: 2019/07/25 19:12:42 by nmartins      ########   odam.nl         */
/*                                                                            */
/* ************************************************************************** */

use std::time::SystemTime;

use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};

use crate::camera::{Camera, PerspectiveCamera};
use crate::image::{ImageBuffer, Rgb};
use crate::lightsource::Lightsource;
use crate::shape::Intersectable;
use crate::texture_map::TextureMap;

use sdl2::pixels::Color;
use sdl2::rect::Point;

use std::sync::{Mutex, Arc};
use scoped_threadpool::Pool;


pub struct Thruster<'a> {
    pub camera: PerspectiveCamera,
    pub shapes: Vec<Box<dyn Intersectable + Sync + 'a>>,
    pub lights: Vec<Box<dyn Lightsource + Sync + 'a>>,
	pub texture_map: TextureMap,
}

impl Thruster<'_> {
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

    pub fn render_to_buffer(
        &self,
        w: f64,
        h: f64,
    ) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        let mut buf = ImageBuffer::new(w as u32, h as u32);
		let mut pool = Pool::new(4);
		
		pool.scoped(|scoped| {
			for (x, y, pix) in buf.enumerate_pixels_mut() {
				scoped.execute(move || {
					let ray = self.camera.project_ray((x as f64, y as f64), (w, h));
					let intersections = ray.cast(&self);
					if let Some(color) = ray.color_function(intersections, self) {
						*pix = Rgb([color.x as u8, color.y as u8, color.z as u8]);
					} else {
						*pix = Rgb([0, 0, 0]);
					}
				})
			}
		});

        buf
    }

    pub fn render(
        &self,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    ) -> Result<(), String> {
        let img = self.render_to_buffer(640.0, 480.0);

		let upscaled = image::imageops::resize(&img, SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32, image::FilterType::Lanczos3);

        for y in 0..(SCREEN_HEIGHT as usize) {
            for x in 0..(SCREEN_WIDTH as usize) {
                let Rgb([r, g, b]) = upscaled.get_pixel(x as u32, y as u32);
                canvas.set_draw_color(Color::RGB(*r, *g, *b));
                canvas.draw_point(Point::new(x as i32, y as i32))?;
            }
        }
        Ok(())
    }
}