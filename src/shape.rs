/* ************************************************************************** */
/*                                                                            */
/*                                                        ::::::::            */
/*   shape.rs                                           :+:    :+:            */
/*                                                     +:+                    */
/*   By: nmartins <nmartins@student.codam.nl>         +#+                     */
/*                                                   +#+                      */
/*   Created: 2019/07/19 18:17:32 by nmartins       #+#    #+#                */
/*   Updated: 2019/07/19 18:58:29 by nmartins      ########   odam.nl         */
/*                                                                            */
/* ************************************************************************** */

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
	x: f64,
	y: f64,
	z: f64,
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
	pub do_intersect: bool,
}

pub trait Intersectable {
	/*
		** Whether or not object intersects with the ray
		*/
	fn do_intersect(&self, ray: &Ray) -> Option<Intersection>;
}

impl Ray {
	pub fn cast(&self, scene: Vec<Box<Intersectable>>) -> Option<Intersection> {
		let mut intersections: Vec<Intersection> = Vec::new();

		for shape in scene.iter() {
			if let Some(intersection) = shape.do_intersect(self) {
				intersections.push(intersection);
			}
		}

		intersections.first().and_then(|x| x.copy())
	}
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sphere {
	pub origin: Vec3,
	pub radius: f64,
}
