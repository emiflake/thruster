/* ************************************************************************** */
/*                                                                            */
/*                                                        ::::::::            */
/*   main.rs                                            :+:    :+:            */
/*                                                     +:+                    */
/*   By: nmartins <nmartins@student.codam.nl>         +#+                     */
/*                                                   +#+                      */
/*   Created: 2019/07/19 18:06:37 by nmartins       #+#    #+#                */
/*   Updated: 2019/08/10 15:41:04 by nmartins      ########   odam.nl         */
/*                                                                            */
/* ************************************************************************** */

extern crate image;
extern crate rand;
extern crate scoped_threadpool;

extern crate glutin;
#[macro_use]
extern crate glium;

#[macro_use]
extern crate imgui;
extern crate imgui_glium_renderer;

mod camera;
mod dither;
mod key_state;
mod lightsource;
mod make_world;
mod material;
mod parser;
mod shape;
mod skybox;
mod support;
mod texture_map;
mod thruster;

use glium::Surface;
use glutin::*;

use imgui::Context;
use imgui::*;
use imgui_winit_support::{HiDpiMode, WinitPlatform};

#[allow(unused_imports)]
use sdl2::event::Event;
#[allow(unused_imports)]
use sdl2::keyboard::Keycode;
use shape::Vec3;

use std::collections::VecDeque;
use std::thread;
#[allow(unused_imports)]
use std::time::{Duration, Instant};

pub fn main() -> std::result::Result<(), String> {
	let mut event_loop = glutin::EventsLoop::new();
	let wb =
		glutin::WindowBuilder::new().with_dimensions(glutin::dpi::LogicalSize::new(1280.0, 720.0));
	let cb = glutin::ContextBuilder::new().with_vsync(true);
	let display = glium::Display::new(wb, cb, &event_loop).unwrap();
	let mut imgui = Context::create();
	imgui.set_ini_filename(None);
	let mut platform = WinitPlatform::init(&mut imgui);
	{
		let gl_window = display.gl_window();
		let window = gl_window.window();
		platform.attach_window(imgui.io_mut(), &window, HiDpiMode::Rounded);
	}

	// Raw image rendering output
	let program = support::get_program(&display);
	let (vertex_buffer, index_buffer) = support::get_buffers(&display);

	// * World setup * //
	let mut thruster = make_world::make_world()?;
	let mut keystate = key_state::Keystate::default();

	// * Variables during looping * //
	let mut renderer = imgui_glium_renderer::Renderer::init(&mut imgui, &display).unwrap();
	let mut last_frame = Instant::now();
	let mut cursor_pos = (0.0, 0.0);
	let mut closed = false;
	let mut dimensions: [i32; 2] = [3840, 2160];
	let mut delays = VecDeque::new();
	while !closed {
		let gl_window = display.gl_window();
		let window = gl_window.window();

		let now = Instant::now();
		let delta = now - last_frame;
		let delta_time = delta.as_secs() as f32 + delta.subsec_nanos() as f32 / 1_000_000_000.0;

		// * Delay counting, for profiler * //
		delays.push_back(delta_time);
		if delays.len() > 100 {
			delays.pop_front();
		}
		// Event handling
		event_loop.poll_events(|event| {
			platform.handle_event(imgui.io_mut(), &window, &event);
			keystate.handle_event(&event);

			match event {
				glutin::Event::WindowEvent { event, .. } => match event {
					glutin::WindowEvent::CloseRequested => closed = true,
					_ => (),
				},
				_ => (),
			}
		});

		// IMGUI PREPARE
		let io = imgui.io_mut();
		last_frame = io.update_delta_time(last_frame);
		let mut ui = imgui.frame();
		imgui::Window::new(&ui, im_str!("Profiler"))
			.size([400.0, 125.0], Condition::FirstUseEver)
			.position([50.0, 200.0], Condition::FirstUseEver)
			.build(|| {
				ui.text(format!("FPS: {:.2}/{:.5}ms", 1.0 / delta_time, delta_time));

				let lines: Vec<f32> = delays.iter().map(|x| *x).collect();
				ui.plot_lines(im_str!("Delay (ms)"), lines.as_ref())
					.graph_size([300.0, 75.0])
					.build();
			});

		imgui::Window::new(&ui, im_str!("Render Controls"))
			.size([300.0, 150.0], Condition::FirstUseEver)
			.build(|| {
				ui.input_int(im_str!("Width"), &mut dimensions[0]).build();
				ui.input_int(im_str!("Height"), &mut dimensions[1]).build();
				if dimensions[0] <= 0 {
					dimensions[0] = 640;
				}
				if dimensions[1] <= 0 {
					dimensions[1] = 480;
				}
				if imgui::Ui::button(&ui, im_str!("Take Screenshot"), [175.0, 50.0]) {
					thruster
						.screenshot(
							"screenshot.png",
							f64::from(dimensions[0]),
							f64::from(dimensions[1]),
						)
						.expect("Could not take screenshot");

					std::process::Command::new("open")
						.arg("screenshot.png")
						.output()
						.expect("failed to execute process");
				}
			});

		if keystate.is_key_down(glutin::VirtualKeyCode::Escape) {
			closed = true;
		}

		handle_keys(&keystate, &mut thruster);

		let image = thruster.render_to_buffer(640.0, 360.0);
		let image_dimensions = image.dimensions();
		let raw_pixels = image.into_raw();

		let mut target = display.draw();
		target.clear_color_srgb(0.0, 0.0, 0.0, 1.0);

		let image =
			glium::texture::RawImage2d::from_raw_rgba_reversed(&raw_pixels, image_dimensions);
		let opengl_texture = glium::texture::CompressedSrgbTexture2d::new(&display, image).unwrap();

		let uniforms = uniform! {
			matrix: [
				[1.0, 0.0, 0.0, 0.0],
				[0.0, 1.0, 0.0, 0.0],
				[0.0, 0.0, 1.0, 0.0],
				[0.0, 0.0, 0.0, 1.0f32]
			],
			tex: &opengl_texture
		};
		target
			.draw(
				&vertex_buffer,
				&index_buffer,
				&program,
				&uniforms,
				&Default::default(),
			)
			.unwrap();

		// IMGUI RENDER
		let draw_data = ui.render();
		renderer
			.render(&mut target, draw_data)
			.expect("Could not render imgui");

		target.finish().expect("Failed to swap buffers");
		thread::sleep(std::time::Duration::from_millis(16));
	}
	Ok(())
}

