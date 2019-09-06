#![allow(unused_variables)]

use thruster::algebra::{Vec2, Vec3, Vertex};
use thruster::app;
use thruster::material::{MatTex, Material, Reflectivity, Transparency};
use thruster::parser;
use thruster::scene::Scene;
use thruster::shape::{Shape, Triangle};
use thruster::texture_map;

pub fn main() -> std::result::Result<(), String> {
    let mut texture_map = texture_map::TextureMap::new();

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
                c_diffuse: 0.3,
                reflectivity: Reflectivity {
                    amount: 0.4,
                    blurriness: 1.0,
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
                c_diffuse: 0.3,
                reflectivity: Reflectivity {
                    amount: 0.4,
                    blurriness: 1.0,
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

    use thruster::acceleration::bvh::*;
    //let accel = BVHAccel::new(BVHConstructionAlgorithm::Normal, scene.shapes.clone());
    //let (total, node) = accel.construct().expect("Could not construct BVHTree");
    //let flat_bvh = accel.flatten(Box::new(node), total);
    //println!("Primitives:     {}", flat_bvh.primitives.len());
    //println!("Expected Nodes: {}", flat_bvh.primitives.len() * 2 - 1);
    //println!("Actual Nodes:   {}", flat_bvh.linear_nodes.len());

    //texture_map.preload_all_in_scene(&scene);
    //scene.screenshot("screenshot.png", 640.0, 480.0, &texture_map);
    Ok(())
}
