/* ************************************************************************** */
/*                                                                            */
/*                                                        ::::::::            */
/*   main.rs                                            :+:    :+:            */
/*                                                     +:+                    */
/*   By: nmartins <nmartins@student.codam.nl>         +#+                     */
/*                                                   +#+                      */
/*   Created: 2019/07/19 18:06:37 by nmartins       #+#    #+#                */
/*   Updated: 2019/07/26 00:12:10 by nmartins      ########   odam.nl         */
/*                                                                            */
/* ************************************************************************** */

extern crate image;
extern crate rand;
extern crate scoped_threadpool;
extern crate sdl2;

mod camera;
mod lightsource;
mod material;
mod parser;
mod shape;
mod texture_map;
mod thruster;

pub const SCREEN_WIDTH: f64 = 1280.0;
pub const SCREEN_HEIGHT: f64 = 720.0;

use camera::PerspectiveCamera;
use lightsource::PointLight;
use material::{MatTex, Material};
use shape::{Intersectable, Plane, Sphere, Triangle, Vec2, Vec3, Vertex};

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

	let mut texture_map = texture_map::TextureMap::new();

	let checker_handle = texture_map.load_image_from_file("checker.png")?;
	let earth_handle = texture_map.load_image_from_file("earth.png")?;
	let bottle_handle = texture_map.load_image_from_file("bottle.png")?;
	let ammo_handle = texture_map.load_image_from_file("ammobox.png")?;

	let checker_mattex = MatTex::from_handle(checker_handle, Vec2::new(1000.0, 1000.0));
	let earth_mattex = MatTex::from_handle(earth_handle, Vec2::new(1.0, 1.0));
	let bottle_mattex = MatTex::from_handle(bottle_handle, Vec2::new(1.0, -1.0));
	let ammo_mattex = MatTex::from_handle(ammo_handle, Vec2::new(1.0, -1.0));
	let plane_mat = Material::reflective(checker_mattex);
	let earth_mat = Material::reflective(earth_mattex);
	let bottle_mat = Material::diffuse(bottle_mattex);
	let ammo_mat = Material::diffuse(ammo_mattex);

	let red = MatTex::Color(Vec3::new(255.0, 0.0, 0.0));
	let red_mat = Material {
		c_diffuse: 0.7,
		c_reflection: 0.3,
		c_ambient: 0.0,
		texture: red,
	};

	let obj = parser::parse("./ammo.obj".to_string());
	let mut scene: Vec<Box<dyn Intersectable + Sync>> = Vec::new();
	for (avt, bvt, cvt) in obj.triangles.iter() {
		scene.push(Box::new(Triangle {
			a: Vertex::from_parsed(avt),
			b: Vertex::from_parsed(bvt),
			c: Vertex::from_parsed(cvt),
			material: ammo_mat,
		}))
	}
	scene.extend::<Vec<Box<dyn Intersectable + Sync>>>(vec![
		Box::new(Plane {
			origin: Vec3::new(0.0, -1.0, 0.0),
			normal: Vec3::new(0.0, 1.0, 0.0),
			material: plane_mat,
		}),
		Box::new(Sphere {
			origin: Vec3::new(50.0, 80.0, 50.0),
			radius: 50.0,
			material: earth_mat,
		}),
	]);

	let mut thruster = thruster::Thruster {
		camera: PerspectiveCamera::new(Vec3::new(0.0, 50.0, -200.0), SCREEN_WIDTH / SCREEN_HEIGHT),
		shapes: scene,
		lights: vec![Box::new(PointLight {
			origin: Vec3::new(1.0, 250.0, -30.0),
			color: Vec3::new(255.0, 255.0, 255.0),
		})],
		texture_map,
	};

	thruster
		.screenshot("screenshot.png", 3840.0, 2160.0)
		// 	// .screenshot("screenshot.png", 800.0, 450.0)
		// .screenshot("screenshot.png", 1920.0, 1080.0)
		.map_err(|_| "Failed to take screenshot")?;

	// let mut canvas = window.into_canvas().build().unwrap();

	// canvas.clear();
	// canvas.present();
	// let mut event_pump = sdl_context.event_pump().unwrap();
	// let mut before = std::time::SystemTime::now();

	// 'running: loop {
	// 	let delta_time: u128 = before.elapsed().unwrap().as_millis();
	// 	before = std::time::SystemTime::now();
	// 	for event in event_pump.poll_iter() {
	// 		match event {
	// 			Event::Quit { .. }
	// 			| Event::KeyDown {
	// 				keycode: Some(Keycode::Escape),
	// 				..
	// 			} => break 'running,
	// 			Event::KeyDown { keycode, .. } => match keycode {
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

	// 	let keys: std::collections::HashSet<Keycode> = event_pump
	// 		.keyboard_state()
	// 		.pressed_scancodes()
	// 		.filter_map(Keycode::from_scancode)
	// 		.collect();

	// 	let speed = if keys.contains(&Keycode::LShift) {
	// 		10f64 * (delta_time as u64 as f64) / 100f64
	// 	} else {
	// 		1f64 * (delta_time as u64 as f64) / 100f64
	// 	};

	// 	thruster.camera.translate(Vec3::new(
	// 		f64::from(keys.contains(&Keycode::A) as i32) * -speed
	// 			+ f64::from(keys.contains(&Keycode::D) as i32) * speed,
	// 		f64::from(keys.contains(&Keycode::Q) as i32) * -speed
	// 			+ f64::from(keys.contains(&Keycode::E) as i32) * speed,
	// 		f64::from(keys.contains(&Keycode::W) as i32) * -speed
	// 			+ f64::from(keys.contains(&Keycode::S) as i32) * speed,
	// 	));

	// 	thruster.render(&mut canvas)?;

	// 	canvas.present();
	// 	::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
	// }
	Ok(())
}
