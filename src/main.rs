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

mod algebra;
mod app;
mod camera;
mod dither;
mod key_state;
mod lightsource;
mod make_world;
mod material;
mod parser;
mod profiler;
mod scene;
mod shape;
mod skybox;
mod support;
mod texture_map;

pub fn main() -> std::result::Result<(), String> {
    let thruster = make_world::make_world()?;

    //let mut app = app::App::new(thruster);
    //
    thruster.screenshot("screenshot.png", 2.0 * 4096.0, 2.0 * 2160.0);

    //app.run()?;
    Ok(())
}
