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

use crate::algebra::{Vec2, Vec3, Vertex};
use crate::material::{MatTex, Material, Reflectivity, Transparency};
use crate::scene::Scene;
use crate::shape::{Shape, Triangle};

pub fn main() -> std::result::Result<(), String> {
    let texture_map = texture_map::TextureMap::new();

    let scn_str =
        std::fs::read_to_string("cfg.ron").expect("Could not read configuration file 'cfg.ron'");
    let mut scene: Scene = ron::de::from_str(&scn_str).expect("Could not parse 'cfg.ron'");

    let obj =
        parser::parse("./objs/codam-text-high.obj".to_string()).expect("Could not parse .obj");
    for (a, b, c) in obj.tris.iter() {
        let a = Vertex {
            origin: a.origin.rotate_around(0, std::f64::consts::FRAC_PI_2)
                - Vec3::new(0.0, 0.0, 1.0),
            normal: a.normal.rotate_around(0, std::f64::consts::FRAC_PI_2),
            uv: a.uv,
        };
        let b = Vertex {
            origin: b.origin.rotate_around(0, std::f64::consts::FRAC_PI_2)
                - Vec3::new(0.0, 0.0, 1.0),
            normal: b.normal.rotate_around(0, std::f64::consts::FRAC_PI_2),
            uv: b.uv,
        };
        let c = Vertex {
            origin: c.origin.rotate_around(0, std::f64::consts::FRAC_PI_2)
                - Vec3::new(0.0, 0.0, 1.0),
            normal: c.normal.rotate_around(0, std::f64::consts::FRAC_PI_2),
            uv: c.uv,
        };

        scene.shapes.push(Shape::Triangle(Triangle {
            a,
            b,
            c,
            material: Material {
                texture: MatTex::Color(Vec3::new(255.0, 255.0, 255.0)),
                c_ambient: 0.3,
                c_diffuse: 0.7,
                reflectivity: Reflectivity {
                    amount: 0.0,
                    blurriness: 0.0,
                },
                transparency: Transparency {
                    amount: 0.0,
                    blurriness: 0.0,
                    index_of_refraction: 1.0,
                },
            },
        }))
    }
    let obj =
        parser::parse("./objs/codam-stripes-high.obj".to_string()).expect("Could not parse .obj");
    for (a, b, c) in obj.tris.iter() {
        let a = Vertex {
            origin: a.origin.rotate_around(0, std::f64::consts::FRAC_PI_2),
            normal: a.normal.rotate_around(0, std::f64::consts::FRAC_PI_2),
            uv: a.uv,
        };
        let b = Vertex {
            origin: b.origin.rotate_around(0, std::f64::consts::FRAC_PI_2),
            normal: b.normal.rotate_around(0, std::f64::consts::FRAC_PI_2),
            uv: b.uv,
        };
        let c = Vertex {
            origin: c.origin.rotate_around(0, std::f64::consts::FRAC_PI_2),
            normal: c.normal.rotate_around(0, std::f64::consts::FRAC_PI_2),
            uv: c.uv,
        };

        scene.shapes.push(Shape::Triangle(Triangle {
            a,
            b,
            c,
            material: Material {
                texture: MatTex::Texture {
                    handle: "./textures/codam.png".to_owned(),
                    scaling: Vec2::new(1.0, 1.0),
                },
                c_ambient: 0.3,
                c_diffuse: 0.7,
                reflectivity: Reflectivity {
                    amount: 0.0,
                    blurriness: 0.0,
                },
                transparency: Transparency {
                    amount: 0.0,
                    blurriness: 0.0,
                    index_of_refraction: 1.0,
                },
            },
        }))
    }

    let mut app = app::App::new(scene, texture_map);

    app.run()?;
    Ok(())
}
