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

#![allow(unused_variables)]

extern crate image;
extern crate rand;
extern crate scoped_threadpool;

extern crate glutin;
#[macro_use]
extern crate glium;

#[macro_use]
extern crate imgui;
extern crate imgui_glium_renderer;

extern crate oidn;

#[macro_use]
extern crate serde_derive;
extern crate ron;
extern crate serde;

mod algebra;
mod app;
mod bvh;
mod camera;
mod denoise;
mod dither;
mod key_state;
mod lightsource;
mod material;
mod parser;
mod profiler;
mod render_config;
mod scene;
mod shape;
mod skybox;
mod support;
mod texture_map;

pub fn main() -> std::result::Result<(), String> {
    let texture_map = texture_map::TextureMap::new();

    let scn_str =
        std::fs::read_to_string("cfg.ron").expect("Could not read configuration file 'cfg.ron'");
    let scene = ron::de::from_str(&scn_str).expect("Could not parse 'cfg.ron'");

    let mut app = app::App::new(scene, texture_map);

    app.run()?;
    Ok(())
}
