/* ************************************************************************** */
/*                                                                            */
/*                                                        ::::::::            */
/*   main.rs                                            :+:    :+:            */
/*                                                     +:+                    */
/*   By: nmartins <nmartins@student.codam.nl>         +#+                     */
/*                                                   +#+                      */
/*   Created: 2019/07/19 18:06:37 by nmartins       #+#    #+#                */
/*   Updated: 2019/07/20 20:16:34 by nmartins      ########   odam.nl         */
/*                                                                            */
/* ************************************************************************** */

#![feature(clamp)]

extern crate quicksilver;

use quicksilver::{
	geom::*,
	graphics::{Background::Col, Color},
	lifecycle::{run, Settings, State, Window},
	Result,
};

mod camera;
mod material;
mod shape;

pub const SCREEN_WIDTH: f64 = 800.0;
pub const SCREEN_HEIGHT: f64 = 600.0;

use camera::{Camera, PerspectiveCamera};
use material::Material;
use shape::{Intersectable, Sphere, Vec3};

struct Thruster {
	pub camera: PerspectiveCamera,
	pub shapes: Vec<Box<dyn Intersectable>>,
}

impl State for Thruster {
	fn new() -> Result<Thruster> {
		Ok(Thruster {
			camera: PerspectiveCamera::new(Vec3::ORIGIN, SCREEN_WIDTH / SCREEN_HEIGHT),
			shapes: vec![Box::new(Sphere {
				origin: Vec3::new(0.0, 0.0, 5.0),
				radius: 2.5,
				material: Material {
					color: Vec3::new(0.0, 255.0, 0.0),
				},
			})],
		})
	}

	fn draw(&mut self, window: &mut Window) -> Result<()> {
		window.clear(Color::WHITE)?;
		println!("{}", window.current_fps());
		for y in 0..(SCREEN_HEIGHT as usize) {
			for x in 0..(SCREEN_WIDTH as usize) {
				let ray = self.camera.project_ray((x as f64, y as f64));

				if let Some(color) = ray.cast(&self.shapes) {
					window.draw(
						&Rectangle::new((x as u32, y as u32), (1, 1)),
						Col(Color::from_rgba(color.x.clamp(0.0, 255.0) as u8, color.y.clamp(0.0, 255.0) as u8, color.z.clamp(0.0, 255.0) as u8, 1.0)),
					);
				} else {
					window.draw(
						&Rectangle::new((x as u32, y as u32), (1, 1)),
						Col(Color::from_rgba(0, 0, 0, 1.0)),
					);
				}
			}
		}
		Ok(())
	}
}
fn main() {
	run::<Thruster>(
		"The Thruster Raytracer",
		Vector::new(SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32),
		Settings::default(),
	);
}