pub fn handle_keys(keystate: &key_state::Keystate, mut thruster: &mut thruster::Thruster) {
	let speed = if keystate.is_key_down(glutin::VirtualKeyCode::LShift) {
		25.0
	} else {
		1.0
	};

	if keystate.is_key_down(glutin::VirtualKeyCode::A) {
		thruster.camera.translate(Vec3::new(-speed, 0.0, 0.0));
	}
	if keystate.is_key_down(glutin::VirtualKeyCode::D) {
		thruster.camera.translate(Vec3::new(speed, 0.0, 0.0));
	}
	if keystate.is_key_down(glutin::VirtualKeyCode::S) {
		thruster.camera.translate(Vec3::new(0.0, 0.0, speed));
	}
	if keystate.is_key_down(glutin::VirtualKeyCode::W) {
		thruster.camera.translate(Vec3::new(0.0, 0.0, -speed));
	}
	if keystate.is_key_down(glutin::VirtualKeyCode::E) {
		thruster.camera.translate(Vec3::new(0.0, speed, 0.0));
	}
	if keystate.is_key_down(glutin::VirtualKeyCode::Q) {
		thruster.camera.translate(Vec3::new(0.0, -speed, 0.0));
	}

	if keystate.is_key_down(glutin::VirtualKeyCode::Left) {
		thruster.camera.rotate(Vec3::new(0.0, -0.05, 0.0));
	}
	if keystate.is_key_down(glutin::VirtualKeyCode::Right) {
		thruster.camera.rotate(Vec3::new(0.0, 0.05, 0.0));
	}
}
