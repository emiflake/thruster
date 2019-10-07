use crate::algebra::prelude::*;
use crate::scene::RenderData;
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
        let mut rng = rand::thread_rng();
        let mut amt = 0.0;
        let (spp, blurriness) = if scene.config.distributed_tracing {
            (scene.config.shadow_spp, 0.15)
        } else {
            (1, 0.0)
        };

        'sample_loop: for _ in 0..spp {
            let light_ray = (self.origin - intersection.origin)
                .normalized()
                .rotate(Vec3::new(
                    (rng.gen::<f64>() - 0.5) * blurriness,
                    (rng.gen::<f64>() - 0.5) * blurriness,
                    (rng.gen::<f64>() - 0.5) * blurriness,
                ));
            if scene.config.shadows
                && Ray::new(
                    intersection.origin + intersection.normal * 0.001,
                    light_ray,
                    0,
                )
                .cast(&scene)
                .is_some()
            {
                continue 'sample_loop;
            }

            let dot = comb::dot(&intersection.normal, &light_ray);
            if dot >= 0.0 {
                amt += dot / f64::from(spp);
            }
        }

        amt
    }

    fn color(&self) -> Vec3 {
        self.color
    }
}
