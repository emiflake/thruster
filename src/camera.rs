/* ************************************************************************** */
/*                                                                            */
/*                                                        ::::::::            */
/*   camera.rs                                          :+:    :+:            */
/*                                                     +:+                    */
/*   By: nmartins <nmartins@student.codam.nl>         +#+                     */
/*                                                   +#+                      */
/*   Created: 2019/07/19 18:23:53 by nmartins       #+#    #+#                */
/*   Updated: 2019/08/05 15:31:20 by nmartins      ########   odam.nl         */
/*                                                                            */
/* ************************************************************************** */

use crate::algebra::Vec3;
use crate::scene::Scene;
use crate::shape::Ray;
use rand::prelude::*;

pub trait Camera {
    fn project_rays(
        &self,
        screen_pos: (f64, f64),
        screen_dim: (f64, f64),
        scene: &Scene,
    ) -> Vec<Ray>;
}

pub struct PerspectiveCamera {
    pub position: Vec3,
    pub rotation: Vec3,
    pub fov: f64,
}

impl PerspectiveCamera {
    #[allow(dead_code)]
    pub fn new(position: Vec3) -> Self {
        PerspectiveCamera {
            position,
            rotation: Vec3::ORIGIN,
            fov: 60.0,
        }
    }

    #[allow(dead_code)]
    pub fn translate(&mut self, v3: Vec3) {
        self.position.x += self.rotation.y.cos() * v3.x - self.rotation.y.sin() * v3.z;
        self.position.y += v3.y;
        self.position.z += -self.rotation.y.cos() * v3.z - self.rotation.y.sin() * v3.x;
    }

    pub fn rotate(&mut self, v3: Vec3) {
        self.rotation = self.rotation + v3;
    }
}

impl Camera for PerspectiveCamera {
    fn project_rays(&self, (sx, sy): (f64, f64), (w, h): (f64, f64), scene: &Scene) -> Vec<Ray> {
        if scene.config.distributed_tracing {
            let mut rng = rand::thread_rng();
            let aspect_ratio = w / h;
            let px = (2.0 * ((sx + 0.5) / w) - 1.0)
                * aspect_ratio
                * (self.fov / 2.0 * std::f64::consts::PI / 180.0).tan();
            let py = (1.0 - 2.0 * ((sy + 0.5) / h))
                * (self.fov / 2.0 * std::f64::consts::PI / 180.0).tan();

            let direction = Vec3::new(px, py, 1.0).normalized().rotate(self.rotation);

            let blurriness = 0.0001;
            let spp = if blurriness == 0.0 { 1 } else { 1 };

            let mut rays: Vec<Ray> = Vec::with_capacity(spp);
            for _ in 0..spp {
                let sample_direction = direction.rotate(Vec3::new(
                    (rng.gen::<f64>() - 0.5) * blurriness,
                    (rng.gen::<f64>() - 0.5) * blurriness,
                    (rng.gen::<f64>() - 0.5) * blurriness,
                ));
                rays.push(Ray {
                    origin: self.position,
                    direction: sample_direction,
                    level: scene.config.recursion_depth,
                });
            }
            rays
        } else {
            let aspect_ratio = w / h;
            let px = (2.0 * ((sx + 0.5) / w) - 1.0)
                * aspect_ratio
                * (self.fov / 2.0 * std::f64::consts::PI / 180.0).tan();
            let py = (1.0 - 2.0 * ((sy + 0.5) / h))
                * (self.fov / 2.0 * std::f64::consts::PI / 180.0).tan();

            let direction = Vec3::new(px, py, 1.0).normalized().rotate(self.rotation);

            vec![Ray {
                origin: self.position,
                direction,
                level: scene.config.recursion_depth,
            }]
        }
    }
}
