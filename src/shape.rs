/* ************************************************************************** */
/*                                                                            */
/*                                                        ::::::::            */
/*   shape.rs                                           :+:    :+:            */
/*                                                     +:+                    */
/*   By: nmartins <nmartins@student.codam.nl>         +#+                     */
/*                                                   +#+                      */
/*   Created: 2019/07/19 18:17:32 by nmartins       #+#    #+#                */
/*   Updated: 2019/07/20 20:20:47 by nmartins      ########   odam.nl         */
/*                                                                            */
/* ************************************************************************** */

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
	pub x: f64,
	pub y: f64,
	pub z: f64,
}

impl std::ops::Sub<Vec3> for Vec3 {
	type Output = Vec3;
	fn sub(self, rhs: Vec3) -> Vec3 {
		Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
	}
}

impl std::ops::Add<Vec3> for Vec3 {
	type Output = Vec3;
	fn add(self, rhs: Vec3) -> Vec3 {
		Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
	}
}

impl std::ops::Mul<f64> for Vec3 {
	type Output = Vec3;
	fn mul(self, rhs: f64) -> Vec3 {
		Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
	}
}

impl std::ops::Div<f64> for Vec3 {
	type Output = Vec3;
	fn div(self, rhs: f64) -> Vec3 {
		Vec3::new(self.x / rhs, self.y / rhs, self.z / rhs)
	}
}

impl Vec3 {
	pub const ORIGIN: Self = Vec3 {
		x: 0.0,
		y: 0.0,
		z: 0.0,
	};

	pub fn new(x: f64, y: f64, z: f64) -> Self {
		Vec3 { x, y, z }
	}
	pub fn length2(&self) -> f64 {
		self.x * self.x + self.y * self.y + self.z * self.z
	}

	pub fn length(&self) -> f64 {
		self.length2().sqrt()
	}

	pub fn dot(&self, rhs: &Vec3) -> f64 {
		self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
	}

	pub fn normalized(&self) -> Self {
		let mag = self.length();

		/* Make a copy and divide it by the magnitude*/
		*self / mag
	}
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ray {
	/*
		** Where the ray is cast from
		*/
	pub origin: Vec3,

	/*
		** (NORMALIZED)
		** In what direction the ray is to be cast
		*/
	pub direction: Vec3,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Intersection {
	pub t: f64,
	pub normal: Vec3,
	pub origin: Vec3,
}

use crate::material::Material;
pub trait Intersectable {
	fn mat(&self) -> Material;
	/* Whether or not object intersects with the ray */
	fn do_intersect(&self, ray: &Ray) -> Option<Intersection>;
}

impl Ray {
	pub fn cast(&self, scene: &Vec<Box<dyn Intersectable>>) -> Option<Vec3> {
		let mut intersections: Vec<(Intersection, &Box<dyn Intersectable>)> = Vec::new();
		let mut closest;

		for shape in scene.iter() {
			if let Some(intersection) = shape.do_intersect(self) {
				intersections.push((intersection, shape));
			}
		}

		if intersections.is_empty() {
			return None;
		}

		closest = intersections.first()?;
		for intersection in intersections.iter() {
			if closest.0.t > intersection.0.t {
				closest = intersection;
			}
		}
		
		Some({ 
			let orig_color = closest.1.mat().color;

			let light_ray = (Vec3::new(0.0, 100.0, 0.0) - closest.0.origin).normalized();
			let dot = closest.0.normal.dot(&light_ray);

			let diffuse_color = orig_color * dot;
			
			orig_color * 0.3 + diffuse_color * 0.7
		})
	}
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sphere {
	pub origin: Vec3,
	pub radius: f64,

	pub material: Material,
}

impl Intersectable for Sphere {
	fn mat(&self) -> Material {
		self.material
	}

	fn do_intersect(&self, ray: &Ray) -> Option<Intersection> {
		let local_ray = self.origin - ray.origin;
		let tca = local_ray.dot(&ray.direction);
		if tca < 0.0 {
			return None;
		}
		let d2 = local_ray.length2() - tca * tca;
		if d2 > self.radius * self.radius {
			return None;
		}
		let thc = (self.radius * self.radius - d2).sqrt();
		let t0 = tca - thc;
		let t1 = tca - thc;
		let t = t0.max(t1);
		let p = ray.origin + (ray.direction * t);
		let normal = (p - self.origin).normalized();
		Some(Intersection {
			t,
			origin: p,
			normal,
		})
	}
}
