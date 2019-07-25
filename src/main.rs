/* ************************************************************************** */
/*                                                                            */
/*                                                        ::::::::            */
/*   main.rs                                            :+:    :+:            */
/*                                                     +:+                    */
/*   By: nmartins <nmartins@student.codam.nl>         +#+                     */
/*                                                   +#+                      */
/*   Created: 2019/07/19 18:06:37 by nmartins       #+#    #+#                */
/*   Updated: 2019/07/25 17:06:27 by nmartins      ########   odam.nl         */
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
mod texture_map;

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

	let checker_mattex = MatTex::from_handle(checker_handle, Vec2::new(100.0, 100.0));
	let plane_mat = Material::reflective(checker_mattex);

	let red = MatTex::Color(Vec3::new(255.0, 0.0, 0.0));
	let red_mat = Material::diffuse(red);

    let obj = parser::parse("./teapot.obj".to_string());
    let mut scene: Vec<Box<dyn Intersectable>> = Vec::new();
    for (avt, bvt, cvt) in obj.triangles.iter() {
        scene.push(Box::new(Triangle {
            a: Vertex::from_parsed(avt),
            b: Vertex::from_parsed(bvt),
            c: Vertex::from_parsed(cvt),
            material: red_mat,
        }))
    }
    scene.extend::<Vec<Box<dyn Intersectable>>>(vec![
        Box::new(Plane {
            origin: Vec3::new(0.0, -1.0, 0.0),
            normal: Vec3::new(0.0, 1.0, 0.0),
            material: plane_mat,
        }),
        Box::new(Sphere {
            origin: Vec3::new(50.0, 80.0, 50.0),
            radius: 50.0,
            material: plane_mat,
        }),
    ]);

    let thruster = thruster::Thruster {
        camera: PerspectiveCamera::new(Vec3::new(0.0, 10.0, -100.0), SCREEN_WIDTH / SCREEN_HEIGHT),
        shapes: scene,
        lights: vec![Box::new(PointLight {
            origin: Vec3::new(1.0, 100.0, -30.0),
            color: Vec3::new(255.0, 255.0, 255.0),
        })],
		texture_map,
    };

    thruster
        .screenshot("screenshot.png", 3840.0, 2160.0)
        .map_err(|_| "Failed to take screenshot")?;

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
