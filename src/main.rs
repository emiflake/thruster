/* ************************************************************************** */
/*                                                                            */
/*                                                        ::::::::            */
/*   main.rs                                            :+:    :+:            */
/*                                                     +:+                    */
/*   By: nmartins <nmartins@student.codam.nl>         +#+                     */
/*                                                   +#+                      */
/*   Created: 2019/07/19 18:06:37 by nmartins       #+#    #+#                */
/*   Updated: 2019/07/27 17:36:40 by nmartins      ########   odam.nl         */
/*                                                                            */
/* ************************************************************************** */

extern crate image;
extern crate rand;
extern crate scoped_threadpool;
extern crate sdl2;

mod camera;
mod dither;
mod lightsource;
mod material;
mod parser;
mod shape;
mod skybox;
mod texture_map;
mod thruster;

pub const SCREEN_WIDTH: f64 = 800.0;
pub const SCREEN_HEIGHT: f64 = 450.0;

use camera::PerspectiveCamera;
use lightsource::PointLight;
use material::{MatTex, Material};
use shape::{Intersectable, Plane, Sphere, Triangle, Vec2, Vec3, Vertex};
use skybox::Skybox;

#[allow(unused_imports)]
use sdl2::event::Event;
#[allow(unused_imports)]
use sdl2::keyboard::Keycode;

#[allow(unused_imports)]
use std::time::Duration;

