/* ************************************************************************** */
/*                                                                            */
/*                                                        ::::::::            */
/*   main.rs                                            :+:    :+:            */
/*                                                     +:+                    */
/*   By: nmartins <nmartins@student.codam.nl>         +#+                     */
/*                                                   +#+                      */
/*   Created: 2019/07/19 18:06:37 by nmartins       #+#    #+#                */
/*   Updated: 2019/07/21 13:26:14 by nmartins      ########   odam.nl         */
/*                                                                            */
/* ************************************************************************** */

extern crate image;

extern crate sdl2;

mod camera;
mod lightsource;
mod material;
mod shape;

pub const SCREEN_WIDTH: f64 = 1280.0;
pub const SCREEN_HEIGHT: f64 = 720.0;

use camera::{Camera, PerspectiveCamera};
use lightsource::{Lightsource, PointLight};
use material::Material;
use shape::{Intersectable, Plane, Sphere, Vec3};

pub struct Thruster {
	pub camera: PerspectiveCamera,
	pub shapes: Vec<Box<dyn Intersectable>>,
	pub lights: Vec<Box<dyn Lightsource>>,
}

// impl State for Thruster {
// 	fn new() -> Result<Thruster> {
// 		Ok(Thruster {
// 			camera: PerspectiveCamera::new(Vec3::ORIGIN, SCREEN_WIDTH / SCREEN_HEIGHT),
// 			shapes: vec![
// 				Box::new(Sphere {
// 					origin: Vec3::new(0.0, 0.0, 5.0),
// 					radius: 1.0,
// 					material: Material {
// 						color: Vec3::new(0.0, 255.0, 0.0),
// 					},
// 				},
// 			)],
// 			lights: vec![
// 				Box::new(PointLight {
// 					origin: Vec3::new(0.0, 10.0, 0.0),
// 					color: Vec3::new(255.0, 255.0, 255.0),
// 				}),
// 			],
// 		})
// 	}

// 	fn draw(&mut self, window: &mut Window) -> Result<()> {
// 		window.clear(Color::WHITE)?;
// 		println!("{}", window.current_fps());
// for y in 0..(SCREEN_HEIGHT as usize) {
// 	for x in 0..(SCREEN_WIDTH as usize) {
// 		let ray = self.camera.project_ray((x as f64, y as f64));

// 		if let Some(color) = ray.cast(&self) {
// 			window.draw(
// 				&Rectangle::new((x as u32, y as u32), (1, 1)),
// 				Col(Color::from_rgba(color.x.clamp(0.0, 255.0) as u8, color.y.clamp(0.0, 255.0) as u8, color.z.clamp(0.0, 255.0) as u8, 1.0)),
// 			);
// 		}
// 	}
// }
// 		Ok(())
// 	}
// }
// fn main() {
// 	run::<Thruster>(
// 		"The Thruster Raytracer",
// 		Vector::new(SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32),
// 		Settings::default(),
// 	);
// }

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;

pub fn main() -> std::result::Result<(), String> {
	let sdl_context = sdl2::init().unwrap();
	let video_subsystem = sdl_context.video().unwrap();

	let window = video_subsystem
		.window(
			"Thruster Raytracer",
			SCREEN_WIDTH as u32,
			SCREEN_HEIGHT as u32,
		)
		.position_centered()
		.build()
		.unwrap();

	use crate::material::MatTex;
	let thruster = Thruster {
		camera: PerspectiveCamera::new(Vec3::ORIGIN, SCREEN_WIDTH / SCREEN_HEIGHT),
		shapes: vec![
			Box::new(Sphere {
				origin: Vec3::new(0.0, 0.5, 5.0),
				radius: 1.0,
				material: Material::diffuse(
					MatTex::load_from_file("wood.png").map_err(|_| "Error loading image")?,
				),
			}),
			Box::new(Plane {
				origin: Vec3::new(0.0, -1.0, 0.0),
				normal: Vec3::new(0.0, 1.0, 0.0),
				material: Material::reflective(
					MatTex::load_from_file("checker.png").map_err(|_| "Error loading image")?,
				),
			}),
		],
		lights: vec![Box::new(PointLight {
			origin: Vec3::new(0.0, 100.0, 0.0),
			color: Vec3::new(255.0, 255.0, 255.0),
		})],
	};

	let mut canvas = window.into_canvas().build().unwrap();

	canvas.clear();
	canvas.present();
	let mut event_pump = sdl_context.event_pump().unwrap();

	use std::time::{Duration, SystemTime};
	let mut last_frame = SystemTime::now();
	'running: loop {
		canvas.set_draw_color(Color::RGB(0, 0, 0));
		canvas.clear();
		for event in event_pump.poll_iter() {
			match event {
				Event::Quit { .. }
				| Event::KeyDown {
					keycode: Some(Keycode::Escape),
					..
				} => break 'running,
				_ => {}
			}
		}

		for y in 0..(SCREEN_HEIGHT as usize) {
			for x in 0..(SCREEN_WIDTH as usize) {
				let ray = thruster.camera.project_ray((x as f64, y as f64));

				if let Some(color) = ray.cast(&thruster) {
					canvas.set_draw_color(Color::RGB(color.x as u8, color.y as u8, color.z as u8));
					canvas.draw_point(Point::new(x as i32, y as i32))?;
				}
			}
		}
		let now = SystemTime::now();
		let delta = now
			.duration_since(last_frame)
			.expect("Could not get delta time");
		println!("{}", 1_000_000f64 / delta.as_micros() as f64);
		last_frame = now;
		canvas.present();
		::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
	}
	Ok(())
}
