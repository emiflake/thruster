use crate::algebra::Vec3;
use crate::scene::RenderData;
use crate::shape::{Intersection, Ray};
use serde_derive::{Deserialize, Serialize};

use rand::prelude::*;

pub trait Light {
    fn luminosity_at(&self, scene: &RenderData, intersection: &Intersection) -> f64;
    fn color(&self) -> Vec3;
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Lightsource {
    Point(PointLight),
}

impl Light for Lightsource {
    fn luminosity_at(&self, scene: &RenderData, intersection: &Intersection) -> f64 {
        match self {
            Self::Point(l) => l.luminosity_at(scene, intersection),
        }
    }

    fn color(&self) -> Vec3 {
        match self {
            Self::Point(l) => l.color(),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct PointLight {
    pub origin: Vec3,
    pub color: Vec3,
}

impl Light for PointLight {
    fn luminosity_at(&self, scene: &RenderData, intersection: &Intersection) -> f64 {
        if scene.config.distributed_tracing {
            let mut rng = rand::thread_rng();
            let mut amt = 0.0;
            let spp = scene.config.shadow_spp;
            let blurriness = 0.15;

            'sample_loop: for _ in 0..spp {
                let light_ray = (self.origin - intersection.origin)
                    .normalized()
                    .rotate(Vec3::new(
                        (rng.gen::<f64>() - 0.5) * blurriness,
                        (rng.gen::<f64>() - 0.5) * blurriness,
                        (rng.gen::<f64>() - 0.5) * blurriness,
                    ));
                if scene.config.shadows {
                    let ray = Ray::new(
                        intersection.origin + intersection.normal * 0.001,
                        light_ray,
                        0,
                    );
                    let intersections = ray.cast(&scene);
                    for inter in intersections.iter() {
                        if intersection.origin.distance(&self.origin) > inter.0.t {
                            continue 'sample_loop;
                        }
                    }
                }

                let dot = intersection.normal.dot(&light_ray);
                if dot >= 0.0 {
                    amt += dot / f64::from(spp);
                }
            }

            amt
        } else {
            let light_ray = (self.origin - intersection.origin).normalized();
            if scene.config.shadows {
                let ray = Ray::new(
                    intersection.origin + intersection.normal * 0.001,
                    light_ray,
                    0,
                );
                let intersections = ray.cast(&scene);
                for inter in intersections.iter() {
                    if intersection.origin.distance(&self.origin) > inter.0.t {
                        return 0.0;
                    }
                }
            }

            let dot = intersection.normal.dot(&light_ray);
            if dot < 0.0 {
                0.0
            } else {
                dot
            }
        }
    }

    fn color(&self) -> Vec3 {
        self.color
    }
}
