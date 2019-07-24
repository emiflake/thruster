/* ************************************************************************** */
/*                                                                            */
/*                                                        ::::::::            */
/*   main.rs                                            :+:    :+:            */
/*                                                     +:+                    */
/*   By: nmartins <nmartins@student.codam.nl>         +#+                     */
/*                                                   +#+                      */
/*   Created: 2019/07/19 18:06:37 by nmartins       #+#    #+#                */
/*   Updated: 2019/07/25 00:03:48 by nmartins      ########   odam.nl         */
/*                                                                            */
/* ************************************************************************** */

extern crate image;
extern crate rand;
extern crate sdl2;

mod camera;
mod lightsource;
mod material;
mod parser;
mod shape;
mod thruster;

pub const SCREEN_WIDTH: f64 = 1280.0;
pub const SCREEN_HEIGHT: f64 = 720.0;

use camera::PerspectiveCamera;
use lightsource::PointLight;
use material::{MatTex, Material};
use shape::{Intersectable, Plane, Sphere, Triangle, Vec3, Vertex};

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

	let obj = parser::parse("./teapot.obj".to_string());
	println!("{:?}", obj);
	let mut scene: Vec<Box<dyn Intersectable>> = Vec::new();
	for (avt, bvt, cvt) in obj.triangles.iter() {
		scene.push(Box::new(Triangle {
			a: Vertex::from_parsed(avt),
			b: Vertex::from_parsed(bvt),
			c: Vertex::from_parsed(cvt),
			material: Material::diffuse(MatTex::Color(Vec3::new(255.0, 0.0, 0.0))),
		}))
	}
	scene.extend::<Vec<Box<dyn Intersectable>>>(vec![Box::new(Plane {
		origin: Vec3::new(0.0, -1.0, 0.0),
		normal: Vec3::new(0.0, 1.0, 0.0),
		material: Material::reflective(
			MatTex::load_from_file("checker.png", (100.0, 100.0))
				.map_err(|_| "Error loading checker image")?,
		),
	})]);

	let mut thruster = thruster::Thruster {
		camera: PerspectiveCamera::new(Vec3::new(0.0, 10.0, -100.0), SCREEN_WIDTH / SCREEN_HEIGHT),
		shapes: scene,
		lights: vec![Box::new(PointLight {
			origin: Vec3::new(1.0, 100.0, -30.0),
			color: Vec3::new(255.0, 255.0, 255.0),
		})],
	};

	thruster.screenshot("screenshot.png", 3840.0, 2160.0);
	// thruster.screenshot("screenshot.png", 640.0, 480.0);

	// let mut canvas = window.into_canvas().build().unwrap();

	// canvas.clear();
	// canvas.present();
	// let mut event_pump = sdl_context.event_pump().unwrap();

	// 'running: loop {
	// 	for event in event_pump.poll_iter() {
	// 		match event {
	// 			Event::Quit { .. }
	// 			| Event::KeyDown {
	// 				keycode: Some(Keycode::Escape),
	// 				..
	// 			} => break 'running,
	// 			Event::KeyDown { keycode, .. } => match keycode {
	// 				Some(Keycode::E) => thruster.camera.position.y -= 1.0,
	// 				Some(Keycode::Q) => thruster.camera.position.y += 1.0,
	// 				Some(Keycode::S) => thruster.camera.position.z -= 1.0,
	// 				Some(Keycode::W) => thruster.camera.position.z += 1.0,
	// 				Some(Keycode::A) => thruster.camera.position.x -= 1.0,
	// 				Some(Keycode::D) => thruster.camera.position.x += 1.0,
	// 				Some(Keycode::Equals) => thruster.camera.fov += 5.0,
	// 				Some(Keycode::Minus) => thruster.camera.fov -= 5.0,
	// 				Some(Keycode::Space) => {
	// 					thruster.screenshot("screenshot.png", 7680.0, 4320.0)?
	// 				}
	// 				_ => {}
	// 			},
	// 			_ => {}
	// 		}
	// 	}

	// 	thruster.render(&mut canvas)?;

	// 	canvas.present();
	// 	::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
	// }
	Ok(())
}
