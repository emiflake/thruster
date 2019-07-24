/* ************************************************************************** */
/*                                                                            */
/*                                                        ::::::::            */
/*   main.rs                                            :+:    :+:            */
/*                                                     +:+                    */
/*   By: nmartins <nmartins@student.codam.nl>         +#+                     */
/*                                                   +#+                      */
/*   Created: 2019/07/19 18:06:37 by nmartins       #+#    #+#                */
/*   Updated: 2019/07/24 20:57:51 by nmartins      ########   odam.nl         */
/*                                                                            */
/* ************************************************************************** */

extern crate image;
extern crate rand;
extern crate sdl2;

mod camera;
mod lightsource;
mod material;
mod shape;
mod thruster;

pub const SCREEN_WIDTH: f64 = 1280.0;
pub const SCREEN_HEIGHT: f64 = 720.0;

use camera::PerspectiveCamera;
use lightsource::PointLight;
use material::{MatTex, Material};
use shape::{Plane, Sphere, Triangle, Vec3};

#[allow(unused_imports)]
use sdl2::event::Event;
#[allow(unused_imports)]
use sdl2::keyboard::Keycode;

use std::time::Duration;

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

	let mut thruster = thruster::Thruster {
		camera: PerspectiveCamera::new(Vec3::ORIGIN, SCREEN_WIDTH / SCREEN_HEIGHT),
		shapes: vec![
			Box::new(Sphere {
				origin: Vec3::new(0.0, 0.5, 5.0),
				radius: 1.0,
				material: Material {
					c_diffuse: 0.8,
					c_ambient: 0.0,
					c_reflection: 0.2,
					texture: MatTex::load_from_file("earth.png")
						.map_err(|_| "Error loading earth image")?,
				},
			}),
			Box::new(Triangle {
				a: Vec3::new(-2.0, 2.0, 10.0),
				b: Vec3::new(2.0, 2.0, 10.0),
				c: Vec3::new(0.0, 5.0, 10.0),
				material: Material::diffuse(MatTex::Color(Vec3::new(255.0, 0.0, 0.0)))
			}),
			// Box::new(Sphere {
			// 	origin: Vec3::new(1.0, 0.5, 3.0),
			// 	radius: 0.5,
			// 	material: Material {
			// 		c_diffuse: 0.95,
			// 		c_ambient: 0.0,
			// 		c_reflection: 0.05,
			// 		texture: MatTex::load_from_file("wood.png")
			// 			.map_err(|_| "Error loading earth image")?,
			// 	},
			// }),
			Box::new(Plane {
				origin: Vec3::new(0.0, -1.0, 0.0),
				normal: Vec3::new(0.0, 1.0, 0.0),
				material: Material::reflective(
					MatTex::load_from_file("checker.png")
						.map_err(|_| "Error loading checker image")?,
				),
			}),
		],
		lights: vec![
			// Box::new(PointLight {
			// 	origin: Vec3::new(0.0, 100.0, 0.0),
			// 	color: Vec3::new(255.0, 255.0, 255.0),
			// }),
			Box::new(PointLight {
				origin: Vec3::new(1.0, 5.0, 5.0),
				color: Vec3::new(255.0, 255.0, 255.0),
			}),
			// Box::new(PointLight {
			// 	origin: Vec3::new(-50.0, 2.0, 0.0),
			// 	color: Vec3::new(255.0, 255.0, 255.0),
			// }),
		],
	};

	let mut canvas = window.into_canvas().build().unwrap();

	canvas.clear();
	canvas.present();
	let mut event_pump = sdl_context.event_pump().unwrap();

	'running: loop {
		for event in event_pump.poll_iter() {
			match event {
				Event::Quit { .. }
				| Event::KeyDown {
					keycode: Some(Keycode::Escape),
					..
				} => break 'running,
				Event::KeyDown { keycode, .. } => match keycode {
					Some(Keycode::E) => thruster.camera.position.y -= 0.1,
					Some(Keycode::Q) => thruster.camera.position.y += 0.1,
					Some(Keycode::S) => thruster.camera.position.z -= 0.1,
					Some(Keycode::W) => thruster.camera.position.z += 0.1,
					Some(Keycode::A) => thruster.camera.position.x -= 0.1,
					Some(Keycode::D) => thruster.camera.position.x += 0.1,
					Some(Keycode::Equals) => thruster.camera.fov += 5.0,
					Some(Keycode::Minus) => thruster.camera.fov -= 5.0,
					Some(Keycode::Space) => {
						thruster.screenshot("screenshot.png", 7680.0, 4320.0)?
					}
					_ => {}
				},
				_ => {}
			}
		}

		thruster.render(&mut canvas)?;

		canvas.present();
		::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
	}
	Ok(())
}
