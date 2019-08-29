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
use crate::shape::Ray;

pub trait Camera {
    fn project_ray(&self, screen_pos: (f64, f64), screen_dim: (f64, f64)) -> Ray;
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
    fn project_ray(&self, (sx, sy): (f64, f64), (w, h): (f64, f64)) -> Ray {
        let aspect_ratio = w / h;
        let px = (2.0 * ((sx + 0.5) / w) - 1.0)
            * aspect_ratio
            * (self.fov / 2.0 * std::f64::consts::PI / 180.0).tan();
        let py =
            (1.0 - 2.0 * ((sy + 0.5) / h)) * (self.fov / 2.0 * std::f64::consts::PI / 180.0).tan();

        let direction = Vec3::new(px, py, 1.0).normalized().rotate(self.rotation);

        Ray {
            origin: self.position,
            direction,
            level: 10,
        }
    }
}
