#![allow(unused_variables)]

use log::info;
use std::sync::Arc;
use thruster::algebra::prelude::*;
use thruster::core::{
    aggregate::Aggregate,
    material::{Material, Matte},
    medium::{HomogeneousMedium, Medium, MediumInterface},
    primitive::{GeometricPrimitive, Primitive},
    scene::Scene,
    spectrum::RGBSpectrum,
    texture::{ConstantTexture, Texture},
};
use thruster::geometry::{shape::Shape, sphere::Sphere};
use thruster::light::area_light::AreaLight;
use thruster::logger;

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
        shape: Arc::new(Sphere::new(Point3::new(0.0, 0.0, 10.0), 5.0)),
        medium_interface: MediumInterface {
            inside: Box::new(ex_medium.clone()),
            outside: Box::new(ex_medium.clone()),
        },
    })];

    let aggregate = Aggregate::from_primitives(prims);

    let scene = Scene::new(Arc::new(aggregate), Vec::new());

    println!("{:#?}", scene);

    println!(
        "{}",
        scene.does_intersect(&Ray::new(
            Point3::new(4.5, 0.0, 0.0),
            Vec3::new(0.01, 0.01, 1.0).normalized()
        ))
    );

    Ok(())
}
