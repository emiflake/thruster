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

use crate::shape::{Intersection, Ray};
use crate::algebra::Vec3;
use crate::scene::Scene;

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

    fn color(&self) -> Vec3 {
        self.color
    }
}
