#![allow(unused_variables)]

use image::ImageBuffer;
use log::info;
use std::sync::{Arc, Mutex};
use thruster::algebra::prelude::*;
use thruster::core::{
    aggregate::Aggregate,
    camera::{Camera, PerspectiveCamera},
    material::{Material, Matte},
    medium::{HomogeneousMedium, Medium, MediumInterface},
    primitive::{GeometricPrimitive, Primitive},
    renderer::{BasicRenderer, RenderOutput, Renderer},
    scene::Scene,
    spectrum::RGBSpectrum,
    texture::{ConstantTexture, Texture},
};
use thruster::geometry::{shape::Shape, sphere::Sphere};
use thruster::light::area_light::AreaLight;
use thruster::logger;
use thruster::sampler::{RandomSampler, Sampler};

pub fn main() -> std::result::Result<(), String> {
    logger::init().expect("Could not initialize logger");

    info!("Hewo user~! Wewcome to tuwuster!");

    let ex_medium = HomogeneousMedium {
        sigma_a: RGBSpectrum::from_rgb(255.0, 255.0, 255.0),
        sigma_s: RGBSpectrum::from_rgb(255.0, 255.0, 255.0),
        sigma_t: RGBSpectrum::from_rgb(255.0, 255.0, 255.0),
        g: 1.0,
    };

    let prims: Vec<Arc<dyn Primitive>> = vec![Arc::new(GeometricPrimitive {
        area_light: Arc::new(AreaLight),
        material: Arc::new(Matte {
            kd: Arc::new(ConstantTexture::new(RGBSpectrum::from_rgb(
                255.0, 255.0, 255.0,
            ))),
        }),
        shape: Arc::new(Sphere::new(Point3::new(0.0, 0.0, 15.0), 5.0)),
        medium_interface: MediumInterface {
            inside: Box::new(ex_medium.clone()),
            outside: Box::new(ex_medium.clone()),
        },
    })];

    let aggregate = Aggregate::from_primitives(prims);

    let scene = Scene::new(Arc::new(aggregate), Vec::new());

    let mut sampler = RandomSampler::new(1);

    let screen_dimensions = Vec2::new(640.0, 480.0);

    let mut camera = PerspectiveCamera::new(
        Transform::rotate_x(50.0)
            * Transform::rotate_y(20.0)
            * Transform::translation(&Vec3::new(0.0, 0.0, -10.0)),
        90.0,
        screen_dimensions,
    );

    let mut renderer = BasicRenderer::new(Arc::new(Mutex::new(sampler)), Arc::new(camera));
    let mut render_out = RenderOutput {
        buf: ImageBuffer::new(screen_dimensions.x as u32, screen_dimensions.y as u32),
    };

    renderer.render_scene(&scene, (), &mut render_out);

    render_out.save("output.png");

    Ok(())
}