pub fn main() -> std::result::Result<(), String> {
	let sdl_context = sdl2::init().unwrap();
	let video_subsystem = sdl_context.video().unwrap();

	#[allow(dead_code)]
	let window = video_subsystem
		.window(
			"Thruster Raytracer",
			SCREEN_WIDTH as u32,
			SCREEN_HEIGHT as u32,
		)
		.position_centered()
		.build()
		.unwrap();

	sdl_context.mouse().set_relative_mouse_mode(true);

	let mut texture_map = texture_map::TextureMap::new();

	let checker_handle = texture_map.load_image_from_file("./textures/checker.png")?;
	let earth_handle = texture_map.load_image_from_file("./textures/earth.png")?;

	let skybox = Skybox::new([
		texture_map.load_image_from_file("./skybox/miramar/miramar_bk.png")?,
		texture_map.load_image_from_file("./skybox/miramar/miramar_ft.png")?,
		texture_map.load_image_from_file("./skybox/miramar/miramar_up.png")?,
		texture_map.load_image_from_file("./skybox/miramar/miramar_dn.png")?,
		texture_map.load_image_from_file("./skybox/miramar/miramar_rt.png")?,
		texture_map.load_image_from_file("./skybox/miramar/miramar_lf.png")?,
	]);
	// let skybox = Skybox::new([
	// 	texture_map.load_image_from_file("./skybox/Yokohama3/negx.png")?,
	// 	texture_map.load_image_from_file("./skybox/Yokohama3/posx.png")?,
	// 	texture_map.load_image_from_file("./skybox/Yokohama3/posy.png")?,
	// 	texture_map.load_image_from_file("./skybox/Yokohama3/negy.png")?,
	// 	texture_map.load_image_from_file("./skybox/Yokohama3/posz.png")?,
	// 	texture_map.load_image_from_file("./skybox/Yokohama3/negz.png")?,
	// ]);

	let checker_mattex = MatTex::from_handle(checker_handle, Vec2::new(1000.0, 1000.0));
	let earth_mattex = MatTex::from_handle(earth_handle, Vec2::new(1.0, 1.0));
	let plane_mat = Material::reflective(checker_mattex);
	let earth_mat = Material::reflective(earth_mattex);

	let red = MatTex::Color(Vec3::new(255.0, 0.0, 0.0));
	let red_mat = Material {
		c_diffuse: 0.7,
		c_reflection: 0.3,
		c_ambient: 0.0,
		texture: red,
	};
	let black = MatTex::Color(Vec3::new(255.0, 255.0, 255.0));
	let black_mat = Material {
		c_diffuse: 0.7,
		c_reflection: 0.3,
		c_ambient: 0.0,
		texture: black,
	};
	let green = MatTex::Color(Vec3::new(0.0, 255.0, 0.0));
	let green_mat = Material {
		c_diffuse: 0.7,
		c_reflection: 0.3,
		c_ambient: 0.0,
		texture: green,
	};

	let obj = parser::parse("./objs/teapot.obj".to_string());
	let mut scene: Vec<Box<dyn Intersectable + Sync>> = Vec::new();
	// for (avt, bvt, cvt) in obj.triangles.iter() {
	// 	scene.push(Box::new(Triangle {
	// 		a: Vertex::from_parsed(avt),
	// 		b: Vertex::from_parsed(bvt),
	// 		c: Vertex::from_parsed(cvt),
	// 		material: red_mat,
	// 	}))
	// }
	scene.extend::<Vec<Box<dyn Intersectable + Sync>>>(vec![
		Box::new(Plane {
			origin: Vec3::new(0.0, -1.0, 0.0),
			normal: Vec3::new(0.0, 1.0, 0.0),
			material: plane_mat,
		}),
		Box::new(Sphere {
			origin: Vec3::new(-75.0, 100.0, 50.0),
			radius: 50.0,
			material: red_mat,
		}),
		Box::new(Sphere {
			origin: Vec3::new(0.0, 100.0, 25.0),
			radius: 25.0,
			material: green_mat,
		}),
		Box::new(Sphere {
			origin: Vec3::new(75.0, 80.0, 50.0),
			radius: 50.0,
			material: earth_mat,
		}),
	]);

	#[allow(unused_mut)]
	let mut thruster = thruster::Thruster {
		camera: PerspectiveCamera::new(Vec3::new(0.0, 50.0, -200.0), SCREEN_WIDTH / SCREEN_HEIGHT),
		shapes: scene,
		lights: vec![Box::new(PointLight {
			origin: Vec3::new(1.0, 250.0, -30.0),
			color: Vec3::new(255.0, 255.0, 255.0),
		})],
		texture_map,
		skybox,
	};

	thruster
		// .screenshot("screenshot.png", 15360.0, 8640.0)
		// .screenshot("screenshot.png", 3840.0, 2160.0)
		// .screenshot("screenshot.png", 1920.0, 1080.0)
		.screenshot("screenshot.png", 800.0, 450.0)
		// .screenshot("screenshot.png", 320.0, 200.0)
		.map_err(|_| "Failed to take screenshot")?;

	let mut canvas = window.into_canvas().build().unwrap();

	canvas.clear();
	canvas.present();
	let mut event_pump = sdl_context.event_pump().unwrap();
	let mut before = std::time::SystemTime::now();
	let mut last_mouse_pos = (0, 0);
	let mut i = 0;

	'running: loop {
		let delta_time: u128 = before.elapsed().unwrap().as_millis();
		before = std::time::SystemTime::now();
		for event in event_pump.poll_iter() {
			match event {
				Event::Quit { .. }
				| Event::KeyDown {
					keycode: Some(Keycode::Escape),
					..
				} => break 'running,
				Event::KeyDown { keycode, .. } => match keycode {
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

		let keys: std::collections::HashSet<Keycode> = event_pump
			.keyboard_state()
			.pressed_scancodes()
			.filter_map(Keycode::from_scancode)
			.collect();

		let mouse_state = event_pump.mouse_state();
		let mouse_pos = (mouse_state.x(), mouse_state.y());
		let delta_mouse = (
			last_mouse_pos.0 - mouse_pos.0,
			last_mouse_pos.1 - mouse_pos.1,
		);
		last_mouse_pos = mouse_pos;

		thruster.camera.rotate(Vec3::new(
			0.0,
			-f64::from(delta_mouse.0) / 100.0,
			f64::from(delta_mouse.1) / 100.0,
		));

		let speed = if keys.contains(&Keycode::LShift) {
			10f64 * (delta_time as u64 as f64) / 100f64
		} else {
			1f64 * (delta_time as u64 as f64) / 100f64
		};

		thruster.camera.rotate(Vec3::new(
			0.0,
			f64::from(keys.contains(&Keycode::L) as i32) / 200.0,
			0.0,
		));

		thruster.camera.translate(Vec3::new(
			f64::from(keys.contains(&Keycode::A) as i32) * -speed
				+ f64::from(keys.contains(&Keycode::D) as i32) * speed,
			f64::from(keys.contains(&Keycode::Q) as i32) * -speed
				+ f64::from(keys.contains(&Keycode::E) as i32) * speed,
			f64::from(keys.contains(&Keycode::W) as i32) * -speed
				+ f64::from(keys.contains(&Keycode::S) as i32) * speed,
		));

		thruster.render(&mut canvas, Some(format!("screen/{}.png", i)))?;

		i += 1u32;

		canvas.present();
		::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
	}
	Ok(())
}
