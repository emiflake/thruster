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
use crate::camera::PerspectiveCamera;
use crate::lightsource::PointLight;
use crate::material::{MatTex, Material, Reflectivity, Transparency};
use crate::render_config::RenderConfig;
use crate::scene::Scene;
use crate::shape::{Plane, Shape, Sphere, Triangle};
use crate::skybox::Skybox;

pub fn main() -> std::result::Result<(), String> {
    let mut texture_map = texture_map::TextureMap::new();

    let checker_handle = texture_map.load_image_from_file("./textures/checker.png")?;
    let wood_handle = texture_map.load_image_from_file("./textures/wood.png")?;
    let earth_handle = texture_map.load_image_from_file("./textures/earth.png")?;

    //let skybox = Skybox::new([
    //texture_map.load_image_from_file("./skybox/miramar/miramar_bk.png")?,
    //texture_map.load_image_from_file("./skybox/miramar/miramar_ft.png")?,
    //texture_map.load_image_from_file("./skybox/miramar/miramar_up.png")?,
    //texture_map.load_image_from_file("./skybox/miramar/miramar_dn.png")?,
    //texture_map.load_image_from_file("./skybox/miramar/miramar_rt.png")?,
    //texture_map.load_image_from_file("./skybox/miramar/miramar_lf.png")?,
    //]);
    let skybox = Skybox::new([
        texture_map.load_image_from_file("./skybox/Yokohama3/negx.png")?,
        texture_map.load_image_from_file("./skybox/Yokohama3/posx.png")?,
        texture_map.load_image_from_file("./skybox/Yokohama3/posy.png")?,
        texture_map.load_image_from_file("./skybox/Yokohama3/negy.png")?,
        texture_map.load_image_from_file("./skybox/Yokohama3/posz.png")?,
        texture_map.load_image_from_file("./skybox/Yokohama3/negz.png")?,
    ]);

    let checker_mattex = MatTex::from_handle(checker_handle, Vec2::new(1000.0, 1000.0));
    let wood_mattex = MatTex::from_handle(wood_handle, Vec2::new(1000.0, 1000.0));
    let earth_mattex = MatTex::from_handle(earth_handle, Vec2::new(1.0, 1.0));
    let plane_mat = Material {
        c_diffuse: 0.3,
        reflectivity: Reflectivity {
            amount: 0.7,
            blurriness: 0.5,
        },
        c_ambient: 0.0,
        transparency: Transparency::not_transparent(),
        texture: checker_mattex,
    };
    let wood_mat = Material::diffuse(wood_mattex);
    let earth_mat = Material::reflective(earth_mattex);

    let red = MatTex::Color(Vec3::new(175.0, 0.0, 0.0));
    let red_mat = Material {
        c_diffuse: 1.0,
        reflectivity: Reflectivity::not_reflective(),
        c_ambient: 0.0,
        transparency: Transparency::not_transparent(),
        texture: red,
    };
    let grey = MatTex::Color(Vec3::new(100.0, 100.0, 100.0));
    let grey_mat = Material {
        c_diffuse: 0.7,
        reflectivity: Reflectivity::not_reflective(),
        c_ambient: 0.3,
        transparency: Transparency::not_transparent(),
        texture: grey,
    };
    let trans_mat = Material {
        c_diffuse: 0.0,
        reflectivity: Reflectivity::not_reflective(),
        c_ambient: 0.0,
        transparency: Transparency {
            amount: 1.0,
            blurriness: 0.4,
            index_of_refraction: 1.4,
        },
        texture: red,
    };
    let refl_mat = Material {
        c_diffuse: 0.3,
        reflectivity: Reflectivity {
            amount: 0.7,
            blurriness: 0.14,
        },
        c_ambient: 0.0,
        transparency: Transparency::not_transparent(),
        texture: MatTex::Color(Vec3::new(255.0, 255.0, 255.0)),
    };
    let black = MatTex::Color(Vec3::new(255.0, 255.0, 255.0));
    let black_mat = Material {
        c_diffuse: 0.7,
        reflectivity: Reflectivity::not_reflective(),
        c_ambient: 0.0,
        transparency: Transparency::not_transparent(),
        texture: black,
    };
    let green = MatTex::Color(Vec3::new(0.0, 170.0, 0.0));
    let green_mat = Material {
        c_diffuse: 1.0,
        reflectivity: Reflectivity::not_reflective(),
        c_ambient: 0.0,
        transparency: Transparency::not_transparent(),
        texture: green,
    };

    let obj = parser::parse("./objs/teapot.obj".to_string());
    let mut scene: Vec<Shape> = Vec::new();
    for (avt, bvt, cvt) in obj.triangles.iter() {
        scene.push(Box::new(Triangle {
            a: Vertex::from_parsed(avt),
            b: Vertex::from_parsed(bvt),
            c: Vertex::from_parsed(cvt),
            material: grey_mat,
        }))
    }
    scene.extend::<Vec<Shape>>(vec![
        Box::new(Plane {
            origin: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 1.0, 0.0).normalized(),
            material: plane_mat,
        }),
        //Box::new(Plane {
        //origin: Vec3::new(-200.0, 0.0, 100.0),
        //normal: Vec3::new(1.0, 0.0, 0.0).normalized(),
        //material: red_mat,
        //}),
        //Box::new(Plane {
        //origin: Vec3::new(0.0, 0.0, 100.0),
        //normal: Vec3::new(0.0, 0.0, -1.0).normalized(),
        //material: grey_mat,
        //}),
        //Box::new(Plane {
        //origin: Vec3::new(0.0, 400.0, 100.0),
        //normal: Vec3::new(0.0, -1.0, 0.0).normalized(),
        //material: grey_mat,
        //}),
        //Box::new(Plane {
        //origin: Vec3::new(200.0, 0.0, 100.0),
        //normal: Vec3::new(-1.0, 0.0, 0.0).normalized(),
        //material: green_mat,
        //}),
        Box::new(Sphere {
            origin: Vec3::new(50.0, 125.0, 0.0),
            radius: 25.0,
            material: refl_mat,
        }),
    ]);

    use crate::bvh::BVHTree;
    let bvh = BVHTree::construct(&mut scene).expect("Could not construct BVHTree");

    #[allow(unused_mut)]
    let mut scene = Scene {
        camera: PerspectiveCamera::new(Vec3::new(0.0, 50.0, -200.0)),
        config: RenderConfig::default(),
        bvh,
        lights: vec![
            Box::new(PointLight {
                origin: Vec3::new(-50.0, 350.0, 50.0),
                color: Vec3::new(255.0, 255.0, 255.0) * 1.0,
            }),
            //Box::new(PointLight {
            //origin: Vec3::new(50.0, 100.0, 50.0),
            //color: Vec3::new(255.0, 255.0, 255.0) * 1.0,
            //}),
        ],
        texture_map,
        skybox,
    };

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

    app.run()?;
    Ok(())
}
