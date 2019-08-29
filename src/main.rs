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
mod scene;
mod app;
mod profiler;
mod algebra;

pub fn main() -> std::result::Result<(), String> {
	let thruster = make_world::make_world()?;

    let mut app = app::App::new(thruster);

    app.run()?;
	Ok(())
}
