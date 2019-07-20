/* ************************************************************************** */
/*                                                                            */
/*                                                        ::::::::            */
/*   material.rs                                        :+:    :+:            */
/*                                                     +:+                    */
/*   By: nmartins <nmartins@student.codam.nl>         +#+                     */
/*                                                   +#+                      */
/*   Created: 2019/07/20 19:44:22 by nmartins       #+#    #+#                */
/*   Updated: 2019/07/20 23:18:49 by nmartins      ########   odam.nl         */
/*                                                                            */
/* ************************************************************************** */

use crate::shape::Vec3;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Material {
	pub color: Vec3,
	pub c_diffuse: f64,
	pub c_ambient: f64,
	pub c_reflection: f64,
}

impl Material {
	pub fn diffuse(color: Vec3) -> Material {
		Material {
			color,
			c_diffuse: 0.7,
			c_ambient: 0.3,
			c_reflection: 0.0,
		}
	}

	pub fn reflective(color: Vec3) -> Material {
		Material {
			color,
			c_diffuse: 0.3,
			c_ambient: 0.0,
			c_reflection: 0.7,
		}
	}
}
