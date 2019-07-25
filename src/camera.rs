/* ************************************************************************** */
/*                                                                            */
/*                                                        ::::::::            */
/*   camera.rs                                          :+:    :+:            */
/*                                                     +:+                    */
/*   By: nmartins <nmartins@student.codam.nl>         +#+                     */
/*                                                   +#+                      */
/*   Created: 2019/07/19 18:23:53 by nmartins       #+#    #+#                */
/*   Updated: 2019/07/25 21:29:26 by nmartins      ########   odam.nl         */
/*                                                                            */
/* ************************************************************************** */

use crate::shape::{Ray, Vec3};

pub trait Camera {
	fn project_ray(&self, screen_pos: (f64, f64), screen_dim: (f64, f64)) -> Ray;
}

pub struct PerspectiveCamera {
	pub position: Vec3,
	pub rotation: Vec3,
	pub fov: f64,
	pub aspect_ratio: f64,
}

impl PerspectiveCamera {
	pub fn new(position: Vec3, aspect_ratio: f64) -> Self {
		PerspectiveCamera {
			position,
			rotation: Vec3::ORIGIN,
			fov: 60.0,
			aspect_ratio,
		}
	}

	pub fn translate(&mut self, v3: Vec3) {
		self.position.x += (self.rotation.y + std::f64::consts::FRAC_PI_2).sin() * v3.x
			+ self.rotation.y.sin() * v3.z;
		self.position.y += v3.y;
		self.position.z += -self.rotation.y.cos() * v3.z + self.rotation.y.sin() * v3.x;
	}
}

impl Camera for PerspectiveCamera {
	fn project_ray(&self, (sx, sy): (f64, f64), (w, h): (f64, f64)) -> Ray {
		let px = (2.0 * ((sx + 0.5) / w) - 1.0)
			* self.aspect_ratio
			* (self.fov / 2.0 * std::f64::consts::PI / 180.0).tan();
		let py =
			(1.0 - 2.0 * ((sy + 0.5) / h)) * (self.fov / 2.0 * std::f64::consts::PI / 180.0).tan();

		Ray {
			origin: self.position,
			direction: Vec3::new(px, py, 1.0).normalized(),
			level: 5,
		}
	}
}
