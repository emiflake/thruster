/* ************************************************************************** */
/*                                                                            */
/*                                                        ::::::::            */
/*   lightsource.rs                                     :+:    :+:            */
/*                                                     +:+                    */
/*   By: nmartins <nmartins@student.codam.nl>         +#+                     */
/*                                                   +#+                      */
/*   Created: 2019/07/20 21:05:24 by nmartins       #+#    #+#                */
/*   Updated: 2019/07/25 21:43:30 by nmartins      ########   odam.nl         */
/*                                                                            */
/* ************************************************************************** */

use crate::algebra::Vec3;
use crate::scene::Scene;
use crate::shape::{Intersection, Ray};

use rand::prelude::*;

pub trait Lightsource {
    fn luminosity_at(&self, scene: &Scene, intersection: &Intersection) -> f64;
    fn color(&self) -> Vec3;
}

pub struct PointLight {
    pub origin: Vec3,
    pub color: Vec3,
}

impl Lightsource for PointLight {
    fn luminosity_at(&self, scene: &Scene, intersection: &Intersection) -> f64 {
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
                let ray = Ray {
                    origin: intersection.origin + intersection.normal * 0.001,
                    direction: light_ray,
                    level: 0,
                };
                let intersections = ray.cast(&scene);
                for inter in intersections.iter() {
                    if intersection.origin.distance(&self.origin) > inter.0.t {
                        continue 'sample_loop;
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
            let ray = Ray {
                origin: intersection.origin + intersection.normal * 0.001,
                direction: light_ray,
                level: 0,
            };
            let intersections = ray.cast(&scene);
            for inter in intersections.iter() {
                if intersection.origin.distance(&self.origin) > inter.0.t {
                    return 0.0;
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
