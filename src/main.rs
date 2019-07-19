/* ************************************************************************** */
/*                                                                            */
/*                                                        ::::::::            */
/*   main.rs                                            :+:    :+:            */
/*                                                     +:+                    */
/*   By: nmartins <nmartins@student.codam.nl>         +#+                     */
/*                                                   +#+                      */
/*   Created: 2019/07/19 18:06:37 by nmartins       #+#    #+#                */
/*   Updated: 2019/07/19 18:58:01 by nmartins      ########   odam.nl         */
/*                                                                            */
/* ************************************************************************** */

extern crate quicksilver;

use quicksilver::{
	geom::*,
	graphics::{Background::Col, Color},
	lifecycle::{run, Settings, State, Window},
	Result,
};

mod camera;
mod shape;

pub const SCREEN_WIDTH: f64 = 800.0;
pub const SCREEN_HEIGHT: f64 = 600.0;

use camera::{PerspectiveCamera, Camera};
use shape::{Intersectable, Vec3};

struct Thruster {
	pub camera: PerspectiveCamera,
	pub shapes: Vec<Box<Intersectable>>,
}

impl State for Thruster {
	fn new() -> Result<Thruster> {
		Ok(Thruster {
			camera: PerspectiveCamera::new(Vec3::ORIGIN),
			shapes: vec![],
		})
	}

	fn draw(&mut self, window: &mut Window) -> Result<()> {
		window.clear(Color::WHITE)?;
		for y in 0..(SCREEN_HEIGHT as usize) {
			for x in 0..(SCREEN_WIDTH as usize) {
				let ray = self.camera.project_ray((x as f64, y as f64));

				if let intersection = ray.cast(self.shapes) {
					window.draw(
						&Rectangle::new((x as u32, y as u32), (1, 1)), Col(Color::from_rgba(255, 255, 255, 1.0))
					);
				} else {
					window.draw(
						&Rectangle::new((x as u32, y as u32), (1, 1)), Col(Color::from_rgba(0, 0, 0, 1.0))
					);
				}
			}
		}
		window.draw(
			&Rectangle::new((0, 0), (100, 100)),
			Col(Color::from_rgba(0, 255, 255, 1.0)),
		);
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
