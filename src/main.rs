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

extern crate oidn;

mod algebra;
mod app;
mod camera;
mod denoise;
mod dither;
mod key_state;
mod lightsource;
mod make_world;
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
    let scene = make_world::make_world()?;

    let config = render_config::RenderConfig {
        reflection_spp: 1,
        refraction_spp: 1,
        shadow_spp: 1,
        distributed_tracing: false,
        recursion_depth: 5,
        denoise: false,
        ..Default::default()
    };

    let scene = scene.with_config(config);

    let mut app = app::App::new(scene);

    //thruster.screenshot("screenshot.png", 4096.0, 2160.0);

    app.run()?;
    Ok(())
}
